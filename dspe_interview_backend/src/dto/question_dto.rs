use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestionDto {
    pub seniority_level: String,
    pub topic: String,
    pub question_text: String,
    pub suggested_answer: String,
}