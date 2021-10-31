use valico::json_dsl::{self, Builder};

use super::schema::{self, Schema};

pub struct CheckerboardSchema;

impl Schema for CheckerboardSchema {
    fn get_schema() -> Builder {
        Builder::build(|params| {
            params.req_nested("queryStringParameters", json_dsl::object(), |query| {
                query.opt("cellWidth", |cell_width| {
                    cell_width.coerce(json_dsl::string());
                    cell_width.regex(schema::get_positive_integer_regex())
                });
                query.opt("cellHeight", |cell_height| {
                    cell_height.coerce(json_dsl::string());
                    cell_height.regex(schema::get_positive_integer_regex());
                });
                query.opt("color1", |color1| {
                    color1.coerce(json_dsl::string());
                    color1.regex(schema::get_hex_color_regex())
                });
                query.opt("color2", |color2| {
                    color2.coerce(json_dsl::string());
                    color2.regex(schema::get_hex_color_regex());
                });
            });
        })
    }
}
