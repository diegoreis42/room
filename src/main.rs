use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use view::banner::Banner;
mod view;
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    let app_result = Banner::default().run(&mut terminal);

    ratatui::restore();
    disable_raw_mode()?;
    app_result
}
