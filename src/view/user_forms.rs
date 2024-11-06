use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
  DefaultTerminal,
  Frame,
  style::{Color, Style},
  widgets::{Block, Borders},
};

use crate::app::app::AppState;

use super::utils::centered_rect;

pub struct UserForms {
  state: AppState,
}

impl UserForms {
  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {

      while self.state == AppState::EDITING  {
          terminal.draw(|frame| self.draw(frame))?;
          self.state = self.handle_events()?;
      }

      Ok(self.state.clone())
}

fn handle_events(&mut self) -> io::Result<AppState> {
  if let Event::Key(key_event) = event::read()? {
      if key_event.kind == KeyEventKind::Press {
          return Ok(self.handle_key_event(key_event));
      }
  }

  Ok(AppState::EDITING)
}

fn handle_key_event(&mut self, key_event: KeyEvent) -> AppState {
      match key_event.code {
          KeyCode::Char('q') => AppState::IDLE,
          _ => AppState::EDITING,
      }
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

impl Default for UserForms {
  fn default() -> Self {
      Self {
          state: AppState::EDITING,
      }
  }
}