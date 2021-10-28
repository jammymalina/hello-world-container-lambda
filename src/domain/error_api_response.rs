use std::collections::HashMap;

use super::{api_response::ApiResponse, errors::base_error::BaseError};

pub struct ErrorApiResponse;

impl ErrorApiResponse {
    pub fn init(err: &Box<dyn BaseError>) -> ApiResponse {
        let response_body = err.get_response_json();
        ApiResponse {
            status_code: err.get_status_code() as i64,
            headers: HashMap::from([
                ("Content-Type", String::from("application/json")),
                ("Content-Length", response_body.len().to_string()),
            ]),
            body: response_body,
            is_base64_encoded: false,
        }
    }
}
