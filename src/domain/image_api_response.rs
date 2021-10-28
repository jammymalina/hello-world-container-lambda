use super::{api_response::ApiResponse, http_status::HttpStatusCode};

pub struct ImageApiResponse;

impl ImageApiResponse {
    pub fn init_from_base64(image: &str) -> ApiResponse {
        ApiResponse {
            status_code: HttpStatusCode::Ok as i64,
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
