use axum::{extract::Multipart, response::Json};
use base64::encode;
use serde_json::{json, Value};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use buckify::handlers::{self, FSPayload};

pub async fn create(multipart: Multipart) -> Json<Value> {
    let result: Result<FSPayload, Box<dyn std::error::Error>> =
        handlers::multipart(multipart).await;

    match result {
        Ok(payload) => {
            if let Some(path) = payload.path {
                if let Some(bytes) = payload.file {
                    if let Some(parent) = Path::new(&path).parent() {
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
                        eprintln!("Invalid path: {}", path);
                    }
                    if Path::new(&path).exists() {
                        eprintln!("File {} already exists. Skipping write.", path);
                    }

                    match File::create(&path) {
                        Ok(mut file) => {
                            if let Err(e) = file.write_all(&bytes) {
                                eprintln!("Failed to write to file {}: {}", path, e);
                                return Json(json!({
                                    "success": "false",
                                    "error": format!("Failed to write to file: {}", e),
                                }));
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to create file {}: {}", path, e);
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
