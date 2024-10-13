use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use super::app::App;

#[derive(Debug, Default)]
pub struct Banner {
    exit: bool,
}

impl Banner {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(terminal)?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event, terminal);
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, terminal: &mut DefaultTerminal) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => App::default().start_app(terminal),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &Banner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![" (Q)uit ".into(), " (R)un ".into()]));

        let block = Block::bordered()
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        Paragraph::new(BANNER.to_string())
            .centered()
            .block(block)
            .render(area, buf);
    }
}

const BANNER: &str = "

██████╗  ██████╗  ██████╗ ███╗   ███╗
██╔══██╗██╔═══██╗██╔═══██╗████╗ ████║
██████╔╝██║   ██║██║   ██║██╔████╔██║
██╔══██╗██║   ██║██║   ██║██║╚██╔╝██║
██║  ██║╚██████╔╝╚██████╔╝██║ ╚═╝ ██║
╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝     ╚═╝
                                     

Welcome to room, here you can chat and see your friends 
without leaving the terminal!
";
