use crate::cli::{
    config::Configuration,
    ui::traits::Paint,
};
use ratatui::{
    layout::{
        Constraint,
        Rect,
    },
    prelude::{
        Color,
        Modifier,
        Style,
        Stylize,
    },
    style::palette::{
        tailwind,
        tailwind::Palette,
    },
    widgets::{
        Block,
        Borders,
        HighlightSpacing,
        Row,
        Table,
    },
    Frame,
};

const PALETTE: Palette = tailwind::RED;
struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
}

impl TableColors {
    const fn new(color: &Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
        }
    }
}

impl Paint for Configuration {
    fn render(&self, f: &mut Frame, area: Rect) {
        // Helper function to format optional fields
        fn format_option<T: std::fmt::Debug>(option: &Option<T>) -> String {
            match option {
                Some(value) => {
                    format!("{:?}", value)
                        .trim_matches(|c| c == '"' || c == '(' || c == ')')
                        .to_string()
                }
                None => "-".to_string(),
            }
        }

        let colors = TableColors::new(&PALETTE);

        let x = &format_option(&self.instantiate_initial_value);
        let x1 = &format_option(&self.cores);
        let x2 = &self.use_honggfuzz.to_string();
        let x3 = &format_option(&self.deployer_address);
        let x4 = &format_option(&self.max_messages_per_exec);
        let x5 = &format_option(&self.report_path);
        let x6 = &self.fuzz_origin.to_string();
        let x7 = &format_option(&self.default_gas_limit);
        let x8 = &format_option(&self.storage_deposit_limit);
        let x9 = &format_option(&self.constructor_payload);
        let x10 = &self.verbose.to_string();
        let x11 = &self
            .instrumented_contract_path
            .clone()
            .unwrap_or_default()
            .to_string();
        let x12 = &format_option(&self.fuzz_output);
        let x13 = &self.show_ui.to_string();
        let items = vec![
            Row::new(vec!["Cores used", x1]),
            Row::new(vec!["Using Honggfuzz", x2]),
            Row::new(vec!["Deployer address", x3]),
            Row::new(vec!["Max messages per exec", x4]),
            Row::new(vec!["Report path", x5]),
            Row::new(vec!["Fuzzing origin", x6]),
            Row::new(vec!["Default gas limit", x7]),
            Row::new(vec!["Storage deposit limit", x8]),
            Row::new(vec!["Instantiate initial value", x]),
            Row::new(vec!["Constructor payload", x9]),
            Row::new(vec!["Verbose mode", x10]),
            Row::new(vec!["Path to instrumented contract", x11]),
            Row::new(vec!["Fuzz output folder", x12]),
            Row::new(vec!["Custom UI", x13]),
        ];

        let rows = items.iter().enumerate().map(|(i, row)| {
            let color = match i % 2 {
                0 => colors.normal_row_color,
                _ => colors.alt_row_color,
            };
            row.clone()
                .style(Style::new().fg(colors.row_fg).bg(color))
                .height(1)
        });

        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(colors.selected_style_fg);
        let header_style = Style::default().fg(colors.header_fg).bg(colors.header_bg);

        let table = Table::new(rows, [Constraint::Length(5), Constraint::Length(5)])
            .header(
                Row::new(vec!["Setting", "Value"])
                    .style(header_style)
                    .bold(),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Configuration"),
            )
            .highlight_style(selected_style)
            .widths([Constraint::Percentage(25), Constraint::Percentage(60)])
            .column_spacing(1)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .bg(colors.buffer_bg)
            .highlight_spacing(HighlightSpacing::Always);

        f.render_widget(table, area);
    }
}
