use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct InterviewResult {
    pub id: i64,
    pub candidate_id: i64,
    pub technology_id: i64,
    pub seniority_level_id: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct InterviewQuestionResult {
    pub id: i64,
    pub interview_result_id: i64,
    pub question_id: i64,
    pub topic_id: i64,
    pub candidate_response: String,
    pub candidate_rating: i32,
}
