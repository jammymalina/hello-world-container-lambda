use lambda_runtime::{handler_fn, Context, Error};

use hello_world_container_lambda::{
    domain::{
        color::Color, event::APIGatewayProxyEvent, image_api_response::ImageApiResponse,
        shape::Shape,
    },
    service::dependency_resolver::{Dependecies, DependencyResolver},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_env().unwrap();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: APIGatewayProxyEvent, _: Context) -> Result<ImageApiResponse, Error> {
    log::debug!("Running in debug mode");

    let mut dependencies = DependencyResolver::resolve_dependecies(&event);
    let resp = execute(&mut dependencies);

    Ok(resp)
}

fn execute(dependencies: &mut Dependecies) -> ImageApiResponse {
    log::info!(
        "Received {} request on {}",
        dependencies.method,
        dependencies.path
    );

    let canvas = dependencies.canvas.as_mut();
    let cell_width = 20;
    let cell_height = 20;
    let colors = [Color::RGB(255, 255, 255), Color::RGB(0, 0, 0)];
    let mut color_flag = true;
    for y in (0..canvas.get_height()).step_by(cell_height) {
        for x in (0..canvas.get_width()).step_by(cell_width) {
            canvas.draw_shape(
                &Shape::RECT {
                    x: x as f32,
                    y: y as f32,
                    width: cell_width as f32,
                    height: cell_height as f32,
                },
                Some(&colors[color_flag as usize]),
            );
            color_flag = !color_flag;
        }
        color_flag = !color_flag;
    }

    ImageApiResponse::init_from_base64(&canvas.get_base64_png_data())
}
