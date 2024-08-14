pub mod body {
    use anyhow::Error;
    use axum::body::Bytes;
    use axum::response::Json;
    use serde::Serialize;
    use serde_json::{json, Value};

    pub struct MultipartFSPayload {
        pub path: Option<String>,
        pub file: Option<Bytes>,
    }

    pub fn response<T: Serialize>(res: Result<T, Error>) -> Json<Value> {
        match res {
            Ok(data) => Json(json!({
                "success": "true",
                "data": data
            })),
            Err(err) => Json(json!({
                "success": "false",
                "error": err.to_string(),
            })),
        }
    }
}
