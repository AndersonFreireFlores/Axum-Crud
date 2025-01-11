mod models;
mod mouse;
mod handlers;
mod error;

use sqlx::postgres::PgPoolOptions;

use axum::{routing::get, Router, ServiceExt};
use axum::handler::Handler;
use axum::routing::post;
use sqlx::PgPool;
use crate::handlers::{create_mouse, get_all_mouses};


#[tokio::main]
async fn main() {


    let database_url = "postgres://postgres:password@localhost:5432/axum_db";
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/hello", get(hello_axum))
        .route("/mouses", get(get_all_mouses))
        .route("/mouse", post(create_mouse))
        .with_state(pool);


    let listener = tokio::net::TcpListener::bind
        ("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub struct AppState {
    db: PgPool,
}


async fn hello_axum() -> &'static str {
    "Hello, Axum!"
}
