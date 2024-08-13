pub mod extractors;
pub mod handlers;
pub mod models;
pub mod schema;

use axum::body::Bytes;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct FSPayload {
    pub path: Option<String>,
    pub file: Option<Bytes>,
}
