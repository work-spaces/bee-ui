use super::data::TimeSeriesDataset;

pub const DEFAULT_MIN_MARGIN_X: f64 = 0.001;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AxisZoomMode {
    #[default]
    FitAll,
    FitVisible,
    Manual,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FollowMode {
    #[default]
    PinnedToLatest,
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewportState {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub x_mode: AxisZoomMode,
    pub y_mode: AxisZoomMode,
    pub follow: FollowMode,
    pub x_window_size: f64,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 1.0,
            y_min: -1.0,
            y_max: 1.0,
            x_mode: AxisZoomMode::FitAll,
            y_mode: AxisZoomMode::FitVisible,
            follow: FollowMode::PinnedToLatest,
            x_window_size: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DataBounds {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl DataBounds {
    pub fn x_span(&self) -> f64 {
        self.x_max - self.x_min
    }

    pub fn y_span(&self) -> f64 {
        self.y_max - self.y_min
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XNavigationLimits {
    pub min_x: f64,
    pub max_x: f64,
    pub margin_x: f64,
}

impl ViewportState {
    pub fn normalize_x_window_size(&mut self) {
        self.x_window_size = sanitize_window_size(self.x_window_size, self.x_max - self.x_min);
    }

    pub fn clamp_x_to_data_bounds(&mut self, bounds: DataBounds, min_margin_x: f64) {
        if let Some((x_min, x_max)) = clamp_x_window_to_data(
            self.x_min,
            self.x_max,
            bounds.x_min,
            bounds.x_max,
            min_margin_x,
        ) {
            self.x_min = x_min;
            self.x_max = x_max;
            self.x_window_size = x_max - x_min;
        }
    }
}

pub fn compute_data_bounds(dataset: &TimeSeriesDataset) -> Option<DataBounds> {
    compute_bounds(dataset, false, None)
}

pub fn compute_visible_data_bounds(dataset: &TimeSeriesDataset) -> Option<DataBounds> {
    compute_bounds(dataset, true, None)
}

pub fn compute_visible_window_bounds(
    dataset: &TimeSeriesDataset,
    x_min: f64,
    x_max: f64,
) -> Option<DataBounds> {
    let (x_min, x_max) = ordered_pair(x_min, x_max)?;
    compute_bounds(dataset, true, Some((x_min, x_max)))
}

pub fn compute_x_navigation_limits(
    data_x_min: f64,
    data_x_max: f64,
    x_window_size: f64,
    min_margin_x: f64,
) -> Option<XNavigationLimits> {
    let (data_x_min, data_x_max) = ordered_pair(data_x_min, data_x_max)?;
    let x_window_size = sanitize_window_size(x_window_size, data_x_max - data_x_min);
    let min_margin_x = sanitize_non_negative(min_margin_x);
    let margin_x = (x_window_size * 0.05).max(min_margin_x);

    Some(XNavigationLimits {
        min_x: data_x_min - margin_x,
        max_x: data_x_max + margin_x,
        margin_x,
    })
}

pub fn clamp_x_window_to_limits(x_min: f64, x_max: f64, limits: XNavigationLimits) -> (f64, f64) {
    let limits_span = limits.max_x - limits.min_x;
    if !limits_span.is_finite() || limits_span <= 0.0 {
        return (limits.min_x, limits.max_x);
    }

    let width_hint = if x_min.is_finite() && x_max.is_finite() {
        (x_max - x_min).abs()
    } else {
        limits_span
    };
    let window_size = sanitize_window_size(width_hint, limits_span);

    if window_size >= limits_span {
        return (limits.min_x, limits.max_x);
    }

    let clamped_min = if x_min.is_finite() {
        x_min.clamp(limits.min_x, limits.max_x - window_size)
    } else {
        limits.min_x
    };

    (clamped_min, clamped_min + window_size)
}

pub fn clamp_x_window_to_data(
    x_min: f64,
    x_max: f64,
    data_x_min: f64,
    data_x_max: f64,
    min_margin_x: f64,
) -> Option<(f64, f64)> {
    let limits =
        compute_x_navigation_limits(data_x_min, data_x_max, (x_max - x_min).abs(), min_margin_x)?;
    Some(clamp_x_window_to_limits(x_min, x_max, limits))
}

fn compute_bounds(
    dataset: &TimeSeriesDataset,
    visible_only: bool,
    x_filter: Option<(f64, f64)>,
) -> Option<DataBounds> {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    for series in &dataset.series {
        if visible_only && !series.visible {
            continue;
        }

        for point in &series.points {
            if !point.x.is_finite() || !point.y.is_finite() {
                continue;
            }

            if let Some((filter_x_min, filter_x_max)) = x_filter
                && (point.x < filter_x_min || point.x > filter_x_max)
            {
                continue;
            }

            x_min = x_min.min(point.x);
            x_max = x_max.max(point.x);
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
        }
    }

    if x_min.is_finite() && x_max.is_finite() && y_min.is_finite() && y_max.is_finite() {
        Some(DataBounds {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    } else {
        None
    }
}

fn ordered_pair(a: f64, b: f64) -> Option<(f64, f64)> {
    if !a.is_finite() || !b.is_finite() {
        return None;
    }

    if a <= b { Some((a, b)) } else { Some((b, a)) }
}

fn sanitize_non_negative(value: f64) -> f64 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn sanitize_window_size(window_size: f64, fallback: f64) -> f64 {
    if window_size.is_finite() && window_size > 0.0 {
        window_size
    } else if fallback.is_finite() && fallback > 0.0 {
        fallback
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::{PlotPoint, TimeSeries, TimeSeriesDataset};

    fn sample_dataset() -> TimeSeriesDataset {
        TimeSeriesDataset::new(vec![
            TimeSeries::new("visible", "Visible").with_points(vec![
                PlotPoint::new(0.0, 1.0),
                PlotPoint::new(1.0, 2.0),
                PlotPoint::new(2.0, 3.0),
            ]),
            TimeSeries::new("hidden", "Hidden")
                .visible(false)
                .with_points(vec![PlotPoint::new(0.0, -10.0), PlotPoint::new(2.0, 10.0)]),
        ])
    }

    #[test]
    fn computes_global_bounds_across_all_series() {
        let dataset = sample_dataset();
        let bounds = compute_data_bounds(&dataset).expect("expected global bounds");

        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 2.0);
        assert_eq!(bounds.y_min, -10.0);
        assert_eq!(bounds.y_max, 10.0);
    }

    #[test]
    fn computes_visible_bounds_excluding_hidden_series() {
        let dataset = sample_dataset();
        let bounds = compute_visible_data_bounds(&dataset).expect("expected visible bounds");

        assert_eq!(bounds.x_min, 0.0);
        assert_eq!(bounds.x_max, 2.0);
        assert_eq!(bounds.y_min, 1.0);
        assert_eq!(bounds.y_max, 3.0);
    }

    #[test]
    fn clamps_x_window_to_navigation_limits_with_margin() {
        let limits = compute_x_navigation_limits(10.0, 20.0, 4.0, DEFAULT_MIN_MARGIN_X)
            .expect("expected x navigation limits");

        let (x_min, x_max) = clamp_x_window_to_limits(0.0, 4.0, limits);
        assert_eq!(x_min, limits.min_x);
        assert_eq!(x_max, limits.min_x + 4.0);

        let (x_min, x_max) = clamp_x_window_to_limits(100.0, 104.0, limits);
        assert_eq!(x_max, limits.max_x);
        assert_eq!(x_min, limits.max_x - 4.0);
    }
}
