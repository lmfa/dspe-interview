use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuestionTopic {
    pub id: i64,
    pub name: String,
}