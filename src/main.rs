#![allow(unused_imports)]

pub mod todo;

use anyhow::anyhow;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;

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

// // Define a Shuttle runtime main function for handling secrets.
// #[shuttle_runtime::main]
// async fn secrets(
//     #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
// ) -> shuttle_axum::ShuttleAxum {
//     // Retrieve the 'API_KEY' secret from the secret store.
//     let secret = if let Some(secret) = secret_store.get("API_KEY") {
//         secret
//     } else {
//         return Err(anyhow!("secret was not found").into());
//     };
//
//     let router = Router::new().route("/", get(hello_shuttle));
//
//     Ok(router.into())
// }
//
// // Define a Shuttle runtime main function for handling a shared PostgreSQL database.
// #[shuttle_runtime::main]
// async fn shared_db(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
//     // Run database migrations using SQLx.
//     sqlx::migrate!()
//         .run(&pool)
//         .await
//         .map_err(CustomError::new)?;
//
//     // Create an Axum router with multiple routes and add a PostgreSQL database connection pool as an extension.
//     let router = Router::new()
//         .route("/", get(hello_shuttle))
//         .route("/todo", post(todo::add))
//         .route("/todo/:id", get(todo::retrieve))
//         .layer(Extension(Arc::new(pool)));
//
//     Ok(router.into())
// }
