use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/hello", get(hello_axum));


    let listener = tokio::net::TcpListener::bind
        ("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello_axum() -> &'static str {
    "Hello, Axum!"
}
