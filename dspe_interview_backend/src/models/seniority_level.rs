use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SeniorityLevel {
    pub id: i64,
    pub name: String,
}