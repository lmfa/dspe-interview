use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;

use crate::dto::candidate_dto::CreateCandidateDto;
use crate::models::candidate::Candidate;

pub async fn get_candidates(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Candidate>>
{
    let candidates =
        sqlx::query_as::<_, Candidate>(
            "SELECT id, name FROM candidates"
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(candidates)
}

pub async fn create_candidate(
    State(pool): State<SqlitePool>,
    Json(dto): Json<CreateCandidateDto>,
)
{
    sqlx::query(
        "INSERT INTO candidates(name) VALUES (?)"
    )
    .bind(dto.name)
    .execute(&pool)
    .await
    .unwrap();
}