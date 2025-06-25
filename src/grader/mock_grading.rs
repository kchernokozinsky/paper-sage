use crate::models::GradingResult;

/// Generate mock grading results for testing and fallback scenarios
pub fn generate_mock_result(filename: &str) -> GradingResult {
    let correctness = 0.8;
    let style = 0.7;
    let edge_cases = 0.6;
    let total = correctness * 0.5 + style * 0.3 + edge_cases * 0.2;

    GradingResult {
        filename: filename.to_string(),
        correctness,
        style,
        edge_cases,
        total,
        comment: "Mock grading response - AI API was unavailable. This is a sample response to demonstrate the system functionality.".to_string(),
    }
}
