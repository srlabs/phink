use ratatui::{
    layout::Constraint,
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
    pub fn new(corpus_counter: &'a [(f64, f64)]) -> Self {
        Self {
            f64_array: corpus_counter,
        }
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

    pub fn get_first_x(&self) -> String {
        let x_values = self.get_x_values();
        format!("{:.1}", Self::find_min(&x_values))
    }

    pub fn get_middle_x(&self) -> String {
        let x_values = self.get_x_values();
        let min_x = Self::find_min(&x_values);
        let max_x = Self::find_max(&x_values);
        format!("{:.1}", (min_x + max_x) / 2.0)
    }

    pub fn get_max_x(&self) -> String {
        let x_values = self.get_x_values();
        format!("{:.1}", Self::find_max(&x_values))
    }

    pub fn get_first_y(&self) -> String {
        let y_values = self.get_y_values();
        format!("{:.1}", Self::find_min(&y_values))
    }

    pub fn get_middle_y(&self) -> String {
        let y_values = self.get_y_values();
        let min_y = Self::find_min(&y_values);
        let max_y = Self::find_max(&y_values);
        format!("{:.1}", (min_y + max_y) / 2.0)
    }

    fn get_max_y(&self) -> String {
        let y_values = self.get_y_values();
        format!("{:.1}", Self::find_max(&y_values))
    }

    pub fn create_chart(&self) -> Chart {
        let dataset = vec![Dataset::default()
            .name("Number of entries")
            .marker(Marker::Dot)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(self.f64_array)];

        Chart::new(dataset)
            .block(
                Block::bordered().title(
                    Title::default()
                        .content("Corpus evolution over time".cyan().bold())
                        .alignment(ratatui::layout::Alignment::Center),
                ),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .labels([
                        self.get_first_x().bold(),
                        Span::from(self.get_middle_x()),
                        self.get_max_x().bold(),
                    ]),
            )
            .y_axis(
                Axis::default()
                    .title("Number of entries")
                    .style(Style::default().fg(Color::Gray))
                    .labels([
                        self.get_first_y().bold(),
                        Span::from(self.get_middle_y()),
                        self.get_max_y().bold(),
                    ]),
            )
            .legend_position(Some(LegendPosition::TopLeft))
            .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)))
    }
}
