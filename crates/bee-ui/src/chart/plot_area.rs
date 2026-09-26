#[derive(Clone, Debug, PartialEq)]
pub struct PlotArea {
    pub id: String,
    pub title: Option<String>,
    pub x_axis_label: Option<String>,
    pub y_axis_label: Option<String>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub fill_width: bool,
    pub legend: bool,
}

impl PlotArea {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: None,
            x_axis_label: None,
            y_axis_label: None,
            width: None,
            height: Some(220.0),
            fill_width: true,
            legend: false,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn x_axis_label(mut self, label: impl Into<String>) -> Self {
        self.x_axis_label = Some(label.into());
        self
    }

    pub fn y_axis_label(mut self, label: impl Into<String>) -> Self {
        self.y_axis_label = Some(label.into());
        self
    }

    pub fn axis_labels(mut self, x_axis: impl Into<String>, y_axis: impl Into<String>) -> Self {
        self.x_axis_label = Some(x_axis.into());
        self.y_axis_label = Some(y_axis.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.fill_width = fill_width;
        self
    }

    pub fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }

    pub(crate) fn build_plot<'a>(&'a self, available_width: f32) -> egui_plot::Plot<'a> {
        let mut plot = egui_plot::Plot::new(self.id.as_str());

        if let Some(width) = self.width {
            plot = plot.width(normalize_non_negative(width));
        } else if self.fill_width {
            plot = plot.width(normalize_non_negative(available_width));
        }

        if let Some(height) = self.height {
            plot = plot.height(normalize_non_negative(height));
        }

        if let Some(x_axis_label) = &self.x_axis_label {
            plot = plot.x_axis_label(x_axis_label.clone());
        }

        if let Some(y_axis_label) = &self.y_axis_label {
            plot = plot.y_axis_label(y_axis_label.clone());
        }

        if self.legend {
            plot = plot.legend(egui_plot::Legend::default());
        }

        plot
    }
}

fn normalize_non_negative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}
