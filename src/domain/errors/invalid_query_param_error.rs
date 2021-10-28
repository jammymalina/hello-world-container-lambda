use super::{base_error::BaseError, error_severity::ErrorSeverity};
use crate::domain::http_status::HttpStatusCode;

pub struct InvalidQueryParamError {
    message: String,
}

impl BaseError for InvalidQueryParamError {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_severity(&self) -> super::error_severity::ErrorSeverity {
        ErrorSeverity::Warning
    }

    fn get_type(&self) -> String {
        String::from("InvalidQueryParameterError")
    }

    fn get_status_code(&self) -> HttpStatusCode {
        HttpStatusCode::BadRequest
    }
}

impl InvalidQueryParamError {
    pub fn init(query_param: &str, addition: Option<&str>) -> Self {
        let message_suffix = match addition {
            Some(a) => format!(", {}", a),
            _ => String::from(""),
        };
        InvalidQueryParamError {
            message: format!("Invalid query parameter {}{}", query_param, message_suffix),
        }
    }
}
