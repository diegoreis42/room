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

    use crate::app::app::AppState;

    #[derive(Default)]
    pub struct Banner {
        state: AppState,
    }

    impl Banner {
        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<AppState> {
            while self.state == AppState::IDLE {
                terminal.draw(|frame| self.draw(frame))?;
                self.state = self.handle_events()?;
            }

            Ok(self.state.clone())
        }

        fn draw(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.area());
        }

        fn handle_events(&self) -> io::Result<AppState> {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    return Ok(self.handle_key_event(key_event));
                }
            }

            Ok(AppState::IDLE)
        }

        fn handle_key_event(
            &self,
            key_event: KeyEvent,
        ) -> AppState {
            match key_event.code {
                KeyCode::Char('q') => AppState::DOWN,
                KeyCode::Char('e') => AppState::EDITING,
                KeyCode::Char('r') => AppState::RUNNING,
                _ => AppState::IDLE,
            }
        }
    }

    impl Widget for &Banner {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let instructions = Title::from(Line::from(vec![
                " (Q)uit ".into(),
                " (R)un ".into(),
                " (E)dit ".into(),
            ]));

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
