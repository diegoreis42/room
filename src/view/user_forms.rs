use std::{
  io,
  time::{Duration},
};
use crossterm::event::{self, Event, KeyCode};

use ratatui::{
  DefaultTerminal,
  Frame,
  style::{Color, Style},
  widgets::{Block, Borders},
};

use crate::app::app::{App, AppState};

use super::utils::centered_rect;

#[derive(Default)]
pub struct UserForms {}

impl UserForms {
  pub fn run(&mut self, app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {


      while app.state == AppState::EDITING  {
          terminal.draw(|frame| self.draw(frame))?;

          if event::poll(Duration::from_millis(1))? {
            self.handle_event(app)?;
        }
        }

      Ok(())
}

fn handle_event(&mut self, app: &mut App) -> io::Result<()> {
  if let Event::Key(key) = event::read()? {
      if key.code == KeyCode::Char('q') {
          app.state = AppState::IDLE;
      }
  }
  Ok(())
}
  fn draw(&self, frame: &mut Frame) {
    let popup_block = Block::default()
    .title("Edit your profile")
    .borders(Borders::NONE)
    .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(popup_block, area);
  }
}
