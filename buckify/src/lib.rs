pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod fs {
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    pub fn write_file(data: &[u8], path: &Path) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }
}

pub mod handlers {
    use std::error::Error;
    use std::str;

    use axum::{body::Bytes, extract::Multipart};

    pub struct FSPayload {
        pub path: Option<String>,
        pub file: Option<Bytes>,
    }

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
}
