use crate::grader::AIClient;
use crate::models::{Config, StudentSubmission, GradingRequest, GradingResult};
use anyhow::Result;

/// Engine that handles the grading logic
pub struct GradingEngine {
    config: Config,
}

impl GradingEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub async fn grade_submission(
        &self,
        ai_client: &AIClient,
        submission: &StudentSubmission,
    ) -> Result<GradingResult> {
        let request = GradingRequest {
            filename: submission.get_main_filename(),
            content: submission.merged_content.clone(),
            task_description: self.config.task_description.clone(),
            evaluation_criteria: self.config.evaluation_criteria.clone(),
            teacher_comment: self.config.teacher_comment.clone(),
        };

        ai_client
            .grade_submission(&request, &self.config.grading_strategy)
            .await
    }
}
