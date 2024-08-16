use crate::{
    cli::ziggy::ZiggyConfig,
    cover::coverage::COVERAGE_PATH,
};

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fs::{
        self,
        File,
    },
    io::Read,
    path::Path,
};
use walkdir::WalkDir;

pub struct CoverageTracker {
    /// One String (entry) is one .rs file
    coverage: HashMap<String, Vec<bool>>,
    /// One entry is one COV identifier (i.e COV=18)
    hit_lines: HashSet<usize>,
}

impl CoverageTracker {
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

    pub fn process_file(&mut self, file_path: &str) -> std::io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        let mut file_coverage = vec![false; lines.len()];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if let Some(cov_num) =
                trimmed.strip_prefix("ink::env::debug_println!(\"COV={}\", ")
            {
                if let Some(cov_num) = cov_num.strip_suffix(");") {
                    if let Ok(num) = cov_num.parse::<usize>() {
                        if self.hit_lines.contains(&num) {
                            // Mark the current line and previous non-empty
                            // lines as covered
                            file_coverage[i] = true;
                        }
                    }
                }
            }
        }
        self.coverage.insert(file_path.to_string(), file_coverage);
        Ok(())
    }

    pub fn generate_report(&self, output_dir: &str) -> std::io::Result<()> {
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
            println!("file: {:?}", file_path);
            println!("cov: {:?}", coverage);

            // Create a sanitized file name for the report
            let sanitized_path = file_path.replace("/", "_").replace("\\", "_");
            let report_path = format!("{}/{}.html", output_dir, sanitized_path);

            self.generate_file_report(file_path, coverage, &report_path)?;

            // Use the full file path in the index, but display only the file name
            let file_name = Path::new(file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file_path);

            index_html.push_str(&format!(
                "<li><a href='{}.html'>- {}</a></li>",
                sanitized_path, file_path
            ));
        }

        index_html.push_str("</ul></body></html>");
        fs::write(format!("{}/index.html", output_dir), index_html)?;

        Ok(())
    }

    fn generate_file_report(
        &self,
        file_path: &str,
        coverage: &[bool],
        output_path: &str,
    ) -> std::io::Result<()> {
        let source_code = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = source_code.lines().collect();

        let mut html = String::from(
            "<!DOCTYPE html><html><head><title>Phink File Coverage</title><style>
            .covered { background-color: #90EE90; }
            /*.uncovered { background-color: #FFB6C1; }*/
            </style></head><body>",
        );

        html.push_str(&format!("<h1>Coverage for {}</h1><pre>", file_path));
        html.push_str("<h3>This is a beta version of the code visualizer. \
        <br>You can assume that if a line is green, it has been executed. <br>\
        If the green line represents a block (e.g., green `if`), it means that the `if` condition was met, and we got inside the condition.<br>\
        The report doesn't integrate the coverage of the crashing seeds.
        <br></h3>");

        for (i, line) in lines.iter().enumerate() {
            let line_class = if coverage[i] { "covered" } else { "uncovered" };
            // if !line.contains("ink::env::debug_println!(\"COV={}\", ") {
            html.push_str(&format!(
                "<span class='{}'>{:4} | {}</span>\n",
                line_class,
                i + 1,
                html_escape(line)
            ));
            // }
        }

        html.push_str("</pre></body></html>");
        fs::write(output_path, html)?;

        Ok(())
    }

    pub fn generate(config: ZiggyConfig) {
        let mut coverage_trace = match File::open(COVERAGE_PATH) {
            Ok(file) => file,
            Err(_) => {
                println!("❌ Coverage file not found. Please execute the \"run\" command to create the coverage file.");
                return;
            }
        };

        let mut contents = String::new();
        coverage_trace.read_to_string(&mut contents).unwrap();
        println!("📄 Successfully read coverage file. {:?}", coverage_trace);

        let mut tracker = CoverageTracker::new(&contents);
        for entry in WalkDir::new(config.contract_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .filter(|e| !e.path().components().any(|c| c.as_os_str() == "target"))
        {
            tracker
                .process_file(entry.path().as_os_str().to_str().unwrap())
                .expect("🙅 Cannot process file");
        }

        tracker
            .generate_report(config.config.report_path.clone().unwrap().to_str().unwrap())
            .expect("🙅 Cannot generate coverage report");
        println!(
            "📊 Coverage report generated at: {}",
            config.config.report_path.unwrap().display()
        );
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
