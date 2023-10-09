mod error;
mod handler;

use anyhow::Context;
use axum::routing::{get, Router};
use handler::{get_greetings, increment_greetings};
use sqlx::Executor;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn shuttle_main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../migrations/schema.sql"))
        .await
        .context("failed to run migration")?;

    let router = Router::new()
        .route("/greeting", get(get_greetings).put(increment_greetings))
        .with_state(pool)
        .nest_service("/", ServeDir::new("assets"));

    Ok(router.into())
}
