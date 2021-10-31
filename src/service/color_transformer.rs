use tiny_skia::Paint;

use crate::domain::color::Color;

pub struct ColorTransformer;

impl ColorTransformer {
    const INVALID_COLOR_MESSAGE: &'static str = "Invalid color";

    pub fn color_to_skia_paint<'a>(color: Color) -> Paint<'a> {
        let mut paint = Paint::default();
        match color {
            Color::RGB(r, g, b) => paint.set_color_rgba8(r, g, b, 255),
            Color::RGBA(r, g, b, a) => paint.set_color_rgba8(r, g, b, a),
        }
        paint
    }

    pub fn hex_str_to_color(hex_color: &str) -> Result<Color, &'static str> {
        let hex_color = if hex_color.starts_with("#") {
            &hex_color[1..]
        } else {
            hex_color
        };
        match hex_color.len() {
            3 | 4 => Self::hex_bytes_to_color(hex_color, 1),
            6 | 8 => Self::hex_bytes_to_color(hex_color, 2),
            _ => Err(Self::INVALID_COLOR_MESSAGE),
        }
    }

    fn hex_bytes_to_color(hex_color: &str, color_size: usize) -> Result<Color, &'static str> {
        let colors = (0..hex_color.len())
            .step_by(color_size)
            .map(|i| u8::from_str_radix(&hex_color[i..i + color_size], 16))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Self::INVALID_COLOR_MESSAGE)?;

        let map_color = |value: u8| {
            if color_size == 1 {
                return (value << 4) + value;
            }
            value
        };
        let mut colors = colors.into_iter().map(map_color);

        match colors.len() {
            3 => Ok(Color::RGB(
                colors.next().unwrap(),
                colors.next().unwrap(),
                colors.next().unwrap(),
            )),
            4 => Ok(Color::RGBA(
                colors.next().unwrap(),
                colors.next().unwrap(),
                colors.next().unwrap(),
                colors.next().unwrap(),
            )),
            _ => Err(Self::INVALID_COLOR_MESSAGE),
        }
    }
}
