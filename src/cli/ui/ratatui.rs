use crate::cli::{
    ui::monitor::{
        corpus::CorpusWatcher,
        logs::{
            AFLDashboard,
            AFLProperties,
        },
    },
    ziggy::ZiggyConfig,
};
use crossterm::{
    event,
    event::Event,
};
use ratatui::{
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
    text::{
        Line,
        Span,
        Text,
    },
    widgets::{
        Axis,
        Block,
        Borders,
        Chart,
        Dataset,
        Gauge,
        LineGauge,
        List,
        ListItem,
        Paragraph,
        Sparkline,
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
                    Constraint::Length(8),
                    Constraint::Min(8),
                ]
                .as_ref(),
            )
            .split(f.area());

        self.clone().render_title(f, chunks[0]);
        self.clone().render_stats(f, chunks[1]);
        self.clone().render_chart_and_config(f, chunks[2]);
    }

    fn render_chart_and_config(self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        self.clone().render_chart(f, chunks[0]);
        self.ziggy_config.config.render_config(f, chunks[1]);
        // self.render_config(f, chunks[1]);
    }

    // fn render_config(&self, f: &mut Frame, area: Rect) {
    //     let item = ListItem::new(Line::from(vec![
    //         Span::raw("Cores: "),
    //         Span::styled(
    //             format!("{:?}", self.ziggy_config.config.cores),
    //             Style::default().fg(Color::Yellow),
    //         ),
    //     ]));
    //
    //     let item2 = ListItem::new(Line::from(vec![
    //         Span::raw("Use Honggfuzz: "),
    //         Span::styled(
    //             format!("{}", self.ziggy_config.config.use_honggfuzz),
    //             Style::default().fg(Color::Yellow),
    //         ),
    //     ]));
    //     let items: Vec<ListItem> = vec![item, item2];
    //
    //     let config_list = List::new(items)
    //         .block(
    //             Block::default()
    //                 .borders(Borders::ALL)
    //                 .title("Configuration"),
    //         )
    //         .style(Style::default().fg(Color::White))
    //         .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    //         .highlight_symbol("> ");
    //
    //     f.render_widget(config_list, area);
    // }
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
            .alignment(ratatui::layout::Alignment::Center);

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
            .alignment(ratatui::layout::Alignment::Center);
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

        let right_stats = self.clone().metrics_right(&data);
        self.stats_left(f, data.borrow(), left_chunks[0]);
        f.render_widget(right_stats, chunks[1]);
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

        // Create the gauge for stability
        let label = format!("Stability: {:.2}%", data.stability * 100.0);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::DarkGray).bg(Color::White))
            .use_unicode(true)
            .label(label)
            .ratio(data.stability);

        frame.render_widget(gauge, chunks[1]);
    }

    fn metrics_right(self, data: &AFLProperties) -> Paragraph {
        Paragraph::new(Vec::from([
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
                    format!("{} exec/sec", data.exec_speed),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Metrics"))
    }

    fn render_chart(mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(area);

        let corpus_counter: &[(f64, f64)] = &self.corpus_watcher.as_tuple_slice();

        // println!("{:?}", corpus_counter);

        let dataset = vec![Dataset::default()
            .name("Executions")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(corpus_counter)];

        let chart = Chart::new(dataset)
            .block(
                Block::default()
                    .title("Execution Speed Over Time")
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray)),
            )
            .y_axis(
                Axis::default()
                    .title("Executions/sec")
                    .style(Style::default().fg(Color::Gray)),
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
