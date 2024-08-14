use axum::{extract::Multipart, response::Json};
use buckify::{establish_connection, extractors, http::body::response, models::Resource, utils};
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

pub async fn get_resource(multipart: Multipart) -> Json<Value> {
    let connection = &mut establish_connection();
    let input = extractors::multipart(multipart)
        .await
        .map_err(|err| anyhow::anyhow!("Multipart extraction error: {}", err));

    match input {
        Ok(payload) => {
            use buckify::schema::resources::dsl::*;
            use diesel::prelude::*;

            if let Some(target_path) = payload.path {
                let result = resources
                    .filter(slug.eq(utils::slugify(&target_path)))
                    .select(Resource::as_select())
                    .first(connection)
                    .optional();

                response(result.map_err(|err| anyhow::anyhow!("{}", err)))
            } else {
                response::<()>(Err(anyhow::anyhow!("No path provided.")))
            }
        }
        Err(err) => response::<()>(Err(anyhow::anyhow!("{}", err))),
    }
}
