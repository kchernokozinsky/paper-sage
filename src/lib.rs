//! Paper Sage - AI-powered student submission grader
//!
//! This library provides functionality for automatically grading student programming submissions
//! using AI models. It supports multiple file formats and can work with both OpenAI and local LLMs.

pub mod config;
pub mod excel_generator;
pub mod file_processor;
pub mod grader;
pub mod models;

// Re-export main types for easier access
pub use excel_generator::ExcelGenerator;
pub use file_processor::FileProcessor;
pub use grader::Grader;
pub use models::Config;
pub use models::{FileContent, GradingRequest, GradingResponse, GradingResult, StudentSubmission};

#[allow(dead_code)]
/// Main application struct that orchestrates the grading process
pub struct PaperSage {
    config: Config,
    app_config: config::AppConfig,
    file_processor: FileProcessor,
    grader: Grader,
    excel_generator: ExcelGenerator,
}

impl PaperSage {
    /// Create a new PaperSage instance
    pub fn new(config_path: &str, model_endpoint: Option<String>) -> anyhow::Result<Self> {
        let config = Config::from_file(config_path)?;
        let app_config = config::AppConfig::from_toml("config.toml")?;
        let file_processor = FileProcessor::new();

        let model_endpoint = model_endpoint
            .unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".to_string());
        let grader = Grader::new(&model_endpoint, &config, Some(app_config.clone()))?;

        let excel_generator = ExcelGenerator::new();

        Ok(Self {
            config,
            app_config,
            file_processor,
            grader,
            excel_generator,
        })
    }

    /// Grade all submissions in the input directory
    pub async fn grade_submissions(&self, input_dir: &str) -> anyhow::Result<Vec<GradingResult>> {
        tracing::info!("Starting to grade submissions in: {}", input_dir);

        let results = self
            .grader
            .grade_submissions(input_dir, &self.file_processor)
            .await?;

        tracing::info!("Completed grading {} submissions", results.len());
        Ok(results)
    }

    /// Resume grading from a previous session
    pub async fn resume_grading(
        &self,
        input_dir: &str,
        resume_path: &str,
    ) -> anyhow::Result<Vec<GradingResult>> {
        tracing::info!("Resuming grading from: {}", resume_path);

        let results = self
            .grader
            .resume_grading(input_dir, &self.file_processor, resume_path)
            .await?;

        tracing::info!(
            "Completed resuming grading for {} submissions",
            results.len()
        );
        Ok(results)
    }

    /// Generate reports from grading results
    pub fn generate_reports(
        &self,
        results: &[GradingResult],
        output_path: &str,
    ) -> anyhow::Result<()> {
        tracing::info!("Generating reports for {} results", results.len());

        // Generate CSV report
        self.excel_generator.generate_report(results, output_path)?;

        // Save JSON results for potential resume
        let json_path = "results.json";
        std::fs::write(json_path, serde_json::to_string_pretty(results)?)?;
        tracing::info!("JSON results saved: {}", json_path);

        Ok(())
    }

    /// Get the loaded configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Process files without grading (useful for testing)
    pub fn process_files(&self, input_dir: &str) -> anyhow::Result<Vec<StudentSubmission>> {
        self.file_processor.process_directory(input_dir)
    }
}

/// Error types specific to Paper Sage
#[derive(Debug, thiserror::Error)]
pub enum PaperSageError {
    #[error("Configuration error: {0}")]
    Config(#[from] anyhow::Error),

    #[error("File processing error: {0}")]
    FileProcessing(#[from] std::io::Error),

    #[error("Grading error: {0}")]
    Grading(String),

    #[error("Report generation error: {0}")]
    ReportGeneration(String),
}

/// Result type for Paper Sage operations
pub type PaperSageResult<T> = Result<T, PaperSageError>;
