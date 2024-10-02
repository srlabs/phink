use chrono::DateTime;
use ratatui::{
    layout::{
        Alignment,
        Constraint,
    },
    style::{
        Color,
        Style,
        Stylize,
    },
    symbols::Marker,
    text::Span,
    widgets,
    widgets::{
        block::Title,
        Axis,
        Block,
        Chart,
        Dataset,
        GraphType,
    },
};
use widgets::LegendPosition;

pub struct ChartManager<'a> {
    f64_array: &'a [(f64, f64)],
}

impl<'a> ChartManager<'a> {
    pub fn new(f64_array: &'a [(f64, f64)]) -> Self {
        Self { f64_array }
    }

    fn get_x_values(&self) -> Vec<f64> {
        self.f64_array.iter().map(|(x, _)| *x).collect()
    }

    fn get_y_values(&self) -> Vec<f64> {
        self.f64_array.iter().map(|(_, y)| *y).collect()
    }

    fn find_max(values: &[f64]) -> f64 {
        values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    }

    fn find_min(values: &[f64]) -> f64 {
        values.iter().cloned().fold(f64::INFINITY, f64::min)
    }
    fn timestamp_to_str(unix_timestamp: f64) -> String {
        DateTime::from_timestamp(unix_timestamp as i64, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
    pub fn get_first_x(&self) -> f64 {
        let x_values = self.get_x_values();
        Self::find_min(&x_values)
    }

    pub fn get_middle_x(&self) -> f64 {
        let x_values = self.get_x_values();
        let min_x = Self::find_min(&x_values);
        let max_x = Self::find_max(&x_values);
        (min_x + max_x) / 2.0
    }

    pub fn get_max_x(&self) -> f64 {
        let x_values = self.get_x_values();
        Self::find_max(&x_values)
    }

    pub fn get_first_y(&self) -> f64 {
        let y_values = self.get_y_values();
        Self::find_min(&y_values)
    }

    pub fn get_middle_y(&self) -> f64 {
        let y_values = self.get_y_values();
        let min_y = Self::find_min(&y_values);
        let max_y = Self::find_max(&y_values);
        (min_y + max_y) / 2.0
    }

    fn get_max_y(&self) -> f64 {
        let y_values = self.get_y_values();
        Self::find_max(&y_values)
    }

    pub fn create_chart(&self) -> Chart {
        let dataset = vec![Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Green))
            .data(self.f64_array)];

        Chart::new(dataset)
            .block(
                Block::bordered().title(
                    Title::default()
                        .content("Corpus evolution".bold())
                        .alignment(Alignment::Center),
                ),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray).italic())
                    .bounds([self.get_first_x(), self.get_max_x()])
                    .labels([
                        Self::timestamp_to_str(self.get_first_x()).bold(),
                        Span::from(Self::timestamp_to_str(self.get_middle_x())),
                        Self::timestamp_to_str(self.get_max_x()).bold(),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Number of entries")
                    .style(Style::default().fg(Color::Gray).italic())
                    .bounds([self.get_first_y(), self.get_max_y()])
                    .labels([
                        self.get_first_y().to_string().bold(),
                        Span::from(self.get_middle_y().to_string()),
                        self.get_max_y().to_string().bold(),
                    ]),
            )
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)))
    }
}
