use crossterm::event::{self, Event, KeyCode};
use opencv::videoio::{self, VideoCapture, VideoCaptureTrait};
use ratatui::{layout::Alignment, widgets::Paragraph, DefaultTerminal, Frame};
use std::{
    io,
    time::{Duration, Instant},
};

use super::ascii_processor::AsciiProcessor;

pub struct App {
    frame: String,
    exit: bool,
    camera: VideoCapture,
    ascii_processor: AsciiProcessor,
    last_update: Instant,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let camera = VideoCapture::new(0, videoio::CAP_ANY)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let ascii_processor = AsciiProcessor::new(80, 40); 

        Ok(Self {
            frame: String::new(),
            exit: false,
            camera,
            ascii_processor,
            last_update: Instant::now(),
        })
    }

    pub fn start_app(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.clear()?;
        while !self.exit {
            if self.last_update.elapsed() >= Duration::from_millis(33) {
                self.update_frame()?;
                self.last_update = Instant::now();
            }

            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(Duration::from_millis(1))? {
                self.handle_event()?;
            }
        }
        Ok(())
    }

    fn update_frame(&mut self) -> io::Result<()> {
        self.camera
            .read(&mut self.ascii_processor.frame)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        self.frame = self.ascii_processor.process();
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let paragraph = Paragraph::new(self.frame.clone()).alignment(Alignment::Center);
        frame.render_widget(paragraph, frame.area());
    }

    fn handle_event(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                self.exit = true;
            }
        }
        Ok(())
    }
}
