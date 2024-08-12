use axum::{routing::post, Router};

pub mod controller;

#[tokio::main]
async fn main() {
    println!("Master project starting...");
    let app = Router::new().route("/upload", post(controller::fs::create));
    // .route("/upload/*path", post(fs::handler));
    // .route("/get_upload/:path", get(fs::handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
