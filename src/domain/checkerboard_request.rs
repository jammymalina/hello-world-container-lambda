use logger::*;
use serde::Serialize;

use super::color::Color;

#[derive(Serialize, JsonDisplay)]
pub struct CheckerboardRequest {
    pub cell_width: u32,
    pub cell_height: u32,
    pub color1: Color,
    pub color2: Color,
}

impl Default for CheckerboardRequest {
    fn default() -> Self {
        Self {
            cell_width: Self::DEFAULT_CELL_DIMENSION,
            cell_height: Self::DEFAULT_CELL_DIMENSION,
            color1: Self::DEFAULT_COLOR1,
            color2: Self::DEFAULT_COLOR2,
        }
    }
}

impl CheckerboardRequest {
    pub const DEFAULT_CELL_DIMENSION: u32 = 20 as u32;
    pub const DEFAULT_COLOR1: Color = Color::RGB(255, 255, 255);
    pub const DEFAULT_COLOR2: Color = Color::RGB(0, 0, 0);
}
