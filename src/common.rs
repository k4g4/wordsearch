use raylib::prelude::*;

pub const WIDTH: i32 = 600;
pub const HEIGHT: i32 = 400;
pub const TITLE: &str = "Word Search";
pub const FRAME_RATE: u32 = 60;

pub const FONT: &[u8] = include_bytes!("../font.ttf");
pub const FONT_COUNT: usize = 3;
pub const FONT_SIZES: [i32; FONT_COUNT] = [30, 45, 60];

pub type Fonts = [Font; FONT_COUNT];

pub struct Input {
    pub pos: Vector2,
    pub clicked: bool,
    pub keys: Vec<KeyboardKey>,
}
