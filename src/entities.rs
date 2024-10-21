use crate::common::*;
use raylib::prelude::*;
use std::borrow::Cow;

pub trait Entity {
    fn draw<'a>(&self, draw: Draw<'a>) -> Draw<'a>;
    fn update(self, input: Input) -> Self;
}

macro_rules! draw {
    ($draw:ident: $($entity:ident),+) => {{
        let draw = $draw;
        $(
            let draw = $entity.draw(draw);
        )+
        draw
    }}
}

#[derive(Copy, Clone, Debug)]
enum Justify {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Text {
    text: Cow<'static, str>,
    font_size: FontSize,
    pos: Pos,
    justify: Justify,
}

impl Text {
    fn new_inner(
        text: impl Into<Cow<'static, str>>,
        font_size: FontSize,
        x: u8,
        y: u8,
        justify: Justify,
    ) -> Self {
        Self {
            text: text.into(),
            font_size,
            pos: (x, y),
            justify,
        }
    }

    pub fn small(text: impl Into<Cow<'static, str>>, x: u8, y: u8, justify: Justify) -> Self {
        Self::new_inner(text, FontSize::Small, x, y, justify)
    }

    pub fn medium(text: impl Into<Cow<'static, str>>, x: u8, y: u8, justify: Justify) -> Self {
        Self::new_inner(text, FontSize::Medium, x, y, justify)
    }

    pub fn large(text: impl Into<Cow<'static, str>>, x: u8, y: u8, justify: Justify) -> Self {
        Self::new_inner(text, FontSize::Large, x, y, justify)
    }
}

impl Entity for Text {
    fn draw<'a>(&self, mut draw: Draw<'a>) -> Draw<'a> {
        let (font, font_size) = get_font_and_size(self.font_size);
        let position = pos_to_vec2(self.pos, get_dimensions(&draw));
        let position = match self.justify {
            Justify::Left => position,
            Justify::Center => Vector2::new(
                position.x - font.measure_text(&self.text, font_size, SPACING).x / 2.0,
                position.y,
            ),
            Justify::Right => Vector2::new(
                position.x - font.measure_text(&self.text, font_size, SPACING).x,
                position.y,
            ),
        };

        draw.draw_text_ex(
            &*font,
            &self.text,
            position,
            font_size,
            SPACING,
            DEFAULT_COLOR,
        );

        draw
    }

    fn update(self, input: Input) -> Self {
        self
    }
}

#[derive(Debug)]
pub enum State {
    Init { title: Text, prompt: Text },
    Searching { title: Text },
}

impl Default for State {
    fn default() -> Self {
        Self::Init {
            title: Text::medium("Word Search", CENTER, TOP.down(5), Justify::Center),
            prompt: Text::medium(
                "Click anywhere to start",
                CENTER,
                CENTER.up(50),
                Justify::Center,
            ),
        }
    }
}

impl Entity for State {
    fn draw<'a>(&self, draw: Draw<'a>) -> Draw<'a> {
        match self {
            Self::Init { title, prompt } => draw!(draw: title, prompt),
            Self::Searching { title } => draw!(draw: title),
        }
    }

    fn update(self, input: Input) -> Self {
        match self {
            Self::Init { title, .. } if input.clicked => Self::Searching { title },

            Self::Searching { title } => Self::Searching { title },

            _ => self,
        }
    }
}
