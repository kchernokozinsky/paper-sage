use crate::models::{GradingResponse, GradingResult, GradingStrategy};
use anyhow::{Context, Result};
use serde_json;

/// Parse AI model responses into grading results
pub fn parse_grading_response(
    response: &str,
    _filename: &str,
    strategy: &GradingStrategy,
) -> Result<GradingResult> {
    // Try to extract JSON from the response
    let json_start = response.find('{');
    let json_end = response.rfind('}');

    let json_text = if let (Some(start), Some(end)) = (json_start, json_end) {
        &response[start..=end]
    } else {
        anyhow::bail!("No valid JSON found in response");
    };

    let grading_response: GradingResponse = serde_json::from_str(json_text)
        .with_context(|| format!("Failed to parse grading response: {}", json_text))?;

    // Calculate total score using the grading strategy
    let total = grading_response.correctness * strategy.correctness_weight
        + grading_response.style * strategy.style_weight
        + grading_response.edge_cases * strategy.edge_cases_weight;

    Ok(GradingResult {
        filename: grading_response.filename,
        correctness: grading_response.correctness,
        style: grading_response.style,
        edge_cases: grading_response.edge_cases,
        total,
        comment: grading_response.comment,
    })
}
