use anyhow::{Context, Result};
use std::path::Path;

/// Read and extract text from PDF files
pub fn read_pdf_file(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("Failed to read PDF file: {}", path.display()))?;

    let text = pdf_extract::extract_text_from_mem(&bytes)
        .with_context(|| format!("Failed to extract text from PDF: {}", path.display()))?;

    Ok(text)
}
