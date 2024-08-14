use axum::response::Json;
use buckify::{establish_connection, http::body::response, models::Resource};
use serde_json::Value;

pub async fn get_resources() -> Json<Value> {
    use buckify::schema::resources::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();
    let results = resources
        .limit(100)
        .select(Resource::as_select())
        .load(connection)
        .expect("Error loading resources");

    response(Ok(results))
}
