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
        seed::SeedDisplayer,
        traits::{
            FromPath,
            Paint,
        },
    },
    ziggy::ZiggyConfig,
};
use anyhow::Context;
use crossterm::event::KeyCode;
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
    process::Child,
    sync::mpsc,
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
            afl_dashboard: AFLDashboard::from_output(output.clone())
                .context("Couldn't create AFL dashboard")?,
            corpus_watcher: CorpusWatcher::from_output(output)
                .context("Couldn't create the corpus watcher")?,
        })
    }

    fn ui(self, f: &mut Frame) -> anyhow::Result<()> {
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
        self.clone().render_seed(f, chunks[3])?;
        Ok(())
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
        let data = self.afl_dashboard.read_properties();

        if let Ok(afl) = data {
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

            self.stats_left(f, afl.borrow(), left_chunks[0]);
            self.metrics_right(f, afl.borrow(), right_chunk[0]);
        }
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

        let block = Block::default()
            .title("System Stability")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightCyan).bg(Color::Black));

        let paragraph = Paragraph::new(vec![Line::raw("Fuzzing stability")])
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(paragraph, area);
        frame.render_widget(gauge, area);
    }
    fn metrics_right(&self, frame: &mut Frame, data: &AFLProperties, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let crash_style = Self::if_crash(data);

        let paragraph = Paragraph::new(Vec::from([
            Line::from(vec![
                Span::raw("Corpus count: "),
                Span::styled(
                    data.corpus_count.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![Span::raw("Saved crashes: "), crash_style]),
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

    fn if_crash(data: &AFLProperties) -> Span {
        let crash_style = if data.saved_crashes > 0 {
            Span::styled(
                data.saved_crashes.to_string(),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
                    .fg(Color::Red),
            )
        } else {
            Span::styled(
                data.saved_crashes.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )
        };
        crash_style
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

    fn render_seed(self, f: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let seed_displayer = SeedDisplayer::new(self.ziggy_config.fuzz_output());

        let mut seed_info_text: String = String::new();
        if let Some(seeds) = seed_displayer.load() {
            seed_info_text = seeds
                .iter()
                .enumerate()
                .map(|(i, seed)| format!("Seed {}: {}", i + 1, seed))
                .collect::<Vec<String>>()
                .join("\n");
        }

        let seed_info = Paragraph::new(seed_info_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Last fuzzed messages"),
        );

        f.render_widget(seed_info, area);

        Ok(())
    }
    pub fn initialize_tui(&self, mut child: Child) -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        terminal.clear()?;

        let (tx, rx) = mpsc::channel();

        let input_handle = thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        // If we CTRL+C, we send the exit signal
                        if key.code == KeyCode::Char('c')
                            && key.modifiers.contains(event::KeyModifiers::CONTROL)
                        {
                            tx.send(()).unwrap();
                            break;
                        }
                    }
                }
            }
        });

        loop {
            if rx.try_recv().is_ok() {
                break;
            }
            terminal.draw(|f| {
                if let Err(err) = self.clone().ui(f) {
                    eprintln!("{:?}", err);
                }
            })?;
            thread::sleep(Duration::from_millis(Self::REFRESH_MS));
        }

        let _ = child.kill();
        input_handle.join().unwrap();

        terminal.clear()?;
        Ok(())
    }
}
