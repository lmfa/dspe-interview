use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;

use crate::models::question_topic::QuestionTopic;

pub async fn get_topics(
    State(pool): State<SqlitePool>,
) -> Json<Vec<QuestionTopic>>
{
    let topics =
        sqlx::query_as::<_, QuestionTopic>(
            r#"
            SELECT id, name
            FROM question_topics
            ORDER BY name
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(topics)
}