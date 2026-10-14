use axum::{
    extract::{Path, State},
    Json,
};

use sqlx::SqlitePool;

use crate::models::{
    question_topic::QuestionTopic,
    technology::Technology,
};

pub async fn get_technologies(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Technology>>
{
    let technologies =
        sqlx::query_as::<_, Technology>(
            r#"
            SELECT id, name
            FROM technologies
            ORDER BY id
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(technologies)
}

pub async fn get_technology_topics(
    State(pool): State<SqlitePool>,
    Path(technology_id): Path<i64>,
) -> Json<Vec<QuestionTopic>> {
    let topics = sqlx::query_as::<_, QuestionTopic>(
        r#"
        SELECT qt.id, qt.name
        FROM question_topics AS qt
        INNER JOIN topic_technologies AS tt
            ON tt.topic_id = qt.id
        WHERE tt.technology_id = ?
        ORDER BY qt.id
        "#,
    )
    .bind(technology_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(topics)
}