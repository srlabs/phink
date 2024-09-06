use crossterm::{
    event,
    event::Event,
};
use ratatui::{
    text::Text,
    Frame,
};

pub fn initialize_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| frame.render_widget(Text::raw("Hello World!"), frame.area()))
            .expect("Failed to draw");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
    Ok(())
}
