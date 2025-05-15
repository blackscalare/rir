use std::collections::VecDeque;

use hud::HUD;
use raylib::prelude::RaylibDrawHandle;

use crate::input_handler::InputEvent;

mod hud;

#[derive(PartialEq)]
enum State {
    Start,
    Options,
    Main,
    Inventory,
}

enum Action {
    SelectHotbar,
    MoveCursor,
    UseHotbar,
    SelectMenuItem,
    None,
}

pub struct GUI {
    hud: HUD,
    state: State,
    action: Action,
}

impl GUI {
    pub fn new() -> GUI {
        GUI {
            hud: HUD::new(),
            state: State::Main,
            action: Action::None,
        }
    }

    pub fn update(&mut self, input_events: &mut VecDeque<InputEvent>) {
        for event in input_events {
            match event {
                InputEvent::Up | InputEvent::Down | InputEvent::Left | InputEvent::Right => {
                    self.action = Action::MoveCursor;
                }

                InputEvent::MouseLeft | InputEvent::MouseRight | InputEvent::Enter => {
                    if self.state == State::Main {
                        self.action = Action::SelectHotbar;
                        // TODO UseHotbar?
                    } else if self.state == State::Start
                        || self.state == State::Options
                        || self.state == State::Inventory
                    {
                        self.action = Action::SelectMenuItem;
                    }
                }

                InputEvent::Key1
                | InputEvent::Key2
                | InputEvent::Key3
                | InputEvent::Key4
                | InputEvent::Key5 => {
                    if self.state == State::Main {
                        self.action = Action::SelectHotbar;
                    }
                }

                InputEvent::E => {
                    self.action = Action::UseHotbar;
                }
            }

            match self.action {
                Action::MoveCursor => {
                    self.move_cursor(event);
                }

                Action::SelectHotbar => {}

                Action::None => {}

                Action::UseHotbar => {}

                Action::SelectMenuItem => {}
            }
        }
    }

    fn move_cursor(&mut self, event: &mut InputEvent) {
        match event {
            InputEvent::Up => {}

            InputEvent::Down => {}

            InputEvent::Left => {}

            InputEvent::Right => {}

            _ => {}
        }
    }

    pub fn draw(&self, draw_handle: &RaylibDrawHandle) {
        //draw_handle.draw_rectangle_rec(rec, color);
    }
}
