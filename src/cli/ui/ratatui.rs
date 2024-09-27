use crate::cli::{
    ui::{
        chart::ChartManager,
        monitor::{
            corpus::CorpusWatcher,
            logs::{
                AFLDashboard,
                AFLProperties,
            },
        },
        traits::Paint,
    },
    ziggy::ZiggyConfig,
};
use ratatui::{
    crossterm::event::{
        self,
        Event,
    },
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Color,
        Modifier,
        Style,
        Stylize,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Block,
        Borders,
        Gauge,
        Paragraph,
    },
    Frame,
};
use std::{
    borrow::Borrow,
    thread,
    time::Duration,
};

#[derive(Clone, Debug)]
pub struct CustomUI {
    ziggy_config: ZiggyConfig,
    afl_dashboard: AFLDashboard,
    corpus_watcher: CorpusWatcher,
}

impl CustomUI {
    const REFRESH_MS: u64 = 500;
    pub fn new(ziggy_config: &ZiggyConfig) -> anyhow::Result<CustomUI> {
        let output = ziggy_config.clone().fuzz_output();
        Ok(Self {
            ziggy_config: ziggy_config.clone(),
            afl_dashboard: AFLDashboard::from_output(output.clone())?,
            corpus_watcher: CorpusWatcher::from_output(output)?,
        })
    }

    fn ui(self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Percentage(20),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(f.area());

        self.clone().render_title(f, chunks[0]);
        self.clone().render_stats(f, chunks[1]);
        self.clone().render_chart_and_config(f, chunks[2]);
        self.clone().render_seed(f, chunks[3]);
    }

    fn render_chart_and_config(self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        self.clone().render_chart(f, chunks[0]);
        self.ziggy_config.config.render(f, chunks[1]);
    }

    fn render_octopus(self, f: &mut Frame, area: Rect) {
        let ascii_art = r#"
,---.
( @ @ )
 ).-.(
'/|||\`
  '|`
  "#;

        let octopus = Paragraph::new(ascii_art)
            .style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);

        f.render_widget(octopus, area);
    }
    fn render_title(self, f: &mut Frame, area: Rect) {
        self.render_octopus(f, area);
        let title = Paragraph::new("Phink Fuzzing Dashboard")
            .style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_stats(self, f: &mut Frame, area: Rect) {
        let data = self.afl_dashboard.read_properties().unwrap();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0)].as_ref())
            .split(chunks[0]);

        let right_chunk = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0)].as_ref())
            .split(chunks[1]);

        self.stats_left(f, data.borrow(), left_chunks[0]);
        self.metrics_right(f, data.borrow(), right_chunk[0]);
    }

    fn stats_left(&self, frame: &mut Frame, data: &AFLProperties, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(70), Constraint::Min(1)].as_ref())
            .split(area);

        let paragraph = Paragraph::new(Vec::from([
            Line::from(vec![
                Span::raw("Running for: "),
                Span::styled(
                    data.run_time.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("Last new find: "),
                Span::styled(
                    data.last_new_find.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("Last saved crash: "),
                Span::styled(
                    data.last_saved_crash.clone(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Statistics"));

        frame.render_widget(paragraph, chunks[0]);

        self.create_stability_display(frame, chunks[1], data);
    }

    fn create_stability_display(&self, frame: &mut Frame, area: Rect, data: &AFLProperties) {
        let label = format!("{:.2}%", data.stability * 100.0);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .use_unicode(true)
            .label(label)
            .bold()
            .ratio(data.stability);

        // Create a styled block with borders
        let block = Block::default()
            .title("System Stability")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightCyan).bg(Color::Black));

        // Create a paragraph with the gauge inside
        let paragraph = Paragraph::new(vec![Line::raw("Fuzzing stability")])
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true });

        // Render the paragraph with the gauge inside
        frame.render_widget(paragraph, area);
        frame.render_widget(gauge, area);
    }
    fn metrics_right(&self, frame: &mut Frame, data: &AFLProperties, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let paragraph = Paragraph::new(Vec::from([
            Line::from(vec![
                Span::raw("Corpus count: "),
                Span::styled(
                    data.corpus_count.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("Saved crashes: "),
                Span::styled(
                    data.saved_crashes.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("Execution speed: "),
                Span::styled(
                    format!("{} execs/sec", data.exec_speed),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Metrics"));

        frame.render_widget(paragraph, chunks[0]);
    }

    fn render_chart(mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let corpus_counter: &[(f64, f64)] = &self.corpus_watcher.as_tuple_slice();

        let chart_manager = ChartManager::new(corpus_counter);
        f.render_widget(chart_manager.create_chart(), chunks[0]);
    }

    fn render_seed(self, f: &mut Frame, area: Rect) {
        let seed_info =
            Paragraph::new(format!("Current Seed: {}", "currentseed")) // todo
                .block(Block::default().borders(Borders::ALL).title("Current Seed"));

        f.render_widget(seed_info, area);
    }
    pub fn initialize_tui(&self) -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        terminal.clear()?;

        loop {
            terminal.draw(|f| self.clone().ui(f))?;
            thread::sleep(Duration::from_millis(Self::REFRESH_MS));
            if event::poll(Duration::from_millis(Self::REFRESH_MS))? {
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
