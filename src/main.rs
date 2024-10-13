use std::io;

use view::banner::Banner;
mod view;
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = Banner::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
