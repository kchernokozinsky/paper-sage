use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub task_description: String,
    pub evaluation_criteria: Vec<String>,
    pub teacher_comment: Option<String>,
    pub grading_strategy: GradingStrategy,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradingStrategy {
    pub correctness_weight: f32,
    pub style_weight: f32,
    pub edge_cases_weight: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradingResult {
    pub filename: String,
    pub correctness: f32,
    pub style: f32,
    pub edge_cases: f32,
    pub total: f32,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    pub filename: String,
    pub content: String,
    pub file_type: String,
}

/// Represents a complete student submission with multiple files
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentSubmission {
    pub student_name: String,
    pub files: Vec<FileContent>,
    pub merged_content: String,
}

impl StudentSubmission {
    /// Create a new student submission from a list of files
    pub fn new(student_name: String, files: Vec<FileContent>) -> Self {
        let merged_content = Self::merge_files(&files);
        Self {
            student_name,
            files,
            merged_content,
        }
    }

    /// Merge all files into a single content string with folder hierarchy preserved
    fn merge_files(files: &[FileContent]) -> String {
        let mut merged = String::new();
        
        for file in files {
            merged.push_str(&format!("=== FILE: {} ===\n", file.filename));
            merged.push_str(&format!("Type: {}\n", file.file_type));
            merged.push_str("Content:\n");
            merged.push_str(&file.content);
            merged.push_str("\n\n");
        }
        
        merged
    }

    /// Get the main filename for the submission (usually the student folder name)
    pub fn get_main_filename(&self) -> String {
        format!("{}/", self.student_name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradingRequest {
    pub filename: String,
    pub content: String,
    pub task_description: String,
    pub evaluation_criteria: Vec<String>,
    pub teacher_comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradingResponse {
    pub filename: String,
    pub correctness: f32,
    pub style: f32,
    pub edge_cases: f32,
    pub total: f32,
    pub comment: String,
}

// OpenAI API structures
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChoice {
    pub message: OpenAIMessage,
}

// Ollama API structures
#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}

impl Default for GradingStrategy {
    fn default() -> Self {
        Self {
            correctness_weight: 0.5,
            style_weight: 0.3,
            edge_cases_weight: 0.2,
        }
    }
}
