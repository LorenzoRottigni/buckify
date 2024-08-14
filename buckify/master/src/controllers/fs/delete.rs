use axum::{extract::Multipart, response::Json};
use buckify::{establish_connection, extractors, http::body::response, utils};
use serde_json::Value;

pub async fn delete_resource(multipart: Multipart) -> Json<Value> {
    let conn = &mut establish_connection();

    let input = extractors::multipart(multipart)
        .await
        .map_err(|err| anyhow::anyhow!("Multipart extraction error: {}", err));

    match input {
        Ok(payload) => {
            use buckify::schema::resources::dsl::*;
            use diesel::prelude::*;

            if let Some(target_path) = payload.path {
                let result =
                    diesel::delete(resources.filter(slug.eq(utils::slugify(&target_path))))
                        .execute(conn);

                response(result.map_err(|err| anyhow::anyhow!("{}", err)))
            } else {
                response::<()>(Err(anyhow::anyhow!("No path provided.")))
            }
        }
        Err(err) => response::<()>(Err(anyhow::anyhow!("{}", err))),
    }
}
