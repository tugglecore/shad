use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use log::info;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState, Widget},
    DefaultTerminal, Frame,
};

const PREAMBLE: &str = "Filters: ";

#[derive(Debug, Default)]
struct Filter {
    token: String,
    attribute: String,
    is_complete: bool,
}

#[derive(Debug, Default)]
pub struct FilterBox {
    pub input: String,
    cursor_position: [u16; 2]
}

impl FilterBox {
    pub fn new() -> Self {
        Self {
            cursor_position: [(PREAMBLE.len() as u16) + 1, 0],
            ..Default::default()
        }
    }
}

impl FilterBox {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, filtering: bool) {
        let mut contents = String::new();
        contents += PREAMBLE;

        if self.input.is_empty() && !filtering {
            contents += "Press f to add filters"
        } else {
            contents += &self.input
        }

        frame.render_widget(Paragraph::new(contents.clone()), area);

        if filtering {
            frame.set_cursor_position(
                Position::new(
                    self.cursor_position[0],
                    self.cursor_position[1] + area.y
                )
            );
        }
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
                self.cursor_position[0] += 1;
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.cursor_position[0] -= 1;
            }
            KeyCode::Right => {
                dbg!(self.cursor_position);
                dbg!(self.input.len());
                if self.cursor_position[0] < (self.input.len() - 1) as u16 {
                    self.cursor_position[0] += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_position[0] > (PREAMBLE.len() +1) as u16 {
                    self.cursor_position[0] -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn count_lines(&self, max_line_width: usize) -> usize {
        ((PREAMBLE.len() + self.input.len()) / max_line_width) + 1
    }
}
