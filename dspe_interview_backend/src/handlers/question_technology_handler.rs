use axum::{
    extract::{State, Path},
    Json,
};

use sqlx::SqlitePool;

use crate::models::technology::Technology;

pub async fn get_question_technologies(
    State(pool): State<SqlitePool>,
    Path(question_id): Path<i64>,
) -> Json<Vec<Technology>>
{
    let technologies =
        sqlx::query_as::<_, Technology>(
            r#"
            SELECT t.id, t.name
            FROM technologies t
            JOIN question_technologies qt ON t.id = qt.technology_id
            WHERE qt.question_id = ?
            ORDER BY t.id
            "#
        )
        .bind(question_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(technologies)
}
