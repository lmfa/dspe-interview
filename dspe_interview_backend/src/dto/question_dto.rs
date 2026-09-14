use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestionDto {
    pub seniority_level_id: i64,
    pub topic_id: i64,
    pub question_text: String,
    pub suggested_answer: String,
    pub technology_ids: Vec<i64>,
}