use crate::handlers::{
    candidate_handler, question_handler, question_technology_handler, question_topic_handler, seniority_level_handler, technology_handler
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
        .route(
            "/questions/{id}/technologies",
            get(question_technology_handler::get_question_technologies),
        )
        .route(
            "/question-topics",
            get(question_topic_handler::get_topics)
        )
        .route(
            "/seniority-levels",
            get(seniority_level_handler::get_levels)
        )
        .route(
            "/technologies",
            get(technology_handler::get_technologies)
        )
        .with_state(pool)
}