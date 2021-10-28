use tiny_skia::Paint;

use crate::domain::color::Color;

pub struct ColorTransformer {}

impl ColorTransformer {
    pub fn color_to_skia_paint<'a>(color: &Color) -> Paint<'a> {
        let mut paint = Paint::default();
        match *color {
            Color::RGB(r, g, b) => paint.set_color_rgba8(r, g, b, 255),
            Color::RGBA(r, g, b, a) => paint.set_color_rgba8(r, g, b, a),
        }
        paint
    }
}
