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
use windows::Win32::NetworkManagement::IpHelper::{GetTcpTable2, MIB_TCPROW2, MIB_TCPTABLE2};

#[derive(Debug, Default)]
pub struct SocketGrid {
    sockets: Vec<Vec<String>>,
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
    pub fn draw(&mut self, frame: &mut Frame) {
        use Constraint::Ratio;

        let amount_of_columns = 5;

        let frame_width =
            (((1f64 / amount_of_columns as f64) * frame.area().width as f64).floor()) as u64;

        let rows = self
            .sockets
            .iter()
            .map(|socket| {
                // Here, we are working with Rows
                let mut height = 1;

                let socket = socket
                    .iter()
                    .map(|cell| {
                        // Now we are working with Cells
                        if cell.len() as u64 <= frame_width {
                            return cell.clone();
                        }

                        // Here, we are proccessing an indiv cell
                        // to determine the amount of lines a cell
                        // will need to be properly rendered
                        let cell = cell
                            .chars()
                            .collect::<Vec<_>>()
                            .chunks(frame_width as usize)
                            .map(String::from_iter)
                            .collect::<Vec<_>>();

                        height = cell.len();
                        cell.join("\n")
                    })
                    .collect::<Vec<_>>();

                Row::new(socket).height((height as u16) * 2)
            })
            .collect::<Vec<_>>();

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
        // frame.render_stateful_widget(table, frame.area(), &mut self.table_state);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Down) => {
                let top_row_index = self.table_state.offset().saturating_add(1);
                *self.table_state.offset_mut() = std::cmp::min(self.sockets.len(), top_row_index);
            }
            (_, KeyCode::Up) => {
                let top_row_index = self.table_state.offset().saturating_sub(1);
                *self.table_state.offset_mut() = std::cmp::max(0, top_row_index);
            }
            // Add other key handlers here.
            _ => {}
        }
    }

    pub fn read_sockets(&mut self) {
        let mut size = u32::default();
        unsafe {
            let b = GetTcpTable2(Some(std::ptr::null_mut()), &mut size, false);
            println!("return status of first call: {}", b);
        }

        let mut buffer = Vec::<MIB_TCPTABLE2>::with_capacity(size as usize);
        // let mut table = MIB_TCPTABLE2 {
        println!("Buffer: {:#?}", buffer);
        println!("Capacity: {:#?}", buffer.capacity());
        println!("Sizepointer before second call: {size:#?}");
        unsafe {
            let a = GetTcpTable2(
                Some(buffer.as_mut_ptr()), 
                &mut size, true);
            println!("return status of second call: {}", a);
        }
        println!("Buffer: {buffer:#?}");
        println!("Sizepoint after second call: {size:#?}");
        unsafe {
            let a = (*buffer.as_mut_ptr().cast::<MIB_TCPTABLE2>()).dwNumEntries;
            println!("What am I doning: {}", a);
            let a = *(*buffer.as_mut_ptr()).table.as_ptr();
            println!("What am I doning: {:#?}", a);
            let a = *(*buffer.as_mut_ptr()).table.as_ptr().add(1);
            println!("What am I doning: {:#?}", a);
            let a = *(*buffer.as_mut_ptr()).table.as_ptr().add(2);
            println!("What am I doning: {:#?}", a);
        };

        let mut cells = vec![
            String::from("1-2-3-4-5-6-7-8-9-a-b-c-defeghijklmnC1sasfasfasfas sfs fas f"),
            String::from("C2"),
            String::from("C3"),
            String::from("C4"),
            String::from("C5"),
        ];

        let mut c = std::array::from_fn::<_, 95, _>(|i| format!("C{}", i + 1))
            .chunks(5)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();

        c.push(cells.clone());
        c.push(cells.clone());
        c.push(cells.clone());

        self.sockets = c;
    }
}
