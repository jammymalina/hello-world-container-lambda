use serde_json;
use std::collections::HashMap;
use valico::json_dsl::Builder;

use super::schemas::{
    base_query_params::BaseQueryParamsSchema, checkerboard::CheckerboardSchema, schema::Schema,
};
use crate::domain::{
    errors::{base_error::BaseError, invalid_query_param_error::InvalidQueryParamError},
    event::ApiGatewayProxyEvent,
    request_type::RequestType,
};

pub struct ApiValidator {
    request_validators: HashMap<RequestType, Builder>,
}

impl ApiValidator {
    pub fn init() -> Self {
        let validators =
            HashMap::from([(RequestType::Checkerboard, CheckerboardSchema::get_schema())]);
        Self {
            request_validators: validators,
        }
    }

    pub fn validate_base_event(event: &ApiGatewayProxyEvent) -> Result<(), Box<dyn BaseError>> {
        let schema = BaseQueryParamsSchema::get_schema();
        let mut event_value = serde_json::to_value(event).unwrap();

        let result = schema.process(&mut event_value, None);

        if result.is_strictly_valid() {
            return Ok(());
        }

        let error = result.errors.into_iter().next().unwrap();
        let param_path = error.get_path();
        return Err(Box::new(InvalidQueryParamError::init(param_path, None)));
    }

    pub fn validate_event(
        &self,
        request_type: RequestType,
        event: &ApiGatewayProxyEvent,
    ) -> Result<(), Box<dyn BaseError>> {
        let schema = self.request_validators.get(&request_type).unwrap();
        let mut event_value = serde_json::to_value(event).unwrap();

        let result = schema.process(&mut event_value, None);

        if result.is_strictly_valid() {
            return Ok(());
        }

        let error = result.errors.into_iter().next().unwrap();
        let param_path = Self::extract_param_name(error.get_path());
        return Err(Box::new(InvalidQueryParamError::init(param_path, None)));
    }

    fn extract_param_name(param_path: &str) -> &str {
        let safe_extract = |param: &str, start_index: usize| {
            if start_index < param.len() - 1 {
                &param_path[start_index..]
            } else {
                param_path
            }
        };

        match param_path.rfind("/") {
            Some(index) => safe_extract(param_path, index + 1),
            _ => param_path,
        }
    }
}
