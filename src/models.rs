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