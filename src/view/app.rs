use std::{io, thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use opencv::videoio::{self, VideoCaptureTrait};
use ratatui::{
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame,
};

use super::ascii_processor::AsciiProcessor;

#[derive(Default)]
pub struct App {
    frame: String,
    exit: bool,
}

impl App {
    pub fn start_app(&mut self, terminal: &mut DefaultTerminal) {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame)).unwrap();
            self.handle_events(terminal);
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let mut cam =
            videoio::VideoCapture::new(0, videoio::CAP_ANY).expect("Failed to get video capture");

        let mut ascii_processor = AsciiProcessor::new(100, 100);

        loop {
            if let Err(e) = cam.read(&mut ascii_processor.frame) {
                eprintln!("Failed to capture frame: {}", e);
                break;
            }

            self.frame = ascii_processor.process();
            frame.render_widget(self.clone(), frame.area());

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

    fn handle_events(&mut self, terminal: &mut DefaultTerminal) {
        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Press {
                self.handle_key_event(key_event, terminal);
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent, terminal: &mut DefaultTerminal) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.frame).centered().render(area, buf);
    }
}

impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            frame: self.frame.clone(),
            exit: self.exit.clone(),
        }
    }
}
