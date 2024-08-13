use axum::{extract::Multipart, response::Json};
use base64::encode;
use buckify::establish_connection;
use buckify::handlers::{self, FSPayload};

use serde_json::{json, Value};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

pub async fn create_resource(multipart: Multipart) -> Json<Value> {
    let conn = &mut establish_connection();

    use buckify::models::{NewResource, Resource};
    use buckify::schema::resources;
    use buckify::schema::resources::dsl::*;
    use diesel::prelude::*;

    let result: Result<FSPayload, Box<dyn std::error::Error>> =
        handlers::multipart(multipart).await;

    match result {
        Ok(payload) => {
            if let Some(target_path) = payload.path {
                if let Some(bytes) = payload.file {
                    if let Some(parent) = Path::new(&target_path).parent() {
                        println!("Parent directory: {}", parent.display());
                        if !Path::new(parent).exists() {
                            println!("Creating parent directory: {}", parent.display());
                            create_dir_all(parent).unwrap_or_else(|err| {
                                eprintln!("Error creating parent directory: {}", err);
                            });
                        } else {
                            println!("Parent directory already exists: {}", parent.display());
                        }
                    } else {
                        eprintln!("Invalid path: {}", target_path);
                    }
                    if Path::new(&target_path).exists() {
                        eprintln!("File {} already exists. Skipping write.", target_path);
                    }

                    match File::create(&target_path) {
                        Ok(mut file) => {
                            if let Err(e) = file.write_all(&bytes) {
                                eprintln!("Failed to write to file {}: {}", target_path, e);
                                return Json(json!({
                                    "success": "false",
                                    "error": format!("Failed to write to file: {}", e),
                                }));
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to create file {}: {}", target_path, e);
                            match e.kind() {
                                std::io::ErrorKind::PermissionDenied => {
                                    return Json(json!({
                                        "success": "false",
                                        "error": "Permission denied",
                                    }));
                                }
                                std::io::ErrorKind::NotFound => {
                                    return Json(json!({
                                        "success": "false",
                                        "error": "Directory not found",
                                    }));
                                }
                                _ => {
                                    return Json(json!({
                                        "success": "false",
                                        "error": format!("Failed to create file: {}", e),
                                    }));
                                }
                            }
                        }
                    }

                    let resource = NewResource {
                        name: &target_path,
                        slug: &target_path,
                        path: &target_path,
                    };

                    diesel::insert_into(resources::table)
                        .values(&resource)
                        .returning(Resource::as_returning())
                        .get_result(conn)
                        .expect("Error saving new post");

                    Json(json!({
                        "success": "true",
                        "path": path,
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
