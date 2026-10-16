use axum::{
    extract::State,
    Json,
};

use sqlx::SqlitePool;

use crate::dto::interview_result_dto::{SaveInterviewResultDto, InterviewResultResponseDto};

pub async fn save_interview_result(
    State(pool): State<SqlitePool>,
    Json(dto): Json<SaveInterviewResultDto>,
) -> Json<InterviewResultResponseDto>
{
    let mut tx = pool.begin().await.unwrap();

    // Insert the main interview result
    let result = sqlx::query(
        r#"
        INSERT INTO interview_results
        (
            candidate_id,
            technology_id,
            seniority_level_id,
            created_at
        )
        VALUES (?, ?, ?, CURRENT_TIMESTAMP)
        "#
    )
    .bind(dto.candidate_id)
    .bind(dto.technology_id)
    .bind(dto.seniority_level_id)
    .execute(&mut *tx)
    .await
    .unwrap();

    let interview_result_id = result.last_insert_rowid();

    // Insert each question's response and rating
    for question in dto.questions
    {
        sqlx::query(
            r#"
            INSERT INTO interview_question_results
            (
                interview_result_id,
                question_id,
                topic_id,
                candidate_response,
                candidate_rating
            )
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(interview_result_id)
        .bind(question.question_id)
        .bind(question.topic_id)
        .bind(question.candidate_response)
        .bind(question.candidate_rating)
        .execute(&mut *tx)
        .await
        .unwrap();
    }

    tx.commit().await.unwrap();

    // Fetch and return the created interview result
    let interview_result = sqlx::query_as::<_, (i64, i64, i64, i64, String)>(
        r#"
        SELECT id, candidate_id, technology_id, seniority_level_id, created_at
        FROM interview_results
        WHERE id = ?
        "#
    )
    .bind(interview_result_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(InterviewResultResponseDto {
        id: interview_result.0,
        candidate_id: interview_result.1,
        technology_id: interview_result.2,
        seniority_level_id: interview_result.3,
        created_at: interview_result.4,
    })
}
