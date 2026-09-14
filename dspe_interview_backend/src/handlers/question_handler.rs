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
                seniority_level_id,
                topic_id,
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
    let mut tx = pool.begin().await.unwrap();

    let result = sqlx::query(
        r#"
        INSERT INTO questions
        (
            seniority_level_id,
            topic_id,
            question_text,
            suggested_answer
        )
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(dto.seniority_level_id)
    .bind(dto.topic_id)
    .bind(dto.question_text)
    .bind(dto.suggested_answer)
    .execute(&mut *tx)
    .await
    .unwrap();

    let question_id = result.last_insert_rowid();

    for technology_id in dto.technology_ids
    {
        sqlx::query(
            r#"
            INSERT INTO question_technologies
            (
                question_id,
                technology_id
            )
            VALUES (?, ?)
            "#
        )
        .bind(question_id)
        .bind(technology_id)
        .execute(&mut *tx)
        .await
        .unwrap();
    }

    tx.commit().await.unwrap();
}
