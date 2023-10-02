#![allow(unused_imports)]

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{FromRow, PgPool};
use std::path::PathBuf;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
pub struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub note: String,
}

pub async fn retrieve_todo(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, impl IntoResponse> {
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

// Define a Shuttle runtime main function for serving a static folder.
#[shuttle_runtime::main]
async fn static_folder() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_shuttle))
        .nest_service("/assets", ServeDir::new(PathBuf::from("assets")));

    Ok(router.into())
}

// Define a Shuttle runtime main function for handling secrets.
#[shuttle_runtime::main]
async fn secrets(
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // Retrieve the 'API_KEY' secret from the secret store.
    let secret = if let Some(secret) = secret_store.get("API_KEY") {
        secret
    } else {
        return Err(anyhow!("secret was not found").into());
    };

    let router = Router::new().route("/", get(hello_shuttle));

    Ok(router.into())
}

// Define a Shuttle runtime main function for handling a shared PostgreSQL database.
#[shuttle_runtime::main]
async fn shared_db(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // Run database migrations using SQLx.
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    // Create an Axum router with multiple routes and add a PostgreSQL database connection pool as an extension.
    let router = Router::new()
        .route("/", get(hello_shuttle))
        .route("/todo", post(add_todo))
        .route("/todo/:id", get(retrieve_todo))
        .with_state(pool);

    Ok(router.into())
}
