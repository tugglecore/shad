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

        let label_text = "Filters";
        let label_widget =
            Text::from(label_text).style(Style::new().add_modifier(Modifier::UNDERLINED));

        let mut label_area = splits[0];
        label_area.width = label_text.len() as u16;

        frame.render_widget(label_widget, label_area);

        let filter_area = splits[1];

        if self.input.is_empty() && !currently_filtering {
            let help_text = "Press f to add filters";
            let help_widget = Text::from(help_text);
            frame.render_widget(help_widget, filter_area);

            return;
        }

        let mut lines = vec![];

        dbg!(&self);
        dbg!(&filter_area);

        dbg!(self.input.len());

        let mut lower_bound = 0;
        while lower_bound < self.input.len() {
            let filter_width = filter_area.width as usize;
            
            let mut upper_bound = lower_bound + (filter_width - 1);
            if upper_bound >= self.input.len() {
                upper_bound = self.input.len() - 1
            }
            // let upper_bound = if upper_bound < self.input.len() {
            //     position + filter_width
            // } else {
            //     position + self.input.len() - 1
            // };

            let filter_line = Line::from(
                &self.input[lower_bound..=upper_bound]
            );

            lines.push(filter_line);

            lower_bound = upper_bound + 1;
        }

        frame.render_widget(Text::from(lines), filter_area);

        frame.set_cursor_position(Position::new(
            filter_area.x + self.cursor_position[0],
            filter_area.y + self.cursor_position[1]
        ));
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
