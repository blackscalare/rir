use std::collections::VecDeque;

use hud::HUD;
use raylib::{
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{config::get_config, constants, input_handler::InputEvent};

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
                        //self.action = Action::SelectHotbar;
                        self.select_hotbar(event);
                    }
                }

                InputEvent::E => {
                    self.action = Action::UseHotbar;
                }

                _ => {}
            }

            match self.action {
                Action::MoveCursor => {
                    self.move_cursor(event);
                    self.action = Action::None;
                }

                Action::SelectHotbar => {}

                Action::None => {}

                Action::UseHotbar => {}

                Action::SelectMenuItem => {}
            }
        }
    }

    fn select_hotbar(&mut self, event: &mut InputEvent) {
        match event {
            InputEvent::Key1 => {
                self.hud.hotbar.selected_rect = 1;
            }
            InputEvent::Key2 => {
                self.hud.hotbar.selected_rect = 2;
            }
            InputEvent::Key3 => {
                self.hud.hotbar.selected_rect = 3;
            }
            InputEvent::Key4 => {
                self.hud.hotbar.selected_rect = 4;
            }
            InputEvent::Key5 => {
                self.hud.hotbar.selected_rect = 5;
            }
            _ => {}
        }
        println!("Selected hotbar {:?}", event);
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

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        let total_width: i32 = (self.hud.hotbar.size * (50 + 10) - 10) as i32;
        for i in 0..self.hud.hotbar.size {
            let start_x = (constants::sizes::WINDOW_WIDTH - total_width) / 2;
            let x_pos: i32 = start_x + i as i32 * (50 + 10);
            let y_pos = constants::sizes::WINDOW_HEIGHT - 60;
            if self.hud.hotbar.selected_rect - 1 == i {
                draw_handle.draw_rectangle_lines(x_pos, y_pos, 50, 50, Color::RED);
            } else {
                draw_handle.draw_rectangle_lines(x_pos, y_pos, 50, 50, Color::BLACK);
            }

            draw_handle.draw_text(
                (i + 1).to_string().as_str(),
                x_pos + 10,
                y_pos + 10,
                8,
                Color::BLACK,
            );
        }
    }
}
