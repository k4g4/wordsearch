use raylib::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub const WIDTH: i32 = 600;
pub const HEIGHT: i32 = 800;
pub const TITLE: &str = "Word Search";
pub const FRAME_RATE: u32 = 60;
pub const DEFAULT_COLOR: Color = Color::BLACK;
pub const SPACING: f32 = 2.0;

pub const FONT_DATA: &[u8] = include_bytes!("../font.ttf");
pub const FONT_COUNT: usize = 3;
pub const FONT_SIZES: [i32; FONT_COUNT] = [30, 45, 60];

pub type Draw<'a> = RaylibDrawHandle<'a>;

pub type Pos = (u8, u8);

pub fn pos_to_vec2(pos: Pos, dimensions: Vector2) -> Vector2 {
    dimensions * Vector2::new(pos.0 as f32 / 255.0, pos.1 as f32 / 255.0)
}

pub const CENTER: u8 = 127;
pub const LEFT: u8 = 0;
pub const RIGHT: u8 = 255;
pub const TOP: u8 = 0;
pub const BOTTOM: u8 = 255;

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum FontSize {
    Small,
    Medium,
    Large,
}

thread_local! {
    pub static FONTS: RefCell<Option<[Rc<Font>; FONT_COUNT]>> = RefCell::new(None);
}

pub fn init_fonts(mut initialize: impl FnMut(&[u8], i32) -> Font) {
    FONTS.replace(Some(
        FONT_SIZES.map(|font_size| Rc::new(initialize(FONT_DATA, font_size))),
    ));
}

pub fn get_font_and_size(size: FontSize) -> (Rc<Font>, f32) {
    let i = size as usize;
    let font = FONTS.with_borrow(|fonts| fonts.as_ref().expect("fonts not initialized")[i].clone());
    let font_size = FONT_SIZES[i] as f32;
    (font, font_size)
}

pub struct Input {
    pub clicked: bool,
    pub keys: Vec<KeyboardKey>,
}

pub fn get_dimensions(handle: &RaylibHandle) -> Vector2 {
    Vector2::new(
        handle.get_screen_width() as f32,
        handle.get_screen_height() as f32,
    )
}

pub trait Reposition {
    fn up(self, by: Self) -> Self;
    fn down(self, by: Self) -> Self;
    fn left(self, by: Self) -> Self;
    fn right(self, by: Self) -> Self;
}

impl Reposition for u8 {
    fn up(self, by: u8) -> Self {
        self - by
    }
    fn down(self, by: u8) -> Self {
        self + by
    }
    fn left(self, by: u8) -> Self {
        self - by
    }
    fn right(self, by: u8) -> Self {
        self + by
    }
}
