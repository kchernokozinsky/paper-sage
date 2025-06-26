use crate::models::Config;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct AiConfig {
    pub timeout_secs: Option<u64>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PromptConfig {
    pub template: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub ai: Option<AiConfig>,
    pub prompt: Option<PromptConfig>,
}

impl AppConfig {
    pub fn from_toml(path: &str) -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .with_context(|| format!("Failed to read TOML config: {}", path))?;
        let app_config: AppConfig = settings
            .try_deserialize()
            .with_context(|| format!("Failed to parse TOML config: {}", path))?;
        Ok(app_config)
    }
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;

        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))?;

        // Validate config
        config.validate()?;

        Ok(config)
    }

    fn validate(&self) -> Result<()> {
        if self.task_description.trim().is_empty() {
            anyhow::bail!("Task description cannot be empty");
        }

        if self.evaluation_criteria.is_empty() {
            anyhow::bail!("Evaluation criteria cannot be empty");
        }

        let strategy = &self.grading_strategy;
        let total_weight =
            strategy.correctness_weight + strategy.style_weight + strategy.edge_cases_weight;

        if (total_weight - 1.0).abs() > 0.01 {
            anyhow::bail!("Grading weights must sum to 1.0, got: {}", total_weight);
        }

        Ok(())
    }
}
