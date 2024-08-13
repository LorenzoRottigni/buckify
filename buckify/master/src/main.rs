use axum::{routing::post, Router};

pub mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/upload", post(controller::fs::create::create_resource));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
