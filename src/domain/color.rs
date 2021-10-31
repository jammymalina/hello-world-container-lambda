use serde::Serialize;

#[derive(Copy, Clone, Serialize)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}
