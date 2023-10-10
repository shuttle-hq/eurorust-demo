mod error;
mod handler;

use anyhow::Context;
use axum::{routing::get, Router};
use sqlx::{Executor, PgPool};
use tower_http::services::ServeDir;

async fn hello_shuttle() -> &'static str {
    "Hello from Shuttle!"
}

#[shuttle_runtime::main]
async fn shuttle_main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../migrations/schema.sql"))
        .await
        .context("failed to run migration")?;

    let router = Router::new()
        .route("/", get(hello_shuttle))
        .route(
            "/greeting",
            get(handler::get_greetings).put(handler::increment_greetings),
        )
        .with_state(pool)
        .nest_service("/assets", ServeDir::new("assets"));

    Ok(router.into())
}
