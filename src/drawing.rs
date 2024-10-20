use crate::common::*;
use raylib::prelude::*;
use std::ops::Deref;

pub struct Draw<'handle> {
    handle: RaylibDrawHandle<'handle>,
    fonts: &'handle Fonts,
}

impl<'handle> Draw<'handle> {
    pub fn new(handle: RaylibDrawHandle<'handle>, fonts: &'handle Fonts) -> Self {
        Self { handle, fonts }
    }

    fn draw_text(
        mut self,
        font_index: usize,
        text: &str,
        position: impl Into<Vector2>,
        color: Color,
    ) -> Self {
        self.handle.draw_text_ex(
            &self.fonts[font_index],
            text,
            position.into(),
            FONT_SIZES[font_index] as _,
            2.0,
            color,
        );
        self
    }

    pub fn small(self, text: &str, position: impl Into<Vector2>, color: Color) -> Self {
        self.draw_text(0, text, position, color)
    }

    pub fn medium(self, text: &str, position: impl Into<Vector2>, color: Color) -> Self {
        self.draw_text(1, text, position, color)
    }

    pub fn large(self, text: &str, position: impl Into<Vector2>, color: Color) -> Self {
        self.draw_text(2, text, position, color)
    }
}

impl Deref for Draw<'_> {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}
