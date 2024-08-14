use axum::{
    routing::{delete, get, post},
    Router,
};

pub mod controllers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(controllers::fs::create::create_resource))
        .route("/read", get(controllers::fs::read::get_resources))
        .route("/delete", delete(controllers::fs::delete::delete_resource));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
