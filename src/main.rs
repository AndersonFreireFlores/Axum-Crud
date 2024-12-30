mod models;
mod mouse;

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;

use axum::{routing::get, Router, ServiceExt};
use axum::handler::Handler;
use sqlx::PgPool;

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
        .with_state(Arc::new(AppState { db: pool.clone() }));


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
