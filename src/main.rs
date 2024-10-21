// #![windows_subsystem = "windows"]

mod common;
mod entities;

use crate::{common::*, entities::State};
use entities::Entity;
use raylib::prelude::*;
use std::iter;

struct Main {
    handle: RaylibHandle,
    thread: RaylibThread,
}

impl Main {
    fn draw(&mut self) -> Draw {
        let mut handle = self.handle.begin_drawing(&self.thread);
        handle.clear_background(Color::WHITE);
        handle
    }

    fn input(&mut self) -> Input {
        Input {
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
        handle.set_window_min_size(WIDTH, HEIGHT);

        init_fonts(|font_data, font_size| {
            handle
                .load_font_from_memory(&thread, ".ttf", font_data, font_size, None)
                .unwrap()
        });

        Self { handle, thread }
    }
}

fn main() {
    let (mut main, mut state) = (Main::default(), State::default());

    while !main.handle.window_should_close() {
        state.draw(main.draw());
        state = state.update(main.input());
    }
}
