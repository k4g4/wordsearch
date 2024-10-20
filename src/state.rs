use crate::{common::Input, drawing::Draw};
use raylib::prelude::*;

#[derive(Clone, Default, Debug)]
pub enum State {
    #[default]
    Init,
    Searching,
}

impl State {
    pub fn draw<'handle>(&self, draw: Draw<'handle>) -> Draw<'handle> {
        let small = format!("{}", draw.get_screen_height());
        match self {
            State::Init => draw.medium("Press ENTER to begin", (10.0, 10.0), Color::BLACK),
            State::Searching => draw.small(&small, (10.0, 10.0), Color::BLACK).large(
                "Large...",
                (10.0, 20.0),
                Color::BLACK,
            ),
        }
    }

    pub fn update(&mut self, Input { pos, clicked, keys }: Input) {
        match self {
            State::Init => {
                if clicked || keys.contains(&KeyboardKey::KEY_ENTER) {
                    *self = State::Searching;
                }
            }
            State::Searching => {
                if clicked || keys.contains(&KeyboardKey::KEY_ENTER) {
                    *self = State::Init;
                }
            }
        }
    }
}
