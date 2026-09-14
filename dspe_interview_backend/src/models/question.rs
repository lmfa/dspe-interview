use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Question {
    pub id: i64,

    pub seniority_level: String,

    pub topic: String,

    pub question_text: String,

    pub suggested_answer: String,
}
