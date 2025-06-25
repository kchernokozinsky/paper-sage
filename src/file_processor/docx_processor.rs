use anyhow::{Context, Result};
use std::path::Path;

/// Read and extract text from DOCX files
pub fn read_docx_file(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("Failed to read DOCX file: {}", path.display()))?;

    let docx = docx_rs::read_docx(&bytes)
        .with_context(|| format!("Failed to parse DOCX file: {}", path.display()))?;

    let text = docx
        .document
        .children
        .iter()
        .map(|content| match content {
            docx_rs::DocumentChild::Paragraph(paragraph) => paragraph
                .children
                .iter()
                .map(|child| match child {
                    docx_rs::ParagraphChild::Run(run) => run
                        .children
                        .iter()
                        .map(|text_child| match text_child {
                            docx_rs::RunChild::Text(text) => text.text.clone(),
                            _ => String::new(),
                        })
                        .collect::<String>(),
                    _ => String::new(),
                })
                .collect::<String>(),
            _ => String::new(),
        })
        .collect::<String>();

    Ok(text)
}
