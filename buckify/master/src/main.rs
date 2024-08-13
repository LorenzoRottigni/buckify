use axum::{routing::post, Router};

pub mod controllers;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(controllers::fs::create::create_resource));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
