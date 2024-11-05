use std::io;

use ratatui::DefaultTerminal;

use crate::view::banner::Banner;

#[derive(Default)]
pub struct App {
    pub state: AppState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if let Err(e) = Banner::default().run(self, terminal) {
            println!("A error occurred when drawing the banner, {}", e);
        }

        Ok(())
    }
}
#[derive(PartialEq)]
pub enum AppState {
    EDITING,
    RUNNING,
    IDLE
}

impl Default for AppState {
    fn default() -> Self {
        AppState::IDLE
    }
}