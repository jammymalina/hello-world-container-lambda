use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ApiResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    pub headers: HashMap<&'static str, String>,
    pub body: String,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: bool,
}
