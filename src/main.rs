async fn hello_axum() -> &'static str {
    "Hello from Axum!"
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/", axum::routing::get(hello_axum));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
