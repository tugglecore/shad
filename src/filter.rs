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
    cursor_position: [u16; 2],
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
        let splits = Layout::new(
            Direction::Vertical,
            [Constraint::Length(1), Constraint::Fill(1)],
        )
        .split(area);

        let label_text = "Filters";
        let label_widget =
            Text::from(label_text).style(Style::new().add_modifier(Modifier::UNDERLINED));

        let mut label_area = splits[0];
        label_area.width = label_text.len() as u16;

        frame.render_widget(label_widget, label_area);

        let filter_area = splits[1];

        if self.input.is_empty() {
            let help_text = "Press f to add filters";
            let help_widget = Text::from(help_text);
            frame.render_widget(help_widget, filter_area);

            return;
        }

        // let mut contents = String::new();
        // contents += PREAMBLE;

        // if self.input.is_empty() && !filtering {
        //     contents += "Press f to add filters"
        // } else {
        //     contents += &self.input
        // }
        // dbg!(area);
        //
        // let paragraph = Paragraph::new(contents.clone()).wrap(Wrap { trim: true });
        // frame.render_widget(paragraph, area);
        //
        // if filtering {
        //     frame.set_cursor_position(Position::new(
        //         self.cursor_position[0],
        //         self.cursor_position[1] + area.y,
        //     ));
        // }
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
                if self.cursor_position[0] <= (PREAMBLE.len() + self.input.len()) as u16 {
                    self.cursor_position[0] += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_position[0] > (PREAMBLE.len() + 1) as u16 {
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
