use crate::cli::ziggy::ZiggyConfig;
use fs::read_to_string;

use crate::{
    cli::config::{
        PFiles::CoverageTracePath,
        PhinkFiles,
    },
    cover::trace::COV_IDENTIFIER,
    EmptyResult,
};
use anyhow::{
    bail,
    Context,
};
use std::{
    collections::HashMap,
    fs::{
        self,
        File,
    },
    io,
    io::Read,
};
use walkdir::WalkDir;

pub struct CoverageTracker {
    /// Maps each *.rs file of the contract to a `Vec<bool>`. This `Vec` represents the coverage
    /// map of the file, with a `len()` equal to the number of lines of the file.
    coverage: HashMap<String, Vec<bool>>,
    /// Stores each hit line's unique identifier.
    hit_lines: Vec<usize>,
}

impl CoverageTracker {
    /// Creates a new `CoverageTracker` from a string representing hit lines.
    pub fn new(coverage_string: &str) -> Self {
        let hit_lines = coverage_string
            .split("\n")
            .filter_map(|s| s.parse().ok())
            .collect();

        CoverageTracker {
            coverage: HashMap::new(),
            hit_lines,
        }
    }

    /// Calculates and prints a benchmark of the coverage achieved
    pub fn benchmark(&self, total_feedback: usize) {
        let mut hit_lines = self.hit_lines.clone();
        hit_lines.sort();
        hit_lines.dedup();

        let total_hit_lines = hit_lines.len();
        let number_of_files = self.coverage.len(); // should be 140 for dummy
        let coverage_percentage = if total_feedback > 0 {
            total_hit_lines * 100 / total_feedback
        } else {
            0 // div. by zero
        };

        println!("📐 Phink Coverage Benchmark:");
        println!("  - Total contract's files: {number_of_files}");
        println!("  - Total of unique hit lines: {total_hit_lines}");
        println!("  - Maximum theoretically reachable coverage: {total_feedback}"); // should be 294 for dummy
        println!("  - Coverage percentage: {coverage_percentage:.2}%");
    }

    /// Create the proper coverage map for the final report
    /// # Returns
    /// Returns the amount of time there was a coverage feedback, reached or not, for one given file
    pub fn process_file(&mut self, file_path: &str) -> io::Result<usize> {
        let content = read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut spotted_debugprint = 0;
        let mut file_coverage = vec![false; lines.len()];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if let Some(cov_num) = trimmed.strip_prefix("ink::env::debug_println!(\"COV={}\", ") {
                if let Some(cov_num) = cov_num.strip_suffix(");") {
                    if let Ok(num) = cov_num.parse::<usize>() {
                        // We increment the number of time we spotted `debug_println`
                        spotted_debugprint += 1;
                        if self.hit_lines.contains(&num) {
                            // Mark the current line and previous non-empty lines as covered
                            // We +1 to avoid marking the debug_println! as the covered one
                            file_coverage[i + 1] = true;
                        }
                    }
                }
            }
        }
        self.coverage.insert(file_path.to_string(), file_coverage);
        Ok(spotted_debugprint)
    }

    pub fn generate_report(&self, output_dir: &str) -> io::Result<()> {
        fs::create_dir_all(output_dir)?;

        let mut index_html = String::from(
            "<!DOCTYPE html>
                        <html>
                        <head>
                            <title>Phink Coverage Report</title>
                            <style>
                                body {
                                    font-family: Arial, sans-serif;
                                    margin: 40px;
                                    background-color: #f4f4f9;
                                }
                                h1 {
                                    color: #333;
                                }
                                ul {
                                    list-style-type: none;
                                    padding: 0;
                                }
                                li {
                                    margin: 10px 0;
                                }
                                a {
                                    text-decoration: none;
                                    color: #007bff;
                                }
                                a:hover {
                                    text-decoration: underline;
                                }
                            </style>
                        </head>
                        <body>
                            <h1>Phink Coverage Report</h1>
                            <ul>",
        );

        for (file_path, coverage) in &self.coverage {
            let sanitized_path = file_path.replace("/", "_").replace("\\", "_");
            let report_path = format!("{output_dir}/{sanitized_path}.html");
            self.generate_file_report(file_path, coverage, &report_path)?;

            index_html.push_str(&format!(
                "<li><a href='{sanitized_path}.html'>- {file_path}</a></li>",
            ));
        }

        index_html.push_str("</ul></body></html>");
        fs::write(format!("{output_dir}/index.html"), index_html)?;
        println!("📊 Coverage report generated at: {output_dir}");

        Ok(())
    }

    fn generate_file_report(
        &self,
        file_path: &str,
        coverage: &[bool],
        output_path: &str,
    ) -> io::Result<()> {
        let source_code = read_to_string(file_path)?;
        let lines: Vec<&str> = source_code.lines().collect();

        let mut html = String::from(
            "<!DOCTYPE html><html><head><title>Phink File Coverage</title><style>
            .covered { background-color: #90EE90; }
            /*.uncovered { background-color: #FFB6C1; }*/
            </style></head><body>",
        );

        html.push_str(&format!("<h1>Coverage for {file_path}</h1><pre>"));
        html.push_str("<h3>This is a beta version of the code visualizer. \
        <br>You can assume that if a line is green, it has been executed. <br>\
        If the green line represents a block (e.g., green `if`), it means that the `if` condition was met, and we got inside the condition.<br>\
        The report doesn't integrate the coverage of the crashing seeds.
        <br></h3>");

        for (i, line) in lines.iter().enumerate() {
            let line_class = if coverage[i] { "covered" } else { "uncovered" };
            html.push_str(&format!(
                "<span class='{}'>{:4} | {}</span>\n",
                line_class,
                i + 1,
                html_escape(line)
            ));
        }

        html.push_str("</pre></body></html>");
        Self::remove_debug_statement(&mut html);
        fs::write(output_path, html)?;

        Ok(())
    }

    pub fn generate(config: ZiggyConfig) -> EmptyResult {
        let cov_trace_path =
            PhinkFiles::new(config.to_owned().fuzz_output()).path(CoverageTracePath);

        let mut coverage_trace = match File::open(cov_trace_path) {
            Ok(file) => file,
            Err(_) => {
                bail!("Coverage file not found. Please execute the \"run\" command to create the coverage file.")
            }
        };

        let mut contents = String::new();
        coverage_trace.read_to_string(&mut contents)?;
        let mut total_feedback = 0;
        let mut tracker = CoverageTracker::new(&contents);
        for entry in WalkDir::new(config.contract_path()?)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
        {
            let entry = entry.path().as_os_str().to_str().unwrap();
            let one_file_feedback = tracker
                .process_file(entry)
                .context(format!("Cannot process {entry:?} file"))?;

            total_feedback += one_file_feedback;
        }

        tracker
            .generate_report(
                config
                    .config()
                    .report_path
                    .clone()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .context("Cannot generate coverage report")?;

        tracker.benchmark(total_feedback);

        Ok(())
    }

    pub fn remove_debug_statement(html: &mut String) {
        let lines: Vec<&str> = html.lines().collect();

        let filtered_lines: Vec<&str> = lines
            .into_iter()
            .filter(|line| {
                !(line.contains("ink::env::debug_println!") && line.contains(COV_IDENTIFIER))
            })
            .collect();

        *html = filtered_lines.join("\n");
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
