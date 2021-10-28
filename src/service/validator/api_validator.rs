use serde_json;

use super::schemas::{base_query_params::BaseQueryParamsSchema, schema::Schema};
use crate::domain::{
    errors::{base_error::BaseError, invalid_query_param_error::InvalidQueryParamError},
    event::ApiGatewayProxyEvent,
};

pub struct ApiValidator {}

impl ApiValidator {
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
}
