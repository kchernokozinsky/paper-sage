use crate::models::Config;
use anyhow::{Context, Result};
use std::fs;

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
        let total_weight = strategy.correctness_weight + strategy.style_weight + strategy.edge_cases_weight;
        
        if (total_weight - 1.0).abs() > 0.01 {
            anyhow::bail!("Grading weights must sum to 1.0, got: {}", total_weight);
        }

        Ok(())
    }
} 