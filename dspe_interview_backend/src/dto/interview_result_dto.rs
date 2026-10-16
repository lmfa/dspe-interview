use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveInterviewResultDto {
    pub candidate_id: i64,
    pub technology_id: i64,
    pub seniority_level_id: i64,
    pub questions: Vec<InterviewQuestionDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterviewQuestionDto {
    pub question_id: i64,
    pub topic_id: i64,
    pub candidate_response: String,
    pub candidate_rating: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterviewResultResponseDto {
    pub id: i64,
    pub candidate_id: i64,
    pub technology_id: i64,
    pub seniority_level_id: i64,
    pub created_at: String,
}
