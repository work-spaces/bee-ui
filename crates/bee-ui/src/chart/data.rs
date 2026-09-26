use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlotPoint {
    pub x: f64,
    pub y: f64,
}

impl PlotPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SeriesId(String);

impl SeriesId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<&str> for SeriesId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for SeriesId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for SeriesId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimeSeries {
    pub id: SeriesId,
    pub label: String,
    pub points: Vec<PlotPoint>,
    pub visible: bool,
}

impl TimeSeries {
    pub fn new(id: impl Into<SeriesId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            points: Vec::new(),
            visible: true,
        }
    }

    pub fn with_points(mut self, points: Vec<PlotPoint>) -> Self {
        self.points = points;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn push_point(&mut self, point: PlotPoint) {
        self.points.push(point);
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimeSeriesDataset {
    pub series: Vec<TimeSeries>,
}

impl TimeSeriesDataset {
    pub fn new(series: Vec<TimeSeries>) -> Self {
        Self { series }
    }

    pub fn is_empty(&self) -> bool {
        self.series.iter().all(|series| series.points.is_empty())
    }

    pub fn push_series(&mut self, series: TimeSeries) {
        self.series.push(series);
    }
}
