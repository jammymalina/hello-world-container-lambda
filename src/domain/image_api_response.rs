use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ImageApiResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    pub headers: HashMap<&'static str, String>,
    pub body: String,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
}

impl ImageApiResponse {
    pub fn init_from_base64(image: &str) -> ImageApiResponse {
        ImageApiResponse {
            status_code: 200,
            headers: [
                ("Content-Type", String::from("image/png")),
                ("Content-Length", image.len().to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
            body: image.to_string(),
            is_base64_encoded: true,
        }
    }
}
