use serde::Serialize;

#[derive(Serialize)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}
