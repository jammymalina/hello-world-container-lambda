use lambda_runtime::{handler_fn, Context, Error};

use hello_world_container_lambda::{
    domain::{
        api_response::ApiResponse,
        checkerboard_request::CheckerboardRequest,
        error_api_response::ErrorApiResponse,
        errors::{base_error::BaseError, error_severity::ErrorSeverity},
        event::ApiGatewayProxyEvent,
        image_api_response::ImageApiResponse,
        request_type::RequestType,
        shape::Shape,
    },
    service::{
        api_gateway_mapper::ApiGatewayMapper,
        dependency_resolver::{Dependecies, DependencyResolver},
        validator::api_validator::ApiValidator,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_env().unwrap();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: ApiGatewayProxyEvent, _: Context) -> Result<ApiResponse, Error> {
    log::debug!("Running in debug mode");
    log::info!("Event: {}", event);

    let status = ApiValidator::validate_base_event(&event);
    if status.is_err() {
        let err = status.unwrap_err();
        return Ok(fail(&err));
    }

    let mut dependencies = DependencyResolver::resolve_dependecies(&event);
    let Dependecies { api_validator, .. } = &dependencies;

    let status = api_validator.validate_event(RequestType::Checkerboard, &event);
    if status.is_err() {
        let err = status.unwrap_err();
        return Ok(fail(&err));
    }

    let req = ApiGatewayMapper::map_checkerboard_request(&event);
    let resp = execute(&req, &mut dependencies);
    Ok(resp)
}

fn execute(request: &CheckerboardRequest, dependencies: &mut Dependecies) -> ApiResponse {
    log::info!(
        "Received {} request on {}",
        dependencies.method,
        dependencies.path
    );

    let canvas = dependencies.canvas.as_mut();
    let colors = [request.color1, request.color2];
    let mut color_flag_y = false;
    let mut color_flag_x;
    for y in (0..canvas.get_height()).step_by(request.cell_height as usize) {
        color_flag_x = color_flag_y;
        for x in (0..canvas.get_width()).step_by(request.cell_width as usize) {
            canvas.draw_shape(
                &Shape::RECT {
                    x: x as f32,
                    y: y as f32,
                    width: request.cell_width as f32,
                    height: request.cell_height as f32,
                },
                Some(colors[color_flag_x as usize]),
            );
            color_flag_x = !color_flag_x;
        }
        color_flag_y = !color_flag_y;
    }

    ImageApiResponse::init_from_base64(&canvas.get_base64_png_data())
}

fn fail(err: &Box<dyn BaseError>) -> ApiResponse {
    match err.get_severity() {
        ErrorSeverity::Warning => log::warn!("{}", err.get_message()),
        ErrorSeverity::Error => log::error!("{}", err.get_message()),
        ErrorSeverity::Fatal => log::error!("{}", err.get_message()),
    };
    ErrorApiResponse::init(&err)
}
