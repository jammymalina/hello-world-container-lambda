use tiny_skia::{FillRule, Path, PathBuilder, Pixmap, Transform};

use super::color_transformer::ColorTransformer;
use crate::domain::canvas::Canvas;
use crate::domain::color::Color;
use crate::domain::shape::Shape;

pub struct CanvasSkia {
    pixmap: Pixmap,
}

impl Canvas for CanvasSkia {
    fn get_width(&self) -> u32 {
        self.pixmap.width()
    }

    fn get_height(&self) -> u32 {
        self.pixmap.height()
    }

    fn draw_shape(&mut self, shape: &Shape, fill: Option<&Color>) {
        let path = match *shape {
            Shape::RECT {
                x,
                y,
                width,
                height,
            } => self.draw_rect(x, y, width, height),
        };

        if fill.is_some() {
            let fill_color = fill.unwrap();
            let skia_paint = ColorTransformer::color_to_skia_paint(fill_color);
            self.pixmap.fill_path(
                &path,
                &skia_paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }

    fn get_base64_png_data(&self) -> String {
        let data = self.pixmap.encode_png().unwrap();
        base64::encode(data)
    }
}

impl CanvasSkia {
    const DEFAULT_IMAGE_SIZE: u32 = 200;

    pub fn init(width: Option<u32>, height: Option<u32>) -> Self {
        let pixmap = Pixmap::new(
            width.unwrap_or(Self::DEFAULT_IMAGE_SIZE),
            height.unwrap_or(Self::DEFAULT_IMAGE_SIZE),
        )
        .unwrap();
        Self { pixmap }
    }

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> Path {
        let mut pb = PathBuilder::new();
        pb.push_rect(x, y, width, height);
        pb.finish().unwrap()
    }
}
