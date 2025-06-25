use clap::Parser;
use paper_sage::PaperSage;
use tracing::{info, Level};

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
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();

    info!("Starting Paper Sage - Student Submission Grader");
    info!("Input folder: {}", args.input);
    info!("Config file: {}", args.config);

    // Initialize Paper Sage application
    let paper_sage = PaperSage::new(&args.config, args.model_endpoint)?;

    info!(
        "Loaded configuration: {}",
        paper_sage.config().task_description
    );

    // Process files and generate grades
    let results = if let Some(resume_path) = args.resume {
        info!("Resuming from: {}", resume_path);
        paper_sage.resume_grading(&args.input, &resume_path).await?
    } else {
        info!("Starting fresh grading session");
        paper_sage.grade_submissions(&args.input).await?
    };

    // Generate reports
    info!("Generating reports...");
    paper_sage.generate_reports(&results, "results.xlsx")?;

    info!("Grading completed successfully!");
    Ok(())
}
