use std::io;

use ratatui::DefaultTerminal;

use crate::view::{banner::Banner, user_forms::UserForms};

use super::camera_engine::CameraEngine;

#[derive(Default)]
pub struct App {
    pub state: AppState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match self.state {
                AppState::IDLE => self.show_banner(terminal)?,
                AppState::EDITING => self.run_user_forms(terminal)?,
                AppState::RUNNING => self.run_camera_engine(terminal)?,
                AppState::DOWN => break,
            }
        }

        Ok(())
    }

    fn show_banner(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let new_state = Banner::default().run(terminal)?;
        self.state = new_state;
        Ok(())
    }

    fn run_user_forms(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let new_state = UserForms::default().run(terminal)?;
        self.state = new_state;
        Ok(())
    }

    fn run_camera_engine(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let new_state = CameraEngine::new()?.start_app(terminal)?;
        self.state = new_state;
        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AppState {
    EDITING,
    RUNNING,
    IDLE,
    DOWN,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::IDLE
    }
}
