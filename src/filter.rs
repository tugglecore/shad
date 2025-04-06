use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use log::info;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState, Widget, Wrap},
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
    filter_input_area: Rect,
    cursor_position: u16,
}

impl FilterBox {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl FilterBox {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect, currently_filtering: bool) {
        let splits = Layout::new(
            Direction::Vertical,
            [Constraint::Length(1), Constraint::Fill(1)],
        )
        .split(area);
        let filter_area = splits[1];

        let label_text = "Filters: ";
        let input_splits = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(label_text.len() as u16),
                Constraint::Fill(1),
            ],
        )
        .split(splits[0]);

        self.filter_input_area = input_splits[1];

        let label_widget = Line::from(label_text);
        frame.render_widget(label_widget, input_splits[0]);

        if self.input.is_empty() && !currently_filtering {
            let help_text = "Press f to add filters";
            let help_widget = Text::from(help_text);
            frame.render_widget(help_widget, input_splits[1]);

            return;
        }

        let filter_input = if self.input.len() > input_splits[1].width as usize {
            let start_slice = self.input.len() - input_splits[1].width as usize;
            &self.input[start_slice..]
        } else {
            self.input.as_str()
        };

        let filter_input = Line::from(filter_input);
        frame.render_widget(filter_input, input_splits[1]);

        frame.set_cursor_position(Position::new(
            input_splits[1].x + self.cursor_position,
            input_splits[1].y
        ));
        
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.input.push(c);
                self.move_cursor_to_right();
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.cursor_position -= 1;
            }
            KeyCode::Right => self.move_cursor_to_right(),
            KeyCode::Left => self.move_cursor_to_left(),
            _ => {}
        }
    }

    fn move_cursor_to_left(&mut self) {
        info!("move the position to the left");

        let left_position = self.cursor_position - 1;

        dbg!(self.input.len());
        dbg!(left_position);

        if left_position as usize > self.input.len() { return }

    }

    fn move_cursor_to_right(&mut self) {
        let next_position = self.cursor_position + 1;

        if next_position as usize > self.input.len() { return }

        if self.cursor_position + 1 > self.filter_input_area.width {
            self.cursor_position = self.filter_input_area.width;
        } else {
            self.cursor_position += 1;
        }
    }

    pub fn count_lines(&self, max_line_width: usize) -> usize {
        ((PREAMBLE.len() + self.input.len()) / max_line_width) + 1
    }
}
