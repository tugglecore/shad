use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState, Widget},
    DefaultTerminal, Frame,
};

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
        let mut prelim = String::from("Filters: ");

        if self.filters.is_empty() && !filtering {
            prelim += "Press f to add filters"
        }

        frame.render_widget(Paragraph::new(prelim.clone()), area);

        if filtering {
            frame.set_cursor_position(Position::new(area.x + (prelim.len() as u16), area.y));
        }
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                if self.filters.len() == 0 {
                    self.filters.push(Filter::default());
                    self.selected_filter = 0;
                }

                self.filters[self.selected_filter].token.push(c)
            }
            KeyCode::Enter => {
                self.filters[self.selected_filter].is_complete = true;
                self.selected_filter += 1;
            }
            _ => {}
        }
    }
}
