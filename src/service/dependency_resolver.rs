use super::{canvas_skia::CanvasSkia, validator::api_validator::ApiValidator};
use crate::domain::{canvas::Canvas, event::ApiGatewayProxyEvent};

pub struct EnvironmentVariables {
    pub stage: String,
    pub aws_region: String,
}

pub struct Dependecies {
    pub transaction_id: String,
    pub path: String,
    pub method: String,
    pub canvas: Box<dyn Canvas>,
    pub api_validator: ApiValidator,
}

pub struct DependencyResolver {}

impl DependencyResolver {
    pub fn resolve_environment_variables() -> EnvironmentVariables {
        EnvironmentVariables {
            stage: String::from(std::env::var("STAGE").unwrap()),
            aws_region: String::from(std::env::var("AWS_DEFAULT_REGION").unwrap()),
        }
    }

    pub fn resolve_dependecies(event: &ApiGatewayProxyEvent) -> Dependecies {
        let transaction_id = event
            .request_context
            .request_id
            .clone()
            .unwrap_or(String::from(""));
        let path = event
            .request_context
            .http
            .path
            .clone()
            .unwrap_or(String::from(""));
        let method = event.request_context.http.method.clone().to_uppercase();

        Dependecies {
            transaction_id,
            path,
            method,
            canvas: Box::new(CanvasSkia::init(
                Self::parse_image_size("width", event),
                Self::parse_image_size("height", event),
            )),
            api_validator: ApiValidator::init(),
        }
    }

    fn parse_image_size(param: &str, event: &ApiGatewayProxyEvent) -> Option<u32> {
        match event.query_string_parameters.get(param) {
            Some(value) => Some(value.parse::<u32>().unwrap()),
            _ => None,
        }
    }
}
