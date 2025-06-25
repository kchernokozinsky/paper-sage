use crate::config::AppConfig;
use crate::grader::mock_grading;
use crate::models::{
    GradingRequest, GradingResult, GradingStrategy, OllamaRequest, OllamaResponse, OpenAIMessage,
    OpenAIRequest, OpenAIResponse,
};
use anyhow::{Context, Result};
use reqwest::Client;

/// AI client for communicating with different model endpoints
pub struct AIClient {
    client: Client,
    model_endpoint: String,
    is_openai: bool,
    app_config: Option<AppConfig>,
}

impl AIClient {
    pub fn new(model_endpoint: &str, app_config: Option<AppConfig>) -> Result<Self> {
        let client = Client::new();
        let is_openai = model_endpoint.contains("openai.com");

        Ok(Self {
            client,
            model_endpoint: model_endpoint.to_string(),
            is_openai,
            app_config,
        })
    }

    pub async fn grade_submission(
        &self,
        request: &GradingRequest,
        strategy: &GradingStrategy,
    ) -> Result<GradingResult> {
        let response = if self.is_openai {
            self.call_openai_api(request, strategy).await?
        } else {
            self.call_ollama_api(request, strategy).await?
        };

        Ok(response)
    }

    async fn call_openai_api(
        &self,
        request: &GradingRequest,
        strategy: &GradingStrategy,
    ) -> Result<GradingResult> {
        let prompt =
            crate::grader::prompt_builder::build_grading_prompt(request, self.app_config.as_ref());

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

        let response = self
            .client
            .post(&self.model_endpoint)
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    std::env::var("OPENAI_API_KEY").unwrap_or_default()
                ),
            )
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
        let content = openai_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from OpenAI"))?
            .message
            .content
            .clone();

        crate::grader::response_parser::parse_grading_response(
            &content,
            &request.filename,
            strategy,
        )
    }

    async fn call_ollama_api(
        &self,
        request: &GradingRequest,
        strategy: &GradingStrategy,
    ) -> Result<GradingResult> {
        let prompt =
            crate::grader::prompt_builder::build_grading_prompt(request, self.app_config.as_ref());

        let ollama_request = OllamaRequest {
            model: "qwen2.5:0.5b".to_string(),
            prompt,
            stream: false,
        };

        println!(
            "Sending request to Ollama: {}",
            &format!("{}/api/generate", self.model_endpoint.trim_end_matches('/'))
        );

        let response = self
            .client
            .post(format!(
                "{}/api/generate",
                self.model_endpoint.trim_end_matches('/')
            ))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(300)) // 300 second timeout (5 minutes)
            .json(&ollama_request)
            .send()
            .await;

        match response {
            Ok(response) => {
                println!("Ollama response status: {}", response.status());
                if !response.status().is_success() {
                    let error_text = response.text().await?;
                    anyhow::bail!("Ollama API error: {}", error_text);
                }

                let ollama_response: OllamaResponse = response.json().await?;
                println!("Ollama response received successfully");
                crate::grader::response_parser::parse_grading_response(
                    &ollama_response.response,
                    &request.filename,
                    strategy,
                )
            }
            Err(e) => {
                // Fallback to mock response when Ollama is not available
                println!(
                    "Ollama API unavailable (error: {:?}), using mock response for {}",
                    e, request.filename
                );
                Ok(mock_grading::generate_mock_result(&request.filename))
            }
        }
    }
}
