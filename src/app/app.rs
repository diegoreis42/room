use std::io;

use ratatui::DefaultTerminal;

use crate::view::banner::Banner;

pub struct App {}

impl App {
    pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
        if let Err(_) = Banner::default().run(terminal) {}

        Ok(())
    }
}
