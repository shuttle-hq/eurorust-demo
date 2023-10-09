use crate::error::AppError;
use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn get_greetings(State(pool): State<PgPool>) -> Result<Json<i64>, AppError> {
    tracing::info!("Retrieving total greetings");
    match sqlx::query_scalar::<_, i64>("SELECT last_value FROM serial")
        .fetch_one(&pool)
        .await
    {
        Ok(greetings) => Ok(Json(greetings)),
        Err(error) => {
            tracing::error!(error = %error, "failed to retrieve greetings total");
            Err(AppError::Internal)
        }
    }
}

pub async fn increment_greetings(State(pool): State<PgPool>) -> Result<Json<i64>, AppError> {
    tracing::info!("Incrementing greetings");
    match sqlx::query_scalar::<_, i64>("SELECT nextval('serial')")
        .fetch_one(&pool)
        .await
    {
        Ok(greetings) => Ok(Json(greetings)),
        Err(error) => {
            tracing::error!(error = %error, "failed to increment greeting total");
            Err(AppError::Internal)
        }
    }
}
