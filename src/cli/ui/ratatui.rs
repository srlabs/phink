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
        Margin,
        Rect,
    },
    style::{
        Color,
        Modifier,
        Style,
        Stylize,
    },
    symbols,
    text::{
        Line,
        Span,
        Text,
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
        Sparkline,
    },
    Frame,
};
use std::{
    borrow::Borrow,
    process::Child,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
    },
    time::Duration,
};

#[derive(Clone, Debug)]
pub struct CustomUI {
    ziggy_config: ZiggyConfig,
    afl_dashboard: AFLDashboard,
    corpus_watcher: CorpusWatcher,
    fuzzing_speed: Vec<u64>,
    seed_text: Text<'static>,
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
            fuzzing_speed: Vec::new(),
            seed_text: Text::from(""),
        })
    }

    fn ui(&mut self, f: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(7),
                    Constraint::Percentage(20),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ]
                .as_ref(),
            )
            .split(f.area());

        self.render_title(f, chunks[0]);
        self.render_stats(f, chunks[1]);
        self.render_chart_and_config(f, chunks[2]);
        self.render_bottom(f, chunks[3])?;
        Ok(())
    }

    fn render_chart_and_config(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        self.render_chart(f, chunks[0]);
        self.ziggy_config.config.render(f, chunks[1]);
    }

    fn render_octopus(&self, f: &mut Frame, area: Rect) {
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
    fn render_title(&self, f: &mut Frame, area: Rect) {
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

    fn render_stats(&mut self, f: &mut Frame, area: Rect) {
        let data = self.afl_dashboard.read_properties();

        if let Ok(afl) = data {
            self.fuzzing_speed.push(afl.exec_speed.into());

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
            self.speed_right(f, right_chunk[0]);
        }
    }

    fn stats_left(&self, frame: &mut Frame, data: &AFLProperties, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
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
            Line::from(vec![
                Span::raw("Corpus count: "),
                Span::styled(
                    data.corpus_count.to_string(),
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
            Line::from(vec![Span::raw("Saved crashes: "), data.span_if_crash()]),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Statistics")
                .bold()
                .title_alignment(Alignment::Center),
        );

        frame.render_widget(paragraph, chunks[0]);
    }

    fn speed_right(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Execution speed evolution (execs/s)")
                    .bold()
                    .title_alignment(Alignment::Center),
            )
            .data(&self.fuzzing_speed)
            .style(Style::default().fg(Color::Red))
            .bar_set(symbols::bar::NINE_LEVELS);

        let stats_chunk = chunks[0].inner(Margin {
            vertical: 1,
            horizontal: 1,
        });
        let stats = [
            format!(
                "Max: {:.2}",
                self.fuzzing_speed
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0)
            ),
            format!(
                "Min: {:.2}",
                self.fuzzing_speed
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0)
            ),
            format!(
                "Avg: {:.2}",
                self.fuzzing_speed.iter().sum::<u64>() / self.fuzzing_speed.len() as u64
            ),
        ];

        frame.render_widget(sparkline, chunks[0]);
        for (i, stat) in stats.iter().enumerate() {
            let stat_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)])
                .split(Rect {
                    x: stats_chunk.x,
                    y: stats_chunk.y + i as u16,
                    width: stats_chunk.width,
                    height: 1,
                });

            frame.render_widget(
                Paragraph::new(stat.as_str()).style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::ITALIC),
                ),
                stat_layout[0],
            );
        }
    }

    fn render_chart(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let corpus_counter: &[(f64, f64)] = &self.corpus_watcher.as_tuple_slice();

        let chart_manager = ChartManager::new(corpus_counter);
        f.render_widget(chart_manager.create_chart(), chunks[0]);
    }

    fn render_bottom(&mut self, f: &mut Frame, area: Rect) -> anyhow::Result<()> {
        let bottom_parts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let seed_info = self.display_fuzzed_seed();
        f.render_widget(seed_info, bottom_parts[0]);

        Ok(())
    }

    fn display_fuzzed_seed(&mut self) -> Paragraph {
        let seed_displayer = SeedDisplayer::new(self.clone().ziggy_config.fuzz_output());
        let seed_info_text: String = match seed_displayer.load() {
            None => String::new(),
            Some(e) => e.to_string(),
        };

        self.seed_text.lines.clear();

        if !seed_info_text.is_empty() {
            self.seed_text.extend(Line::raw(seed_info_text));
        } else {
            self.seed_text.extend(Line::styled(
                "Running the seeds, please wait until we actually start fuzzing",
                Style::new().fg(Color::Green).bold(),
            ));
        }

        Paragraph::new(self.seed_text.clone()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default())
                .title(Span::styled(
                    "Last Fuzzed Messages",
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center),
        )
    }

    pub fn initialize_tui(&mut self, mut child: Child) -> anyhow::Result<()> {
        let stdout = std::io::stdout();
        let backend = ratatui::backend::CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        terminal.clear()?;

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })?;

        while running.load(Ordering::SeqCst) {
            terminal.draw(|f| {
                if let Err(err) = self.ui(f) {
                    eprintln!("{:?}", err);
                }
            })?;

            if event::poll(Duration::from_millis(Self::REFRESH_MS))? {
                if let Event::Key(key) = crossterm::event::read()? {
                    if key.kind == event::KeyEventKind::Press
                        && key.code == event::KeyCode::Char('q')
                    {
                        let _ = child.kill();
                        break;
                    }
                }
            }
        }
        let _ = child.kill();
        terminal.clear()?;

        println!(
            "👋 It was nice fuzzing with you. Killing PID {}. Bye bye! ",
            child.id()
        );

        Ok(())
    }
}
