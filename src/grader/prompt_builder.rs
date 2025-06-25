use crate::config::AppConfig;
use crate::models::GradingRequest;

/// Build grading prompts for AI models
pub fn build_grading_prompt(request: &GradingRequest, app_config: Option<&AppConfig>) -> String {
    let criteria_text = request
        .evaluation_criteria
        .iter()
        .enumerate()
        .map(|(i, criteria)| format!("{}. {}", i + 1, criteria))
        .collect::<Vec<_>>()
        .join("\n");

    let teacher_comment = request
        .teacher_comment
        .as_ref()
        .map(|c| format!("\nTeacher Comment: {}", c))
        .unwrap_or_default();

    let default_template = "Please grade the following student submission according to the task description and evaluation criteria.\n\nTask Description:\n{task_description}\n\nEvaluation Criteria:\n{criteria}{teacher_comment}\n\nStudent Submission (File: {filename}):\n```\n{content}\n```\n\nPlease provide your evaluation in the following JSON format:\n{{\n    \"filename\": \"{filename}\",\n    \"correctness\": <score 0-100>,\n    \"style\": <score 0-100>,\n    \"edge_cases\": <score 0-100>,\n    \"total\": <weighted average score 0-100>,\n    \"comment\": \"<detailed feedback>\"\n}}\n\nNote: The total score will be calculated automatically using the grading weights.";

    let template = app_config
        .and_then(|cfg| cfg.prompt.as_ref()?.template.as_ref())
        .map(|s| s.as_str())
        .unwrap_or(default_template);

    template
        .replace("{task_description}", &request.task_description)
        .replace("{criteria}", &criteria_text)
        .replace("{teacher_comment}", &teacher_comment)
        .replace("{filename}", &request.filename)
        .replace("{content}", &request.content)
}
