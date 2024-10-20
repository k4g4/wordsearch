// #![windows_subsystem = "windows"]

mod common;
mod drawing;
mod state;

use crate::{common::*, drawing::Draw, state::State};
use raylib::prelude::*;
use std::{iter, ops::Deref};

struct Main {
    handle: RaylibHandle,
    thread: RaylibThread,
    fonts: Fonts,
}

impl Main {
    fn draw(&mut self) -> Draw {
        let mut handle = self.handle.begin_drawing(&self.thread);
        handle.clear_background(Color::WHITE);

        Draw::new(handle, &self.fonts)
    }

    fn input(&mut self) -> Input {
        Input {
            pos: self.handle.get_mouse_position(),
            clicked: self
                .handle
                .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
            keys: iter::from_fn(|| self.handle.get_key_pressed()).collect(),
        }
    }
}

impl Default for Main {
    fn default() -> Self {
        let (mut handle, thread) = init()
            .size(WIDTH, HEIGHT)
            .title(TITLE)
            .resizable()
            .log_level(TraceLogLevel::LOG_ALL)
            .build();

        handle.set_target_fps(FRAME_RATE);
        let fonts = FONT_SIZES.map(|font_size| {
            handle
                .load_font_from_memory(&thread, ".ttf", FONT, font_size, None)
                .unwrap()
        });

        Self {
            handle,
            thread,
            fonts,
        }
    }
}

impl Deref for Main {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

fn main() {
    let (mut main, mut state) = (Main::default(), State::default());

    while !main.window_should_close() {
        state.draw(main.draw());
        state.update(main.input());
    }
}
