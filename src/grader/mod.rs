pub mod ai_client;
pub mod grading_engine;
pub mod mock_grading;
pub mod prompt_builder;
pub mod response_parser;

use crate::config::AppConfig;
use crate::file_processor::FileProcessor;
use crate::models::{Config, GradingResult};
use anyhow::Result;
use std::collections::HashMap;
use tracing::{error, info};

pub use ai_client::AIClient;
pub use grading_engine::GradingEngine;

/// Main grader that orchestrates the grading process
pub struct Grader {
    ai_client: AIClient,
    grading_engine: GradingEngine,
}

impl Grader {
    pub fn new(
        model_endpoint: &str,
        config: &Config,
        app_config: Option<AppConfig>,
    ) -> Result<Self> {
        let ai_client = AIClient::new(model_endpoint, app_config)?;
        let grading_engine = GradingEngine::new(config);

        Ok(Self {
            ai_client,
            grading_engine,
        })
    }

    pub async fn grade_submissions(
        &self,
        input_dir: &str,
        file_processor: &FileProcessor,
    ) -> Result<Vec<GradingResult>> {
        let files = file_processor.process_directory(input_dir)?;
        let mut results = Vec::new();

        info!("Starting to grade {} files", files.len());

        for (i, file) in files.iter().enumerate() {
            info!("Grading file {}/{}: {}", i + 1, files.len(), file.filename);

            match self.grading_engine.grade_file(&self.ai_client, file).await {
                Ok(result) => {
                    info!(
                        "Successfully graded: {} (Score: {:.2})",
                        file.filename, result.total
                    );
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

    pub async fn resume_grading(
        &self,
        input_dir: &str,
        file_processor: &FileProcessor,
        resume_path: &str,
    ) -> Result<Vec<GradingResult>> {
        let existing_results: Vec<GradingResult> =
            serde_json::from_str(&std::fs::read_to_string(resume_path)?)?;

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
                match self.grading_engine.grade_file(&self.ai_client, &file).await {
                    Ok(result) => {
                        info!(
                            "Successfully graded: {} (Score: {:.2})",
                            file.filename, result.total
                        );
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
}
