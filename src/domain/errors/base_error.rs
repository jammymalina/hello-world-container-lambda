use serde_json::json;

use super::error_severity::ErrorSeverity;
use crate::domain::http_status::HttpStatusCode;

pub trait BaseError {
    fn get_message(&self) -> String;
    fn get_severity(&self) -> ErrorSeverity;
    fn get_type(&self) -> String;
    fn get_status_code(&self) -> HttpStatusCode;

    fn get_response_json(&self) -> String {
        let response = json!({
            "message": self.get_message(),
            "type": self.get_type(),
        });
        response.to_string()
    }
}
