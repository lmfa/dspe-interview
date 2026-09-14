use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;

use crate::models::technology::Technology;

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