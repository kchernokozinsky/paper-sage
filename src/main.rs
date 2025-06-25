use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber;

mod config;
mod file_processor;
mod grader;
mod models;
mod excel_generator;

use crate::models::Config;
use crate::file_processor::FileProcessor;
use crate::grader::Grader;
use crate::excel_generator::ExcelGenerator;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to folder containing student submissions
    #[arg(short, long)]
    input: String,

    /// Path to JSON config file with task description and grading criteria
    #[arg(short, long)]
    config: String,

    /// Base URL of the AI model endpoint (defaults to OpenAI)
    #[arg(short, long)]
    model_endpoint: Option<String>,

    /// Resume from partial results file
    #[arg(short, long)]
    resume: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let args = Args::parse();

    info!("Starting Paper Sage - Student Submission Grader");
    info!("Input folder: {}", args.input);
    info!("Config file: {}", args.config);

    // Load configuration
    let config = Config::from_file(&args.config)?;
    info!("Loaded configuration: {}", config.task_description);

    // Initialize file processor
    let file_processor = FileProcessor::new();

    // Initialize grader with model endpoint
    let model_endpoint = args.model_endpoint
        .unwrap_or_else(|| "https://api.openai.com/v1/chat/completions".to_string());
    let grader = Grader::new(&model_endpoint, &config)?;

    // Initialize Excel generator
    let excel_generator = ExcelGenerator::new();

    // Process files and generate grades
    let results = if let Some(resume_path) = args.resume {
        info!("Resuming from: {}", resume_path);
        grader.resume_grading(&args.input, &file_processor, &resume_path).await?
    } else {
        info!("Starting fresh grading session");
        grader.grade_submissions(&args.input, &file_processor).await?
    };

    // Generate Excel report
    info!("Generating Excel report...");
    excel_generator.generate_report(&results, "results.xlsx")?;
    info!("Excel report generated: results.xlsx");

    // Save JSON results for potential resume
    let json_path = "results.json";
    std::fs::write(json_path, serde_json::to_string_pretty(&results)?)?;
    info!("JSON results saved: {}", json_path);

    info!("Grading completed successfully!");
    Ok(())
}
