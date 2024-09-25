use crate::cli::ui::logs::{
    AFLDashboard,
    AFLProperties,
};
use crossterm::{
    event,
    event::Event,
};
use ratatui::{
    backend::Backend,
    layout::{
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Color,
        Modifier,
        Style,
    },
    symbols,
    widgets::{
        Axis,
        Block,
        Borders,
        Chart,
        Dataset,
        Paragraph,
    },
    Frame,
};
use std::{
    path::PathBuf,
    time::Duration,
};

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

    fn ui(self, f: &mut Frame, data: AFLProperties) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(8),
                    Constraint::Min(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        self.clone().render_title(f, chunks[0]);
        self.clone().render_stats(f, chunks[1], data);
        self.render_chart(f, chunks[2]);
    }

    fn render_title(self, f: &mut Frame, area: Rect) {
        let title = Paragraph::new("Fuzzing Dashboard")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_stats(self, f: &mut Frame, area: Rect, data: AFLProperties) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        let left_stats = Paragraph::new(format!(
            "Run Time: {}\nLast New Find: {}\nLast Saved Crash: {}",
            data.run_time, data.last_new_find, data.last_saved_crash
        ))
        .block(Block::default().borders(Borders::ALL).title("Stats"));

        let right_stats = Paragraph::new(format!(
            "Corpus Count: {}\nSaved Crashes: {}\nExec Speed: {} exec/sec",
            data.corpus_count, data.saved_crashes, data.exec_speed
        ))
        .block(Block::default().borders(Borders::ALL).title("Metrics"));

        f.render_widget(left_stats, chunks[0]);
        f.render_widget(right_stats, chunks[1]);
    }

    fn render_chart(self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(area);

        let chart_data = vec![
            (0.0, 5.0),
            (1.0, 6.0),
            (2.0, 7.0),
            (3.0, 8.0),
            (4.0, 4.0),
            (5.0, 3.0),
            (6.0, 5.0),
        ];

        let dataset = vec![Dataset::default()
            .name("Executions")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&chart_data)];

        let chart = Chart::new(dataset)
            .block(
                Block::default()
                    .title("Execution Speed Over Time")
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 6.0]),
            )
            .y_axis(
                Axis::default()
                    .title("Executions/sec")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([0.0, 10.0]),
            );

        f.render_widget(chart, chunks[0]);

        let seed_info =
            Paragraph::new(format!("Current Seed: {}", "currentseed")) // todo
                .block(Block::default().borders(Borders::ALL).title("Current Seed"));

        f.render_widget(seed_info, chunks[1]);
    }
    pub fn initialize_tui(&self) -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        terminal.clear()?;

        loop {
            terminal.draw(|f| {
                self.clone()
                    .ui(f, self.afl_dashboard.read_properties().unwrap())
            })?;

            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = crossterm::event::read()? {
                    if key.kind == event::KeyEventKind::Press
                        && key.code == event::KeyCode::Char('q')
                    {
                        break;
                    }
                }
            }
        }

        terminal.clear()?;
        Ok(())
    }
}
