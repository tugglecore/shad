use crate::reader::{read_sockets, Socket};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct SocketGrid {
    // TODO: Refactor the visibility of this attribute.
    // We only need it public to allow the renderer to
    // get the count of sockets.
    pub sockets: Vec<Socket>,
    table_state: TableState,
}

impl SocketGrid {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut me = Self::default();
        me.read_sockets();
        me
    }
}

impl SocketGrid {
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        use Constraint::Ratio;

        let amount_of_columns = 5;

        let frame_width = (((1f64 / amount_of_columns as f64) * area.width as f64).floor()) as u64;

        let rows = self.sockets.iter().map(|socket| {
            // Here, we are working with Rows
            let mut height = 1usize;
            let row = vec![
                socket.process_id.clone(),
                socket.protocol.clone(),
                socket.local_address.clone(),
                socket.remote_address.clone(),
                format!("{}", 1),
            ]
            .into_iter()
            .map(|attribute| {
                let attribute = attribute
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(frame_width as usize)
                    .map(String::from_iter)
                    .collect::<Vec<_>>();
                height = attribute.len();
                let mut attribute = attribute.join("\n");

                attribute.insert(0, '\n');
                attribute.push('\n');
                attribute
            })
            .collect::<Vec<_>>();
            Row::new(row).height((height * 2) as u16)
        });

        let headers = Row::new([
            "PID",
            "Protocol",
            "Local Address",
            "Foreign Address",
            "State",
        ]);

        let widths = vec![Ratio(1, amount_of_columns); amount_of_columns as usize];

        let table = Table::new(rows, widths);
        let table = table.header(headers).column_spacing(7);
        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Down) | (_, KeyCode::Char('j')) => {
                let top_row_index = self.table_state.offset().saturating_add(1);
                *self.table_state.offset_mut() = std::cmp::min(self.sockets.len(), top_row_index);
            }
            (_, KeyCode::Up) | (_, KeyCode::Char('k')) => {
                let top_row_index = self.table_state.offset().saturating_sub(1);
                *self.table_state.offset_mut() = std::cmp::max(0, top_row_index);
            }
            // Add other key handlers here.
            _ => {}
        }
    }

    pub fn read_sockets(&mut self) {
        self.sockets = read_sockets();
    }
}
