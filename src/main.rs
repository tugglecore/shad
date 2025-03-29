pub mod app;
mod filter;
mod reader;
pub mod sockets;

pub use app::App;

fn main() -> color_eyre::Result<()> {
    env_logger::init();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
