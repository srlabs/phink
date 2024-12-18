use crate::{
    cli::{
        config::PFiles,
        ui::traits::FromPath,
    },
    ResultOf,
};
use anyhow::bail;
use ratatui::{
    style::{
        Color,
        Modifier,
        Style,
    },
    text::Span,
};
use regex::Regex;
use std::{
    fmt,
    fmt::{
        Debug,
        Formatter,
    },
    fs,
    fs::File,
    io,
    io::{
        BufRead,
        BufReader,
        Seek,
        SeekFrom,
    },
    path::PathBuf,
    str::FromStr,
};

#[derive(Default, Debug, Clone)]
pub struct AFLProperties {
    pub run_time: String,
    pub last_new_find: String,
    pub last_saved_crash: String,
    pub corpus_count: u32,
    pub saved_crashes: u32,
    pub exec_speed: u32,
    pub stability: f64,
}

impl AFLProperties {
    pub fn crashed(&self) -> bool {
        self.saved_crashes > 0
    }

    pub fn bad_stability(&self) -> bool {
        self.stability < 0.8
    }

    pub fn span_if_crash(&self) -> Span {
        let crashes = &self.saved_crashes;
        if crashes > &0 {
            Span::styled(
                format!("{} (you have some crashes, please check!!)", crashes),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
                    .underline_color(Color::White)
                    .fg(Color::Green),
            )
        } else {
            Span::styled(
                crashes.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )
        }
    }

    pub fn span_if_bad_stability(&self) -> Span {
        let stability = format!("{}%", &self.stability * 100_f64);

        if self.bad_stability() {
            Span::styled(
                format!("{stability} (the seeds are very unstable!!)",),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
                    .underline_color(Color::White)
                    .fg(Color::Red),
            )
        } else {
            Span::styled(
                stability.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )
        }
    }
}

impl FromStr for AFLProperties {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut props = AFLProperties::default();

        // Function to extract value using regex
        fn extract_value<T: FromStr>(text: &str, pattern: &str) -> Option<T> {
            Regex::new(pattern)
                .ok()?
                .captures(text)?
                .get(1)?
                .as_str()
                .parse()
                .ok()
        }

        if let Some(cap) = Regex::new(r"run time : (.+?)\s+│").unwrap().captures(s) {
            props.run_time = cap[1].to_string();
        }

        if let Some(cap) = Regex::new(r"last new find : (.+?)\s+│")
            .unwrap()
            .captures(s)
        {
            props.last_new_find = cap[1].to_string();
        }

        if let Some(cap) = Regex::new(r"last saved crash : (.+?)\s+│")
            .unwrap()
            .captures(s)
        {
            props.last_saved_crash = cap[1].to_string();
        }

        if let Some(cap) = Regex::new(r"stability : (.+?)\s+│").unwrap().captures(s) {
            let percentage_str = cap[1].to_string().replace("%", "");
            let percentage: f64 = percentage_str.parse().unwrap();
            props.stability = percentage / 100.0;
        }

        props.corpus_count = extract_value(s, r"corpus count : (\d+)").unwrap_or_default();
        props.saved_crashes = extract_value(s, r"saved crashes : (\d+)").unwrap_or_default();
        props.exec_speed = extract_value(s, r"exec speed : (\d+)").unwrap_or_default();

        Ok(props)
    }
}
#[derive(Clone, Default)]
pub struct AFLDashboard {
    pub log_fullpath: PathBuf,
}

impl Debug for AFLDashboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Path for the log: {:?}", self.log_fullpath)?;

        let file = File::open(&self.log_fullpath).map_err(|_| fmt::Error)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(0)).map_err(|_| fmt::Error)?;

        if let Ok(lines) = reader.lines().collect::<io::Result<Vec<String>>>() {
            let last_20_lines = lines
                .iter()
                .rev()
                .take(20)
                .cloned()
                .collect::<Vec<String>>();
            for line in last_20_lines.iter().rev() {
                writeln!(f, "{line}")?;
            }
        }
        Ok(())
    }
}

impl FromPath for AFLDashboard {
    type Output = AFLDashboard;

    fn create_instance(log_fullpath: PathBuf) -> Self::Output {
        AFLDashboard { log_fullpath }
    }

    fn get_filetype() -> PFiles {
        PFiles::AFLLog
    }
}

impl AFLDashboard {
    /// Read and parse properties from the log file
    pub fn read_properties(&self) -> ResultOf<AFLProperties> {
        let content = self.show_log()?;

        let delimiter = "AFL";
        let dashboards: Vec<&str> = content.split(delimiter).collect();

        if let Some(last_dashboard) = dashboards.last() {
            let last_dashboard = format!("{}{}", delimiter, last_dashboard);

            let cleaned = Regex::new(r"\x1b\[[^m]*m")?
                .replace_all(&last_dashboard, "")
                .to_string(); // remove ANSI for shell colors

            return Self::parse_properties(&cleaned)
        }
        bail!("Couldn't parse the set of dashboards of AFL")
    }

    pub fn get_path(&self) -> PathBuf {
        self.log_fullpath.to_path_buf()
    }

    pub fn show_log(&self) -> std::io::Result<String> {
        fs::read_to_string(&self.log_fullpath)
    }

    // Function to parse properties using regex
    fn parse_properties(content: &str) -> ResultOf<AFLProperties> {
        match AFLProperties::from_str(content) {
            Ok(e) => Ok(e),
            Err(_) => bail!("Couldn't parse the AFL dashboard"),
        }
    }

    // Check if the dashboard is ready based on specific content
    pub fn is_ready(&self) -> bool {
        fs::read_to_string(&self.log_fullpath)
            .map(|content| content.contains("findings in depth"))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::EmptyResult;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_ready() -> EmptyResult {
        let afl_dashboard = "      AFL ++4.21c {mainaflfuzzer} (./target/afl/debug/phink) [explore]
┌─ process timing ────────────────────────────────────┬─ overall results ────┐
│        run time : 0 days, 0 hrs, 2 min, 49 sec      │  cycles done : 312   │
│   last new find : 0 days, 0 hrs, 2 min, 49 sec      │ corpus count : 5     │
│last saved crash : none seen yet                     │saved crashes : 4     │
│ last saved hang : none seen yet                     │  saved hangs : 0     │
├─ cycle progress ─────────────────────┬─ map coverage┴──────────────────────┤
│  now processing : 1.312 (20.0%)      │    map density : 0.11% / 0.13%      │
│  runs timed out : 0 (0.00%)          │ count coverage : 54.00 bits/tuple   │
├─ stage progress ─────────────────────┼─ findings in depth ─────────────────┤
│  now trying : havoc                  │ favored items : 3 (60.00%)          │
│ stage execs : 174/400 (43.50%)       │  new edges on : 3 (60.00%)          │
│ total execs : 1.51M                  │ total crashes : 0 (0 saved)         │
│  exec speed : 8726/sec               │  total tmouts : 0 (0 saved)         │
├─ fuzzing strategy yields ────────────┴─────────────┬─ item geometry ───────┤
│   bit flips : 0/0, 0/0, 0/0                        │    levels : 1         │
│  byte flips : 0/0, 0/0, 0/0                        │   pending : 0         │
│ arithmetics : 0/0, 0/0, 0/0                        │  pend fav : 0         │
│  known ints : 0/0, 0/0, 0/0                        │ own finds : 0         │
│  dictionary : 0/0, 0/0, 0/0, 0/0                   │  imported : 4         │
│havoc/splice : 0/514k, 0/992k                       │ stability : 100.00%   │
│py/custom/rq : unused, unused, unused, unused       ├───────────────────────┘
│    trim/eff : disabled, n/a                        │          [cpu023: 51%]
└─ strategy: explore ────────── state: started :-) ──┘";

        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "{afl_dashboard}")?;
        let path = temp_file.path();

        let dashboard = AFLDashboard::from_fullpath(path.into())?;
        assert!(dashboard.is_ready());
        Ok(())
    }

    #[test]
    fn test_spot_crashes() -> EmptyResult {
        let afl_dashboard = "      AFL ++4.21c {mainaflfuzzer} (./target/afl/debug/phink) [explore]
┌─ process timing ────────────────────────────────────┬─ overall results ────┐
│        run time : 0 days, 0 hrs, 2 min, 1 sec      │  cycles done : 312   │
│   last new find : 0 days, 0 hrs, 2 min, 49 sec      │ corpus count : 5     │
│last saved crash : none seen yet                     │saved crashes : 4     │
│ last saved hang : none seen yet                     │  saved hangs : 0     │
├─ cycle progress ─────────────────────┬─ map coverage┴──────────────────────┤
│  now processing : 1.312 (20.0%)      │    map density : 0.11% / 0.13%      │
│  runs timed out : 0 (0.00%)          │ count coverage : 54.00 bits/tuple   │
├─ stage progress ─────────────────────┼─ findings in depth ─────────────────┤
│  now trying : havoc                  │ favored items : 3 (60.00%)          │
│ stage execs : 174/400 (43.50%)       │  new edges on : 3 (60.00%)          │
│ total execs : 1.51M                  │ total crashes : 0 (0 saved)         │
│  exec speed : 8726/sec               │  total tmouts : 0 (0 saved)         │
├─ fuzzing strategy yields ────────────┴─────────────┬─ item geometry ───────┤
│   bit flips : 0/0, 0/0, 0/0                        │    levels : 1         │
│  byte flips : 0/0, 0/0, 0/0                        │   pending : 0         │
│ arithmetics : 0/0, 0/0, 0/0                        │  pend fav : 0         │
│  known ints : 0/0, 0/0, 0/0                        │ own finds : 0         │
│  dictionary : 0/0, 0/0, 0/0, 0/0                   │  imported : 4         │
│havoc/splice : 0/514k, 0/992k                       │ stability : 97.42%   │
│py/custom/rq : unused, unused, unused, unused       ├───────────────────────┘
│    trim/eff : disabled, n/a                        │          [cpu023: 51%]
└─ strategy: explore ────────── state: started :-) ──┘

";

        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "{afl_dashboard}")?;
        let path = temp_file.path();

        let dashboard = AFLDashboard::from_fullpath(path.into())?;
        let properties = dashboard.read_properties()?;

        assert_eq!(properties.saved_crashes, 4);
        assert_eq!(properties.run_time, "0 days, 0 hrs, 2 min, 1 sec");
        assert_eq!(properties.exec_speed, 8726);
        assert_eq!(properties.last_new_find, "0 days, 0 hrs, 2 min, 49 sec");
        assert_eq!(properties.last_saved_crash, "none seen yet");
        assert_eq!(properties.corpus_count, 5);
        assert_eq!(properties.stability, 0.9742000000000001);

        Ok(())
    }

    #[test]
    fn test_no_crashes_or_hangs() -> EmptyResult {
        let afl_dashboard = "      AFL ++4.21c {mainaflfuzzer} (./target/afl/debug/phink) [explore]
┌─ process timing ────────────────────────────────────┬─ overall results ────┐
│        run time : 0 days, 1 hrs, 30 min, 0 sec      │  cycles done : 500   │
│   last new find : 0 days, 1 hrs, 15 min, 30 sec     │ corpus count : 10    │
│last saved crash : none seen yet                     │saved crashes : 0     │
│ last saved hang : none seen yet                     │  saved hangs : 0     │
├─ cycle progress ─────────────────────┬─ map coverage┴──────────────────────┤
│  now processing : 2.500 (50.0%)      │    map density : 0.15% / 0.18%      │
│  runs timed out : 0 (0.00%)          │ count coverage : 60.00 bits/tuple   │
├─ stage progress ─────────────────────┼─ findings in depth ─────────────────┤
│  now trying : havoc                  │ favored items : 5 (50.00%)          │
│ stage execs : 250/500 (50.00%)       │  new edges on : 5 (50.00%)          │
│ total execs : 2.5M                   │ total crashes : 0 (0 saved)         │
│  exec speed : 10000/sec              │  total tmouts : 0 (0 saved)         │
├─ fuzzing strategy yields ────────────┴─────────────┬─ item geometry ───────┤
│   bit flips : 0/0, 0/0, 0/0                        │    levels : 2         │
│  byte flips : 0/0, 0/0, 0/0                        │   pending : 0         │
│ arithmetics : 0/0, 0/0, 0/0                        │  pend fav : 0         │
│  known ints : 0/0, 0/0, 0/0                        │ own finds : 5         │
│  dictionary : 0/0, 0/0, 0/0, 0/0                   │  imported : 5         │
│havoc/splice : 0/1M, 0/1.5M                         │ stability : 100.00%   │
│py/custom/rq : unused, unused, unused, unused       ├───────────────────────┘
│    trim/eff : disabled, n/a                        │          [cpu047: 75%]
└─ strategy: explore ────────── state: running ──────┘";

        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "{afl_dashboard}")?;
        let path = temp_file.path();

        let dashboard = AFLDashboard::from_fullpath(path.into())?;
        let properties = dashboard.read_properties()?;

        assert_eq!(properties.saved_crashes, 0);
        assert_eq!(properties.run_time, "0 days, 1 hrs, 30 min, 0 sec");
        assert_eq!(properties.exec_speed, 10000);
        Ok(())
    }

    #[test]
    fn test_with_crashes_and_hangs() -> EmptyResult {
        let afl_dashboard = "      AFL ++4.21c {mainaflfuzzer} (./target/afl/debug/phink) [explore]
┌─ process timing ────────────────────────────────────┬─ overall results ────┐
│        run time : 1 days, 2 hrs, 45 min, 30 sec     │  cycles done : 1000  │
│   last new find : 0 days, 23 hrs, 59 min, 59 sec    │ corpus count : 20    │
│last saved crash : 0 days, 0 hrs, 15 min, 0 sec      │saved crashes : 5     │
│ last saved hang : 0 days, 1 hrs, 30 min, 0 sec      │  saved hangs : 2     │
├─ cycle progress ─────────────────────┬─ map coverage┴──────────────────────┤
│  now processing : 5.000 (100.0%)     │    map density : 0.20% / 0.25%      │
│  runs timed out : 2 (0.04%)          │ count coverage : 75.00 bits/tuple   │
├─ stage progress ─────────────────────┼─ findings in depth ─────────────────┤
│  now trying : havoc                  │ favored items : 10 (50.00%)         │
│ stage execs : 500/500 (100.00%)      │  new edges on : 15 (75.00%)         │
│ total execs : 5M                     │ total crashes : 5 (5 saved)         │
│  exec speed : 15000/sec              │  total tmouts : 2 (2 saved)         │
├─ fuzzing strategy yields ────────────┴─────────────┬─ item geometry ───────┤
│   bit flips : 1/1M, 0/500k, 0/250k                 │    levels : 3         │
│  byte flips : 0/125k, 0/62k, 0/31k                 │   pending : 0         │
│ arithmetics : 0/15k, 0/7k, 0/3k                    │  pend fav : 0         │
│  known ints : 0/1k, 0/500, 0/250                   │ own finds : 15        │
│  dictionary : 0/100, 0/50, 0/25, 0/12              │  imported : 5         │
│havoc/splice : 4/2M, 0/2M                           │ stability : 99.96%    │
│py/custom/rq : unused, unused, unused, unused       ├───────────────────────┘
│    trim/eff : 0.00%/5, disabled                    │          [cpu095: 98%]
└─ strategy: explore ────────── state: running ──────┘";

        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "{afl_dashboard}")?;
        let path = temp_file.path();

        let dashboard = AFLDashboard::from_fullpath(path.into())?;
        let properties = dashboard.read_properties()?;

        assert_eq!(properties.saved_crashes, 5);
        assert_eq!(properties.run_time, "1 days, 2 hrs, 45 min, 30 sec");
        assert_eq!(properties.last_saved_crash, "0 days, 0 hrs, 15 min, 0 sec");
        assert_eq!(properties.exec_speed, 15000);
        Ok(())
    }

    #[test]
    fn test_with_real_fixture() -> EmptyResult {
        let dashboard = AFLDashboard::from_fullpath(PathBuf::from("tests/fixtures/afl.log"))?;
        let properties = dashboard.read_properties()?;
        assert_eq!(properties.saved_crashes, 42);
        assert_eq!(properties.run_time, "0 days, 0 hrs, 1 min, 21 sec");
        assert_eq!(properties.last_saved_crash, "none seen yet");
        assert_eq!(properties.exec_speed, 5555);
        Ok(())
    }
}
