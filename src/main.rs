pub mod todo;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
    #[shuttle_static_folder::StaticFolder(folder = "assets")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/todo", post(todo::add))
        .route("/todo/:id", get(todo::retrieve))
        .layer(Extension(Arc::new(pool)))
        .nest_service("/assets", ServeDir::new(static_folder));

    Ok(router.into())
}
