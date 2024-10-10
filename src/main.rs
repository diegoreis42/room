mod ascii_processor;
mod view;

use std::io;

use view::View;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = View::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
