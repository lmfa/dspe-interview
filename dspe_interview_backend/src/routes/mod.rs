use axum::{
    routing::{get, post},
    Router,
};

use sqlx::SqlitePool;

use crate::handlers::candidate_handler;

pub fn create_routes(
    pool: SqlitePool,
) -> Router
{
    Router::new()
        .route(
            "/candidates",
            get(candidate_handler::get_candidates)
            .post(candidate_handler::create_candidate),
        )
        .with_state(pool)
}