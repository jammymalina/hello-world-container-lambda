use super::{color::Color, shape::Shape};

pub trait Canvas {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn draw_shape(&mut self, shape: &Shape, fill: Option<&Color>);
    fn get_base64_png_data(&self) -> String;
}
