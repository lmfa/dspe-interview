use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;

use crate::dto::question_dto::CreateQuestionDto;
use crate::models::question::Question;

pub async fn get_questions(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Question>>
{
    let questions =
        sqlx::query_as::<_, Question>(
            r#"
            SELECT
                id,
                seniority_level,
                topic,
                question_text,
                suggested_answer
            FROM questions
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(questions)
}

pub async fn create_question(
    State(pool): State<SqlitePool>,
    Json(dto): Json<CreateQuestionDto>,
)
{
    sqlx::query(
        r#"
        INSERT INTO questions
        (
            seniority_level,
            topic,
            question_text,
            suggested_answer
        )
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(dto.seniority_level)
    .bind(dto.topic)
    .bind(dto.question_text)
    .bind(dto.suggested_answer)
    .execute(&pool)
    .await
    .unwrap();
}