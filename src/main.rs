use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{Executor, FromRow, PgPool};
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
pub struct TodoNew {
    pub note: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub note: String,
}

pub async fn retrieve_todo(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("Retrieving todo: {:?}", id);
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn add_todo(
    State(pool): State<PgPool>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("Received data: {:?}", data);
    match sqlx::query_as::<_, Todo>("INSERT INTO todos (note) VALUES ($1) RETURNING id, note")
        .bind(&data.note)
        .fetch_one(&pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn hello_shuttle() -> &'static str {
    "Hello from Shuttle!"
}

#[shuttle_runtime::main]
async fn shuttle_main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../migrations/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/", get(hello_shuttle))
        .route("/todo", post(add_todo))
        .route("/todo/:id", get(retrieve_todo))
        .with_state(pool)
        .nest_service("/assets", ServeDir::new(PathBuf::from("assets")));

    Ok(router.into())
}
