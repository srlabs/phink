use crate::cli::ui::logs::AFLDashboard;
use crossterm::{
    event,
    event::Event,
};
use ratatui::{
    text::Text,
    Frame,
};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CustomUI {
    afl_dashboard: AFLDashboard,
}

impl CustomUI {
    pub fn new(output: PathBuf) -> anyhow::Result<CustomUI> {
        Ok(Self {
            afl_dashboard: AFLDashboard::from_output(output)?,
        })
    }
    pub fn initialize_tui(self) -> anyhow::Result<()> {
        let mut terminal = ratatui::init();
        loop {
            terminal
                .draw(|frame: &mut Frame| {
                    frame.render_widget(Text::raw("Hello World!"), frame.area())
                })
                .expect("Failed to draw");
            if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
                break;
            }
        }
        ratatui::restore();
        Ok(())
    }
}
