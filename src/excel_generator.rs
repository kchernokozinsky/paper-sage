use crate::models::GradingResult;
use anyhow::Result;
use tracing::info;
use std::fs::File;
use std::io::Write;

pub struct ExcelGenerator;

impl ExcelGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_report(&self, results: &[GradingResult], output_path: &str) -> Result<()> {
        info!("Creating report with {} results", results.len());
        
        // Create CSV file
        let csv_path = output_path.replace(".xlsx", ".csv");
        let mut file = File::create(&csv_path)?;
        
        // Write headers
        writeln!(file, "Filename,Correctness,Style,EdgeCases,Total,Comment")?;
        
        // Write data rows
        for result in results {
            // Escape quotes in comment field
            let escaped_comment = result.comment.replace("\"", "\"\"");
            writeln!(
                file,
                "\"{}\",{:.2},{:.2},{:.2},{:.2},\"{}\"",
                result.filename,
                result.correctness,
                result.style,
                result.edge_cases,
                result.total,
                escaped_comment
            )?;
        }
        
        info!("CSV report saved to: {}", csv_path);
        Ok(())
    }
} 