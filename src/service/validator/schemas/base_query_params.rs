use valico::json_dsl::{self, Builder};

use super::schema::{self, Schema};

pub struct BaseQueryParamsSchema;

impl Schema for BaseQueryParamsSchema {
    fn get_schema() -> Builder {
        Builder::build(|params| {
            params.req_nested("queryStringParameters", json_dsl::object(), |query| {
                query.opt("width", |width| {
                    width.coerce(json_dsl::string());
                    width.regex(schema::get_positive_integer_regex())
                });
                query.opt("height", |height| {
                    height.coerce(json_dsl::string());
                    height.regex(schema::get_positive_integer_regex());
                });
            });
        })
    }
}
