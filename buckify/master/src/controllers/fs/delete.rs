use axum::{extract::Multipart, response::Json};
use buckify::{establish_connection, extractors, http::body::response, utils};
use serde_json::Value;

pub async fn delete_resource(multipart: Multipart) -> Json<Value> {
    let conn = &mut establish_connection();

    let data = extractors::multipart(multipart)
        .await
        .map_err(|err| anyhow::anyhow!("Multipart extraction error: {}", err));

    match data {
        Ok(payload) => {
            use buckify::schema::resources::dsl::*;
            use diesel::prelude::*;

            if let Some(target_path) = payload.path {
                let result =
                    diesel::delete(resources.filter(slug.eq(utils::slugify(&target_path))))
                        .execute(conn);

                match result {
                    Ok(affected) => response(Ok(format!("{:?}", affected))),
                    Err(err) => response::<()>(Err(anyhow::anyhow!("{}", err))),
                }

                // response::<String>(Ok(format!("Deleted {} records.", result)))
            } else {
                response::<()>(Err(anyhow::anyhow!("No path provided.")))
            }
        }
        Err(err) => response::<()>(Err(anyhow::anyhow!("{}", err))),
    }
}
