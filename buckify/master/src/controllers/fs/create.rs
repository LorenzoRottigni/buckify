use axum::{extract::Multipart, response::Json};
use base64::encode;
use buckify::{establish_connection, extractors, handlers, FSPayload};
use serde_json::{json, Value};
use std::convert::TryInto;
use std::io::{Error, ErrorKind};

pub async fn create_resource(multipart: Multipart) -> Json<Value> {
    let conn = &mut establish_connection();
    let result: Result<FSPayload, Box<dyn std::error::Error>> =
        extractors::multipart(multipart).await;

    match result {
        Ok(payload) => {
            if let Some(target_path) = payload.path {
                if let Some(bytes) = payload.file {
                    match handlers::fs::create(target_path.clone(), bytes.clone()) {
                        Ok(file_data) => {
                            use buckify::{
                                models::{NewResource, Resource},
                                schema::resources,
                            };
                            use diesel::prelude::*;

                            let size_i32: i32 = match file_data.size.try_into() {
                                Ok(size) => size,
                                Err(_) => {
                                    eprintln!("Size conversion failed.");
                                    return Json(json!({
                                        "success": "false",
                                        "error": "Size conversion failed.",
                                    }));
                                }
                            };

                            let resource = NewResource {
                                path: &file_data.path,
                                name: &file_data.path,
                                slug: &file_data.path,
                                size: size_i32,
                            };

                            // Insert into the database
                            let inserted_resource = diesel::insert_into(resources::table)
                                .values(&resource)
                                .execute(conn);

                            match inserted_resource {
                                Ok(_) => println!("Resource inserted successfully."),
                                Err(err) => eprintln!("Error inserting resource: {}", err),
                            }

                            // Retrieve the most recently inserted resource
                            let retrieved_resource = resources::table
                                .order(resources::id.desc())
                                .select(Resource::as_select()) // Ensure that `as_select()` is correct
                                .first::<Resource>(conn);

                            println!("After save: {}", file_data.path);

                            match retrieved_resource {
                                Ok(resource) => println!("Retrieved resource: {:?}", resource),
                                Err(err) => eprintln!("Error retrieving resource: {}", err),
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating file: {}", err)
                        }
                    }

                    Json(json!({
                        "success": "true",
                        "path": target_path,
                        "file": encode(&bytes),
                    }))
                } else {
                    eprintln!("No file bytes provided.");
                    Json(json!({
                        "success": "false",
                        "error": "No file bytes provided.",
                    }))
                }
            } else {
                eprintln!("No path provided in payload.");
                Json(json!({
                    "success": "false",
                    "error": "No path provided.",
                }))
            }
        }
        Err(err) => {
            eprintln!("Error processing multipart data: {}", err);
            Json(json!({
                "success": "false",
                "error": format!("Error processing multipart data: {}", err),
            }))
        }
    }
}
