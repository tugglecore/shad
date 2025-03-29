use std::cmp::min;
use crate::filter::FilterBox;
use crate::sockets::SocketGrid;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState},
    DefaultTerminal, Frame,
};
use log::info;

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    table_state: TableState,
    current_mode: Mode,
    current_screen: SocketGrid,
    filter: FilterBox,
}

#[derive(Debug, Default)]
enum Mode {
    #[default]
    Browse,
    Filter,
    Sort,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        let mut me = Self::default();
        let sg = SocketGrid::new();
        me.current_screen = sg;
        me
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Max(5),
            Constraint::Min(3),
            Constraint::Length(3),
        ]);

        let rects = vertical.split(frame.area());

        self.render_header(frame, rects[0]);
        self.current_screen.draw(frame, rects[1]);
        self.render_footer(frame, rects[2]);
    }

    fn render_header(&mut self, frame: &mut Frame, mut area: Rect) {
        let horizontal = Layout::horizontal(
        [Constraint::Ratio(1, 3); 3]
        );

        let [filter_width, _, _] = horizontal.areas(area);

        let lines_for_filter = self.filter.count_lines(
            (filter_width.width - 1) as usize
        );
        info!("lines needed: {:#?}", lines_for_filter);
        area.height = min(
            area.height - 2, // 2 lines are required for top & bottom border
            lines_for_filter as u16
        );

        let block = Block::bordered();
        frame.render_widget(&block, area);


        let rects = horizontal.split(block.inner(area));

        let mut filter_area = rects[0];
        filter_area.height = 1;
        self.filter
            .draw(frame, filter_area, matches!(self.current_mode, Mode::Filter));

        let total_sockets = self.current_screen.sockets.len();
        let count = Paragraph::new(format!("Total: {}", total_sockets));

        let mut count_area = rects[1];
        count_area.height = 1;

        frame.render_widget(count, count_area);

        let sort = Paragraph::new("Sort: Press s to sort");

        let mut sort_area = rects[2];
        sort_area.height = 1;
        frame.render_widget(sort, sort_area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let (mode, controls) = match self.current_mode {
            Mode::Browse => ("BROWSING", "q quit | ↑ or k move up | ↓ or j move down"),
            Mode::Filter => (
                "FILTERING",
                "q quit filtering | type any text then press Enter to add filter",
            ),
            Mode::Sort => (
                "SORTING",
                "q quit sorting | → move right | ← move left | Enter toggle order",
            ),
        };

        let block = Block::bordered()
            .title(mode)
            .padding(ratatui::widgets::Padding::horizontal(10));

        let controls = Paragraph::new(controls).centered();

        let inner_area = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(controls, inner_area);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        let key_combo = (key.modifiers, key.code);

        let user_want_to_quit = matches!(
            key_combo,
            (_, KeyCode::Esc)
                | (
                    KeyModifiers::CONTROL,
                    KeyCode::Char('c') | KeyCode::Char('C')
                )
        );

        if user_want_to_quit {
            self.quit()
        }

        if matches!(key_combo, (_, KeyCode::Char('q'))) {
            if matches!(self.current_mode, Mode::Browse) {
                self.quit()
            } else {
                self.current_mode = Mode::Browse
            }
        }

        match self.current_mode {
            Mode::Browse => match key_combo {
                (_, KeyCode::Char('f')) => self.current_mode = Mode::Filter,
                (_, KeyCode::Char('s')) => self.current_mode = Mode::Sort,
                _ => {}
            },
            Mode::Filter => self.filter.on_key_event(key),
            Mode::Sort => match key_combo {
                _ => {}
            },
        }

        self.current_screen.on_key_event(key);
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
