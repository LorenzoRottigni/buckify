use crate::FSPayload;
use std::error::Error;
use std::str;

use axum::extract::Multipart;

pub async fn multipart(mut multipart: Multipart) -> Result<FSPayload, Box<dyn Error>> {
    let mut payload = FSPayload {
        path: None,
        file: None,
    };
    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().unwrap().to_string();
        let value = field.bytes().await?;
        match field_name.as_str() {
            "path" => {
                // parse raw bytes to a string
                if let Ok(path_str) = str::from_utf8(&value) {
                    payload.path = Some(path_str.to_string());
                } else {
                    // Handle invalid UTF-8 data (e.g., return Err(...))
                    // return Err(Box::new(YourCustomError::InvalidPath));
                }
            }
            "file" => payload.file = Some(value),
            _ => println!("Ignoring unknown field: {}", field_name),
        }
    }
    Ok(payload)
}
