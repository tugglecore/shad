use crate::sockets::SocketGrid;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, TableState},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    table_state: TableState,
    current_screen: SocketGrid,
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
        self.current_screen.draw(frame)
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
        match (key.modifiers, key.code) {
            // (_, KeyCode::Esc | KeyCode::Char('p')) =>
            //     self.current_screen = ProcessTable
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }

        self.current_screen.on_key_event(key);
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
