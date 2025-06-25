use crate::models::FileContent;
use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;
use tracing::{info, warn};

pub struct FileProcessor;

impl FileProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_directory(&self, dir_path: &str) -> Result<Vec<FileContent>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                
                if self.is_supported_format(&ext) {
                    match self.read_file(path) {
                        Ok(content) => {
                            info!("Successfully read file: {}", path.display());
                            files.push(content);
                        }
                        Err(e) => {
                            warn!("Failed to read file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
        
        info!("Found {} supported files", files.len());
        Ok(files)
    }

    fn is_supported_format(&self, extension: &str) -> bool {
        matches!(extension, "rs" | "py" | "java" | "txt" | "pdf" | "docx")
    }

    fn read_file(&self, path: &Path) -> Result<FileContent> {
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let content = match extension.as_str() {
            "rs" | "py" | "java" | "txt" => self.read_text_file(path)?,
            "pdf" => self.read_pdf_file(path)?,
            "docx" => self.read_docx_file(path)?,
            _ => anyhow::bail!("Unsupported file format: {}", extension),
        };
        
        Ok(FileContent {
            filename,
            content,
            file_type: extension,
        })
    }

    fn read_text_file(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read text file: {}", path.display()))
    }

    fn read_pdf_file(&self, path: &Path) -> Result<String> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read PDF file: {}", path.display()))?;
        
        let text = pdf_extract::extract_text_from_mem(&bytes)
            .with_context(|| format!("Failed to extract text from PDF: {}", path.display()))?;
        
        Ok(text)
    }

    fn read_docx_file(&self, path: &Path) -> Result<String> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("Failed to read DOCX file: {}", path.display()))?;
        
        let docx = docx_rs::read_docx(&bytes)
            .with_context(|| format!("Failed to parse DOCX file: {}", path.display()))?;
        
        let text = docx.document.children.iter()
            .map(|content| {
                match content {
                    docx_rs::DocumentChild::Paragraph(paragraph) => {
                        paragraph.children.iter()
                            .map(|child| {
                                match child {
                                    docx_rs::ParagraphChild::Run(run) => {
                                        run.children.iter()
                                            .map(|text_child| {
                                                match text_child {
                                                    docx_rs::RunChild::Text(text) => text.text.clone(),
                                                    _ => String::new(),
                                                }
                                            })
                                            .collect::<String>()
                                    }
                                    _ => String::new(),
                                }
                            })
                            .collect::<String>()
                    }
                    _ => String::new(),
                }
            })
            .collect::<String>();
        
        Ok(text)
    }
} 