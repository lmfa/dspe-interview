use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub id: i64,

    pub topic_id: i64,

    pub seniority_level_id: i64,

    pub question_text: String,

    pub suggested_answer: String,
}
