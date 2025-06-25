mod docx_processor;
mod pdf_processor;
mod supported_formats;
mod text_processor;

use crate::models::FileContent;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;

pub use supported_formats::SupportedFormats;

/// Main file processor that handles reading different file formats
pub struct FileProcessor;

impl Default for FileProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl FileProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process_directory(&self, dir_path: &str) -> Result<Vec<FileContent>> {
        let mut files = Vec::new();
        let root = PathBuf::from(dir_path);

        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();

                if SupportedFormats::is_supported(&ext) {
                    match self.read_file(path, &root) {
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

    fn read_file(&self, path: &Path, root: &Path) -> Result<FileContent> {
        let rel_path = path.strip_prefix(root).unwrap_or(path);
        let filename = rel_path.to_string_lossy().replace("\\", "/");
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let content = match extension.as_str() {
            "rs" | "py" | "java" | "txt" => text_processor::read_text_file(path)?,
            "pdf" => pdf_processor::read_pdf_file(path)?,
            "docx" => docx_processor::read_docx_file(path)?,
            _ => anyhow::bail!("Unsupported file format: {}", extension),
        };

        Ok(FileContent {
            filename,
            content,
            file_type: extension,
        })
    }
}
