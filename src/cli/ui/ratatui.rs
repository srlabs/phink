use crate::{
    cli::{
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
    },
    instrumenter::instrumentation::Instrumenter,
};
use anyhow::Context;
use backend::CrosstermBackend;
use contract_transcode::ContractMessageTranscoder;
use ratatui::{
    backend,
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
    collections::VecDeque,
    fmt::Write,
    io,
    process::Child,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
        OnceLock,
    },
    thread::sleep,
    time::Duration,
};

#[derive(Clone, Debug)]
pub struct CustomUI {
    ziggy_config: ZiggyConfig,
    afl_dashboard: AFLDashboard,
    corpus_watcher: CorpusWatcher,
    fuzzing_speed: VecDeque<u64>,
}

pub static CTOR_VALUE: OnceLock<String> = OnceLock::new();

impl CustomUI {
    pub fn new(ziggy_config: &ZiggyConfig) -> anyhow::Result<CustomUI> {
        CTOR_VALUE.get_or_init(|| {
            if let Ok(maybe_metadata) = Instrumenter::new(ziggy_config.clone()).find() {
                if let Ok(transcoder) = ContractMessageTranscoder::load(maybe_metadata.specs_path) {
                    if let Some(ctor) = &ziggy_config.clone().config().constructor_payload {
                        return if let Ok(encoded_bytes) = hex::decode(ctor) {
                            if let Ok(str) =
                                transcoder.decode_contract_constructor(&mut &encoded_bytes[..])
                            {
                                str.to_string()
                            } else {
                                "Couldn't decode {encoded_bytes}".to_string()
                            }
                        } else {
                            "Double check your constructor in your `phink.toml`".to_string()
                        }
                    }
                } else {
                    return "Couldn't load the JSON specs".parse().unwrap()
                }
            }
            "-".into()
        });

        let output = ziggy_config.clone().fuzz_output();

        Ok(Self {
            ziggy_config: ziggy_config.clone(),
            afl_dashboard: AFLDashboard::from_output(output.clone())
                .context("Couldn't create AFL dashboard")?,
            corpus_watcher: CorpusWatcher::from_output(output)
                .context("Couldn't create the corpus watcher")?,
            fuzzing_speed: VecDeque::new(),
        })
    }

    fn ui(&mut self, f: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
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
        self.render_bottom(f, chunks[3])
            .context("Couldn't render the bottom span")?;
        Ok(())
    }

    fn render_chart_and_config(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        self.render_chart(f, chunks[0]);
        self.ziggy_config.config().render(f, chunks[1]);
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
            .style(Style::default())
            .alignment(Alignment::Center);
        f.render_widget(octopus, area);
    }
    fn render_title(&self, f: &mut Frame, area: Rect) {
        self.render_octopus(f, area);
        let title = Paragraph::new("Phink Fuzzing Dashboard")
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
        f.render_widget(title, area);
    }

    fn render_stats(&mut self, f: &mut Frame, area: Rect) {
        let data = self.afl_dashboard.read_properties();

        if let Ok(afl) = data {
            self.update_fuzzing_speed(afl.exec_speed.into());
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

    fn update_fuzzing_speed(&mut self, new_speed: u64) {
        const MAX_POINTS: usize = 100; // Adjust as needed

        self.fuzzing_speed.push_back(new_speed);
        if self.fuzzing_speed.len() > MAX_POINTS {
            self.fuzzing_speed.pop_front();
        }
    }
    fn speed_right(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        let speed_vec = &self.fuzzing_speed.make_contiguous();
        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .title("Execution speed evolution (execs/s)")
                    .bold()
                    .title_alignment(Alignment::Center),
            )
            .data(speed_vec)
            .style(Style::default().fg(Color::White))
            .bar_set(symbols::bar::NINE_LEVELS);

        let stats_chunk = chunks[0].inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        frame.render_widget(sparkline, chunks[0]);

        let stats = [
            format!(
                "Max: {:.2}",
                self.fuzzing_speed
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0)
            ),
            format!(
                "Avg: {:.2}",
                self.fuzzing_speed.iter().sum::<u64>() / self.fuzzing_speed.len() as u64
            ),
        ];
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

            let paragraph = Paragraph::new(stat.as_str()).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            );
            frame.render_widget(paragraph, stat_layout[0]);
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

    // Keeps ASCII graphic characters and whitespace as-is.
    // Replaces other characters with a caret (^) followed by the corresponding ASCII character
    // (similar to how bat does it). A null byte (\0) would be displayed as ^@
    // A carriage return (\r) would be displayed as ^M
    // Other control characters would be displayed as ^A, ^B, etc.
    fn escape_non_printable(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for byte in s.bytes() {
            match byte {
                0x20..=0x7E | b'\n' | b'\t' => result.push(byte as char),
                _ => write!(result, "^{}", byte.wrapping_add(64) as char).unwrap(),
            }
        }
        result
    }
    fn display_fuzzed_seed(&mut self) -> Paragraph {
        let mut seed_text: Text = Default::default();
        let seed_info_text: String =
            match SeedDisplayer::new(self.clone().ziggy_config.fuzz_output()).load() {
                None => String::new(),
                Some(e) => e.to_string(),
            };

        if !seed_info_text.is_empty() {
            let escaped_text = Self::escape_non_printable(&seed_info_text);
            for line in escaped_text.lines() {
                seed_text.push_line(Line::styled(line.to_string(), Style::default()));
            }
        } else {
            seed_text.push_span(Span::styled(
                format!(
                    "Running the seeds, please wait until we actually start fuzzing...\n
                 If this screen get stuck for a while, execute `tail -f {}`.",
                    &self.afl_dashboard.get_path().to_str().unwrap()
                ),
                Style::default().fg(Color::Yellow),
            ));
            seed_text.push_span(Span::styled(
                "Either there is a terible bug, either we are still looking for a decodable seed.",
                Style::default().fg(Color::Yellow),
            ));
        }

        Paragraph::new(seed_text.clone()).block(
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
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal =
            ratatui::Terminal::new(backend).context("Couldn't create the terminal backend")?;
        terminal.clear().context("Couldn't clear the terminal")?;

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })?;

        while running.load(Ordering::SeqCst) {
            terminal.draw(|f| {
                sleep(Duration::from_millis(500));
                if let Err(err) = self.ui(f) {
                    eprintln!("{:?}", err);
                }
            })?;
        }

        let i = child.id();

        terminal.clear()?;
        child
            .kill()
            .context(format!("Couldn't kill the child n°{i}"))?;
        println!("👋 It was nice fuzzing with you. Killing PID {i}. Bye bye! ",);

        Ok(())
    }
}
