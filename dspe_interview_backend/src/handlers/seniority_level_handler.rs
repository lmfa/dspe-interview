use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;

use crate::models::seniority_level::SeniorityLevel;

pub async fn get_levels(
    State(pool): State<SqlitePool>,
) -> Json<Vec<SeniorityLevel>>
{
    let levels =
        sqlx::query_as::<_, SeniorityLevel>(
            r#"
            SELECT id, name
            FROM seniority_levels
            ORDER BY id
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(levels)
}