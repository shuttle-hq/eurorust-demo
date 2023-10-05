use axum::{routing::get, Router};

async fn hello_shuttle() -> &'static str {
    "Hello from Shuttle!"
}

#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_shuttle));

    Ok(router.into())
}
