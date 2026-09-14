use crate::handlers::{
    candidate_handler,
    question_handler,
};

use axum::{
    routing::get,
    Router,
};
use sqlx::SqlitePool;

pub fn create_routes(pool: SqlitePool) -> Router
{
    Router::new()
        .route(
            "/candidates",
            get(candidate_handler::get_candidates)
                .post(candidate_handler::create_candidate),
        )
        .route(
            "/questions",
            get(question_handler::get_questions)
                .post(question_handler::create_question),
        )
        .with_state(pool)
}