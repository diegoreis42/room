use std::io;

use app::app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
mod app;
mod view;
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    let app_result = App::run(&mut terminal);

    ratatui::restore();
    disable_raw_mode()?;
    app_result
}
