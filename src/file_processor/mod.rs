mod docx_processor;
mod pdf_processor;
mod supported_formats;
mod text_processor;

use crate::models::{FileContent, StudentSubmission};
use anyhow::Result;
use std::collections::HashMap;
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

    /// Process directory and group files by student folder
    pub fn process_directory(&self, dir_path: &str) -> Result<Vec<StudentSubmission>> {
        let mut student_files: HashMap<String, Vec<FileContent>> = HashMap::new();
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
                            
                            // Extract student name from the path
                            let student_name = self.extract_student_name(path, &root);
                            student_files
                                .entry(student_name)
                                .or_insert_with(Vec::new)
                                .push(content);
                        }
                        Err(e) => {
                            warn!("Failed to read file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        // Convert grouped files to StudentSubmission objects
        let mut submissions = Vec::new();
        for (student_name, files) in student_files {
            if !files.is_empty() {
                let submission = StudentSubmission::new(student_name, files);
                info!("Created submission for student '{}' with {} files", 
                      submission.student_name, submission.files.len());
                submissions.push(submission);
            }
        }

        info!("Found {} student submissions", submissions.len());
        Ok(submissions)
    }

    /// Extract student name from file path
    fn extract_student_name(&self, file_path: &Path, root: &Path) -> String {
        if let Ok(rel_path) = file_path.strip_prefix(root) {
            // Get the first component of the relative path as student name
            if let Some(first_component) = rel_path.components().next() {
                return first_component.as_os_str().to_string_lossy().to_string();
            }
        }
        
        // Fallback: use the root directory name
        root.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string()
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
            // Text-based files (programming languages, configs, docs)
            "rs" | "py" | "java" | "cpp" | "c" | "cs" | "js" | "ts" | "php" | "rb" | "go" | "swift" | "kt" |
            "html" | "css" | "jsx" | "tsx" | "vue" | "svelte" |
            "txt" | "md" | "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" |
            "csv" | "sql" | "sh" | "bat" | "ps1" => text_processor::read_text_file(path)?,
            // Binary/document files
            "pdf" => pdf_processor::read_pdf_file(path)?,
            "docx" | "doc" | "rtf" => docx_processor::read_docx_file(path)?,
            _ => anyhow::bail!("Unsupported file format: {}", extension),
        };

        Ok(FileContent {
            filename,
            content,
            file_type: extension,
        })
    }
}
