pub mod app;
mod reader;
pub mod sockets;

pub use app::App;
use reader::read_sockets;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
