use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame};

use crate::app::app::AppState;

use super::utils::centered_rect;




pub struct UserForms {
    nick_name: String,
    input_mode: InputMode,
    state: AppState,
}
#[derive(PartialEq)]
enum InputMode {
    NickName,
}

impl Default for UserForms {
    fn default() -> Self {
        UserForms {
            nick_name: String::new(),
            input_mode: InputMode::NickName,
            state: AppState::EDITING,
        }
    }
}


impl UserForms {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {
        while self.state == AppState::EDITING {
            terminal.draw(|frame| self.draw(frame))?;
            self.state = self.handle_events()?;
        }
        Ok(self.state.clone())
    }

    fn handle_events(&mut self) -> io::Result<AppState> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
            }
        }
        Ok(self.state.clone())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.state = AppState::IDLE,
            KeyCode::Enter => self.switch_input_mode(),
            KeyCode::Char(c) => self.handle_character_input(c),
            KeyCode::Backspace => self.handle_backspace(),
            _ => {}
        }
    }

    fn switch_input_mode(&mut self) {
        match self.input_mode {
            InputMode::NickName => self.state = AppState::IDLE,
        }
    }

    fn handle_character_input(&mut self, c: char) {
        match self.input_mode {
            InputMode::NickName => self.nick_name.push(c),
        }
    }

    fn handle_backspace(&mut self) {
        match self.input_mode {
            InputMode::NickName => self.nick_name.pop(),
        };
    }

    fn draw(&self, frame: &mut Frame) {
        let area = centered_rect(60, 25, frame.area());

        let block = Block::default()
            .title("User Infos")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::DarkGray));
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(area);

        let nickname_label = Paragraph::new("Nickname:")
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(nickname_label, chunks[0]);

        let nickname_input = Paragraph::new(self.nick_name.clone())
            .style(
                Style::default().fg(if self.input_mode == InputMode::NickName {
                    Color::Yellow
                } else {
                    Color::White
                }),
            )
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(nickname_input, chunks[1]);
    }
}