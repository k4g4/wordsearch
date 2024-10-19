// #![windows_subsystem = "windows"]

use std::ops::Deref;

use raylib::prelude::*;

const FONT: &[u8] = include_bytes!("../font.ttf");
const FONT_COUNT: usize = 3;
const FONT_SIZES: [usize; FONT_COUNT] = [30, 45, 60];

type Fonts = [Font; FONT_COUNT];

enum State {
    Init,
    Searching,
}

impl State {
    fn toggle(&mut self) {
        match self {
            State::Init => *self = State::Searching,
            State::Searching => *self = State::Init,
        }
    }
}

struct Draw<'handle> {
    handle: RaylibDrawHandle<'handle>,
    fonts: &'handle Fonts,
}

impl Draw<'_> {
    fn small(&mut self, text: &str, position: impl Into<Vector2>, color: Color) {
        self.handle.draw_text_ex(
            &self.fonts[0],
            text,
            position.into(),
            FONT_SIZES[0] as _,
            2.0,
            color,
        );
    }

    fn medium(&mut self, text: &str, position: impl Into<Vector2>, color: Color) {
        self.handle.draw_text_ex(
            &self.fonts[1],
            text,
            position.into(),
            FONT_SIZES[1] as _,
            2.0,
            color,
        );
    }

    fn large(&mut self, text: &str, position: impl Into<Vector2>, color: Color) {
        self.handle.draw_text_ex(
            &self.fonts[2],
            text,
            position.into(),
            FONT_SIZES[2] as _,
            2.0,
            color,
        );
    }
}

struct Handle {
    handle: RaylibHandle,
    thread: RaylibThread,
    fonts: Fonts,
    state: State,
}

impl Handle {
    fn draw(&mut self) -> Draw {
        let mut handle = self.handle.begin_drawing(&self.thread);
        handle.clear_background(Color::WHITE);

        Draw {
            handle,
            fonts: &self.fonts,
        }
    }
}

impl Deref for Handle {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

fn main() {
    let mut handle = {
        let (mut handle, thread) = init().size(600, 400).title("Word Search").build();
        let fonts = FONT_SIZES.map(|font_size| {
            handle
                .load_font_from_memory(&thread, ".ttf", FONT, font_size as _, None)
                .unwrap()
        });

        Handle {
            handle,
            thread,
            fonts,
            state: State::Init,
        }
    };

    while !handle.window_should_close() {
        match handle.state {
            State::Init => {
                let mut d = handle.draw();

                d.medium("Press ENTER to begin", (10.0, 10.0), Color::BLACK);
            }
            State::Searching => {
                let mut d = handle.draw();

                d.small("Small...", (10.0, 10.0), Color::BLACK);
                d.large("Large...", (10.0, 20.0), Color::BLACK);
            }
        }

        if handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
            handle.state.toggle();
        }
    }
}
