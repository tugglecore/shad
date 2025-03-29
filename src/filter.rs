use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState, Widget},
    DefaultTerminal, Frame,
};
use log::info;

#[derive(Debug, Default)]
struct Filter {
    token: String,
    attribute: String,
    is_complete: bool,
}

#[derive(Debug, Default)]
pub struct FilterBox {
    selected_filter: usize,
    pub filters: Vec<Filter>,
    pub input: String
}

impl FilterBox {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FilterBox {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, filtering: bool) {
        // println!("Height: {:#?}", area.height);
        let mut prelim = String::from("Filters: ");

        if self.input.is_empty() && !filtering {
            prelim += "Press f to add filters"
        } else {
            prelim += &self.input
        }

        frame.render_widget(Paragraph::new(prelim.clone()), area);

        if filtering {
            frame.set_cursor_position(Position::new(area.x + (prelim.len() as u16), area.y));
        }
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c)
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            _ => {}
        }
    }

    pub fn count_lines(&self, line_width: usize) -> usize {
        let r =  (line_width / std::cmp::max(1, self.input.len()));
        info!("math result: {r:#?}");

        r
    }
}
