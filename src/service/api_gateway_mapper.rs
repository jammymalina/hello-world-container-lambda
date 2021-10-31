use crate::{
    domain::{checkerboard_request::CheckerboardRequest, event::ApiGatewayProxyEvent},
    service::color_transformer::ColorTransformer,
};

pub struct ApiGatewayMapper;

impl ApiGatewayMapper {
    pub fn map_checkerboard_request(event: &ApiGatewayProxyEvent) -> CheckerboardRequest {
        let mut request = CheckerboardRequest {
            ..Default::default()
        };

        let keys = vec!["cellWidth", "cellHeight", "color1", "color2"];
        let entries = Self::get_query_param_entries(event, keys);
        for entry in entries {
            let (key, value) = entry;
            match key.as_str() {
                "cellWidth" => request.cell_width = value.parse::<u32>().unwrap(),
                "cellHeight" => request.cell_height = value.parse::<u32>().unwrap(),
                "color1" => request.color1 = ColorTransformer::hex_str_to_color(value).unwrap(),
                "color2" => request.color2 = ColorTransformer::hex_str_to_color(value).unwrap(),
                _ => (),
            }
        }

        request
    }

    fn get_query_param_entries<'a>(
        event: &'a ApiGatewayProxyEvent,
        keys: Vec<&str>,
    ) -> Vec<(&'a String, &'a String)> {
        keys.into_iter()
            .filter_map(|key| event.query_string_parameters.get_key_value(key))
            .collect()
    }
}
