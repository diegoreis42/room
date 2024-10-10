use std::{io, thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use opencv::videoio::{self, VideoCaptureTrait};
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

use crate::ascii_processor::AsciiProcessor;

#[derive(Debug, Default)]
pub struct View {
    exit: bool,
}

impl View {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event);
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.start_app(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn start_app(&self) {
        let mut cam =
            videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Failed to get video capture");

        let mut ascii_processor = AsciiProcessor::new(200, 90);

        loop {
            if let Err(e) = cam.read(&mut ascii_processor.frame) {
                eprintln!("Failed to capture frame: {}", e);
                break;
            }

            ascii_processor.process();

            sleep(Duration::from_millis(33));

            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) = event::read().unwrap()
                {
                    break;
                }
            }
        }
    }
}

impl Widget for &View {
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
                                     
";
