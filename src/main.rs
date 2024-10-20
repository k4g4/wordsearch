// #![windows_subsystem = "windows"]

use std::{iter, ops::Deref};

use raylib::prelude::*;

const FONT: &[u8] = include_bytes!("../font.ttf");
const FONT_COUNT: usize = 3;
const FONT_SIZES: [i32; FONT_COUNT] = [30, 45, 60];

type Fonts = [Font; FONT_COUNT];

struct Draw<'handle> {
    handle: RaylibDrawHandle<'handle>,
    fonts: &'handle Fonts,
}

impl<'handle> Draw<'handle> {
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

    fn small(self, text: &str, position: impl Into<Vector2>, color: Color) -> Self {
        self.draw_text(0, text, position, color)
    }

    fn medium(self, text: &str, position: impl Into<Vector2>, color: Color) -> Self {
        self.draw_text(1, text, position, color)
    }

    fn large(self, text: &str, position: impl Into<Vector2>, color: Color) -> Self {
        self.draw_text(2, text, position, color)
    }
}

impl Deref for Draw<'_> {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

struct Input {
    #[allow(unused)]
    pos: Vector2,
    keys: Vec<KeyboardKey>,
}

struct Global {
    handle: RaylibHandle,
    thread: RaylibThread,
    fonts: Fonts,
}

impl Global {
    fn draw(&mut self) -> Draw {
        let mut handle = self.handle.begin_drawing(&self.thread);
        handle.clear_background(Color::WHITE);

        Draw {
            handle,
            fonts: &self.fonts,
        }
    }

    fn input(&mut self) -> Input {
        Input {
            pos: self.handle.get_mouse_position(),
            keys: iter::from_fn(|| self.handle.get_key_pressed()).collect(),
        }
    }
}

impl Deref for Global {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

#[derive(Clone, Default, Debug)]
enum State {
    #[default]
    Init,
    Searching,
}

impl State {
    fn draw<'handle>(&self, draw: Draw<'handle>) -> Draw<'handle> {
        match self {
            State::Init => draw.medium("Press ENTER to begin", (10.0, 10.0), Color::BLACK),
            State::Searching => draw.small("Small...", (10.0, 10.0), Color::BLACK).large(
                "Large...",
                (10.0, 20.0),
                Color::BLACK,
            ),
        }
    }

    fn update(&mut self, Input { pos: _, keys }: Input) {
        match self {
            State::Init => {
                if keys.contains(&KeyboardKey::KEY_ENTER) {
                    *self = State::Searching;
                }
            }
            State::Searching => {
                if keys.contains(&KeyboardKey::KEY_ENTER) {
                    *self = State::Init;
                }
            }
        }
    }
}

fn main() {
    let (mut handle, thread) = init().size(600, 400).title("Word Search").build();
    handle.set_target_fps(60);
    handle.set_trace_log(TraceLogLevel::LOG_ALL);
    let fonts = FONT_SIZES.map(|font_size| {
        handle
            .load_font_from_memory(&thread, ".ttf", FONT, font_size, None)
            .unwrap()
    });
    let mut global = Global {
        handle,
        thread,
        fonts,
    };
    let mut state = State::default();

    while !global.window_should_close() {
        state.draw(global.draw());
        state.update(global.input());
    }
}
