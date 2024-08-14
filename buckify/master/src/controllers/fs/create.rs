use anyhow::{Context, Result};
use axum::{extract::Multipart, response::Json};
use buckify::{
    establish_connection, extractors, handlers,
    http::body::{response, MultipartFSPayload},
    utils,
};
use diesel::prelude::*;
use serde_json::Value;

pub async fn create_resource(multipart: Multipart) -> Json<Value> {
    let conn = &mut establish_connection();

    let data: Result<MultipartFSPayload> = extractors::multipart(multipart)
        .await
        .map_err(|err| anyhow::anyhow!("Multipart extraction error: {}", err));

    match data {
        Ok(payload) => {
            if let Some(target_path) = payload.path {
                if let Some(bytes) = payload.file {
                    match handlers::fs::create(target_path.clone(), bytes.clone()) {
                        Ok(file_data) => {
                            use buckify::{
                                models::{NewResource, Resource},
                                schema::resources,
                            };

                            let size_i32: i32 = match file_data.size.try_into() {
                                Ok(size) => size,
                                Err(_) => {
                                    eprintln!("Size conversion failed.");
                                    return response::<()>(Err(anyhow::anyhow!(
                                        "Size conversion failed."
                                    )));
                                }
                            };

                            let resource = NewResource {
                                path: &file_data.path,
                                name: &utils::extract_file_name(&file_data.path),
                                slug: &utils::slugify(&file_data.path),
                                size: size_i32,
                            };

                            let inserted_resource = diesel::insert_into(resources::table)
                                .values(&resource)
                                .execute(conn)
                                .context("Error inserting resource");

                            if let Err(err) = inserted_resource {
                                eprintln!("{}", err);
                                return response::<()>(Err(anyhow::anyhow!("{}", err)));
                            }

                            let retrieved_resource = resources::table
                                .order(resources::id.desc())
                                .select(Resource::as_select())
                                .first::<Resource>(conn)
                                .context("Error retrieving resource");

                            response(retrieved_resource.map_err(|err| anyhow::anyhow!("{}", err)))
                        }
                        Err(err) => response::<()>(Err(anyhow::anyhow!("{}", err))),
                    }
                } else {
                    response::<()>(Err(anyhow::anyhow!("No file bytes provided.")))
                }
            } else {
                response::<()>(Err(anyhow::anyhow!("No path provided.")))
            }
        }
        Err(err) => response::<()>(Err(err)),
    }
}
