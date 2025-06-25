use crate::models::{
    Config, FileContent, GradingRequest, GradingResponse, GradingResult,
    OpenAIRequest, OpenAIResponse, OpenAIMessage,
    OllamaRequest, OllamaResponse
};
use crate::file_processor::FileProcessor;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json;
use std::collections::HashMap;
use tracing::{info, error};

pub struct Grader {
    client: Client,
    model_endpoint: String,
    config: Config,
    is_openai: bool,
}

impl Grader {
    pub fn new(model_endpoint: &str, config: &Config) -> Result<Self> {
        let client = Client::new();
        let is_openai = model_endpoint.contains("openai.com");
        
        Ok(Self {
            client,
            model_endpoint: model_endpoint.to_string(),
            config: config.clone(),
            is_openai,
        })
    }

    pub async fn grade_submissions(&self, input_dir: &str, file_processor: &FileProcessor) -> Result<Vec<GradingResult>> {
        let files = file_processor.process_directory(input_dir)?;
        let mut results = Vec::new();
        
        info!("Starting to grade {} files", files.len());
        
        for (i, file) in files.iter().enumerate() {
            info!("Grading file {}/{}: {}", i + 1, files.len(), file.filename);
            
            match self.grade_file(file).await {
                Ok(result) => {
                    info!("Successfully graded: {} (Score: {:.2})", file.filename, result.total);
                    results.push(result);
                }
                Err(e) => {
                    error!("Failed to grade {}: {}", file.filename, e);
                    // Add a failed result
                    results.push(GradingResult {
                        filename: file.filename.clone(),
                        correctness: 0.0,
                        style: 0.0,
                        edge_cases: 0.0,
                        total: 0.0,
                        comment: format!("Error during grading: {}", e),
                    });
                }
            }
        }
        
        Ok(results)
    }

    pub async fn resume_grading(&self, input_dir: &str, file_processor: &FileProcessor, resume_path: &str) -> Result<Vec<GradingResult>> {
        let existing_results: Vec<GradingResult> = serde_json::from_str(
            &std::fs::read_to_string(resume_path)?
        )?;
        
        let mut completed_files: HashMap<String, GradingResult> = existing_results
            .into_iter()
            .map(|r| (r.filename.clone(), r))
            .collect();
        
        let files = file_processor.process_directory(input_dir)?;
        let mut results = Vec::new();
        
        for file in files {
            if let Some(existing) = completed_files.remove(&file.filename) {
                results.push(existing);
                info!("Using existing result for: {}", file.filename);
            } else {
                info!("Grading file: {}", file.filename);
                match self.grade_file(&file).await {
                    Ok(result) => {
                        info!("Successfully graded: {} (Score: {:.2})", file.filename, result.total);
                        results.push(result);
                    }
                    Err(e) => {
                        error!("Failed to grade {}: {}", file.filename, e);
                        results.push(GradingResult {
                            filename: file.filename.clone(),
                            correctness: 0.0,
                            style: 0.0,
                            edge_cases: 0.0,
                            total: 0.0,
                            comment: format!("Error during grading: {}", e),
                        });
                    }
                }
            }
        }
        
        Ok(results)
    }

    async fn grade_file(&self, file: &FileContent) -> Result<GradingResult> {
        let request = GradingRequest {
            filename: file.filename.clone(),
            content: file.content.clone(),
            task_description: self.config.task_description.clone(),
            evaluation_criteria: self.config.evaluation_criteria.clone(),
            teacher_comment: self.config.teacher_comment.clone(),
        };

        let response = if self.is_openai {
            self.call_openai_api(&request).await?
        } else {
            self.call_ollama_api(&request).await?
        };

        Ok(response)
    }

    async fn call_openai_api(&self, request: &GradingRequest) -> Result<GradingResult> {
        let prompt = self.build_grading_prompt(request);
        
        let openai_request = OpenAIRequest {
            model: "gpt-4".to_string(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: "You are an expert programming instructor. Grade the student submission according to the provided criteria and return a JSON response with the specified fields.".to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            temperature: 0.3,
            max_tokens: 1000,
        };

        let response = self.client
            .post(&self.model_endpoint)
            .header("Authorization", format!("Bearer {}", std::env::var("OPENAI_API_KEY").unwrap_or_default()))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await
            .with_context(|| "Failed to send request to OpenAI API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("OpenAI API error: {}", error_text);
        }

        let openai_response: OpenAIResponse = response.json().await?;
        let content = openai_response.choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?
            .message.content.clone();

        self.parse_grading_response(&content, &request.filename)
    }

    async fn call_ollama_api(&self, request: &GradingRequest) -> Result<GradingResult> {
        let prompt = self.build_grading_prompt(request);
        
        let ollama_request = OllamaRequest {
            model: "llama2".to_string(),
            prompt,
            stream: false,
        };

        let response = self.client
            .post(&self.model_endpoint)
            .header("Content-Type", "application/json")
            .json(&ollama_request)
            .send()
            .await
            .with_context(|| "Failed to send request to Ollama API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Ollama API error: {}", error_text);
        }

        let ollama_response: OllamaResponse = response.json().await?;
        self.parse_grading_response(&ollama_response.response, &request.filename)
    }

    fn build_grading_prompt(&self, request: &GradingRequest) -> String {
        let criteria_text = request.evaluation_criteria
            .iter()
            .enumerate()
            .map(|(i, criteria)| format!("{}. {}", i + 1, criteria))
            .collect::<Vec<_>>()
            .join("\n");

        let teacher_comment = request.teacher_comment
            .as_ref()
            .map(|c| format!("\nTeacher Comment: {}", c))
            .unwrap_or_default();

        format!(
            "Please grade the following student submission according to the task description and evaluation criteria.

Task Description:
{}

Evaluation Criteria:
{}{}

Student Submission (File: {}):
```
{}
```

Please provide your evaluation in the following JSON format:
{{
    \"filename\": \"{}\",
    \"correctness\": <score 0-100>,
    \"style\": <score 0-100>,
    \"edge_cases\": <score 0-100>,
    \"total\": <weighted average score 0-100>,
    \"comment\": \"<detailed feedback>\"
}}

Ensure the total score is calculated as: correctness * 0.5 + style * 0.3 + edge_cases * 0.2",
            request.task_description,
            criteria_text,
            teacher_comment,
            request.filename,
            request.content,
            request.filename
        )
    }

    fn parse_grading_response(&self, response: &str, _filename: &str) -> Result<GradingResult> {
        // Try to extract JSON from the response
        let json_start = response.find('{');
        let json_end = response.rfind('}');
        
        let json_text = if let (Some(start), Some(end)) = (json_start, json_end) {
            &response[start..=end]
        } else {
            anyhow::bail!("No valid JSON found in response");
        };

        let grading_response: GradingResponse = serde_json::from_str(json_text)
            .with_context(|| format!("Failed to parse grading response: {}", json_text))?;

        Ok(GradingResult {
            filename: grading_response.filename,
            correctness: grading_response.correctness,
            style: grading_response.style,
            edge_cases: grading_response.edge_cases,
            total: grading_response.total,
            comment: grading_response.comment,
        })
    }
} 