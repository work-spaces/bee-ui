use super::data::TimeSeriesDataset;

pub const DEFAULT_MIN_MARGIN_X: f64 = 0.001;
const DEFAULT_DEGENERATE_SPAN: f64 = 1.0;
const EPSILON: f64 = 1e-9;

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
    pub lock_aspect_ratio: bool,
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
            lock_aspect_ratio: false,
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
    pub fn x_span(&self) -> f64 {
        (self.x_max - self.x_min).abs()
    }

    pub fn y_span(&self) -> f64 {
        (self.y_max - self.y_min).abs()
    }

    pub fn normalized_bounds(&self) -> (f64, f64, f64, f64) {
        let x_fallback = sanitize_window_size(self.x_window_size, self.x_span());
        let y_fallback = sanitize_window_size(self.y_span(), DEFAULT_DEGENERATE_SPAN);

        let (x_min, x_max) = sanitize_range(self.x_min, self.x_max, x_fallback);
        let (y_min, y_max) = sanitize_range(self.y_min, self.y_max, y_fallback);

        (x_min, x_max, y_min, y_max)
    }

    pub fn set_x_mode(&mut self, mode: AxisZoomMode) {
        self.x_mode = mode;
    }

    pub fn set_y_mode(&mut self, mode: AxisZoomMode) {
        self.y_mode = mode;
    }

    pub fn set_follow_mode(&mut self, mode: FollowMode) {
        self.follow = mode;
    }

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

    pub fn fit_x_to_data_bounds(&mut self, bounds: DataBounds, min_margin_x: f64) {
        let x_fallback = sanitize_window_size(self.x_window_size, bounds.x_span().abs());
        let (x_min, x_max) = sanitize_range(bounds.x_min, bounds.x_max, x_fallback);

        self.x_min = x_min;
        self.x_max = x_max;
        self.x_window_size = x_max - x_min;
        self.clamp_x_to_data_bounds(bounds, min_margin_x);
    }

    pub fn fit_y_to_data_bounds(&mut self, bounds: DataBounds) {
        self.fit_y_to_range(bounds.y_min, bounds.y_max);
    }

    pub fn fit_y_to_range(&mut self, y_min: f64, y_max: f64) {
        let fallback = sanitize_window_size(self.y_span(), DEFAULT_DEGENERATE_SPAN);
        let (y_min, y_max) = sanitize_range(y_min, y_max, fallback);
        self.y_min = y_min;
        self.y_max = y_max;
    }

    pub fn fit_xy_to_data_bounds(&mut self, bounds: DataBounds, min_margin_x: f64) {
        self.fit_x_to_data_bounds(bounds, min_margin_x);
        self.fit_y_to_data_bounds(bounds);
    }

    pub fn pin_x_to_latest(&mut self, data_x_min: f64, data_x_max: f64, min_margin_x: f64) -> bool {
        let Some((data_x_min, data_x_max)) = ordered_pair(data_x_min, data_x_max) else {
            return false;
        };

        let window_size = sanitize_window_size(self.x_window_size, data_x_max - data_x_min);
        let next_min = data_x_max - window_size;
        let next_max = data_x_max;

        if let Some((x_min, x_max)) =
            clamp_x_window_to_data(next_min, next_max, data_x_min, data_x_max, min_margin_x)
        {
            let changed = !approx_eq(self.x_min, x_min) || !approx_eq(self.x_max, x_max);
            self.x_min = x_min;
            self.x_max = x_max;
            self.x_window_size = x_max - x_min;
            return changed;
        }

        false
    }

    pub fn set_x_window_size_manual(
        &mut self,
        window_size: f64,
        data_x_min: f64,
        data_x_max: f64,
        min_margin_x: f64,
    ) -> bool {
        let Some((data_x_min, data_x_max)) = ordered_pair(data_x_min, data_x_max) else {
            return false;
        };

        let next_window_size = sanitize_window_size(window_size, self.x_window_size);
        let center = finite_midpoint(self.x_min, self.x_max).unwrap_or(data_x_max);
        let next_min = center - next_window_size * 0.5;
        let next_max = center + next_window_size * 0.5;

        if let Some((x_min, x_max)) =
            clamp_x_window_to_data(next_min, next_max, data_x_min, data_x_max, min_margin_x)
        {
            let changed = !approx_eq(self.x_min, x_min) || !approx_eq(self.x_max, x_max);
            self.x_min = x_min;
            self.x_max = x_max;
            self.x_window_size = x_max - x_min;
            return changed;
        }

        false
    }

    pub fn set_x_window_size_preserving_pin(
        &mut self,
        window_size: f64,
        data_x_min: f64,
        data_x_max: f64,
        min_margin_x: f64,
    ) -> bool {
        let keep_pinned = self.follow == FollowMode::PinnedToLatest;

        let mut changed =
            self.set_x_window_size_manual(window_size, data_x_min, data_x_max, min_margin_x);

        if keep_pinned {
            self.x_mode = AxisZoomMode::Manual;
            changed |= self.pin_x_to_latest(data_x_min, data_x_max, min_margin_x);
            self.follow = FollowMode::PinnedToLatest;
        }

        changed
    }

    pub fn set_x_start_manual(
        &mut self,
        x_start: f64,
        data_x_min: f64,
        data_x_max: f64,
        min_margin_x: f64,
    ) -> bool {
        let window_size = sanitize_window_size(self.x_window_size, self.x_span());
        let requested_end = x_start + window_size;

        if let Some((x_min, x_max)) =
            clamp_x_window_to_data(x_start, requested_end, data_x_min, data_x_max, min_margin_x)
        {
            let changed = !approx_eq(self.x_min, x_min) || !approx_eq(self.x_max, x_max);
            self.x_min = x_min;
            self.x_max = x_max;
            self.x_window_size = x_max - x_min;
            self.x_mode = AxisZoomMode::Manual;
            self.follow = FollowMode::Free;
            return changed;
        }

        false
    }

    pub fn set_manual_x_bounds(
        &mut self,
        x_min: f64,
        x_max: f64,
        data_x_min: f64,
        data_x_max: f64,
        min_margin_x: f64,
    ) -> bool {
        let requested_width = sanitize_window_size((x_max - x_min).abs(), self.x_window_size);
        let Some((x_min, x_max)) = clamp_x_window_to_data(
            x_min,
            x_min + requested_width,
            data_x_min,
            data_x_max,
            min_margin_x,
        ) else {
            return false;
        };

        let changed = !approx_eq(self.x_min, x_min) || !approx_eq(self.x_max, x_max);
        self.x_min = x_min;
        self.x_max = x_max;
        self.x_window_size = x_max - x_min;
        self.x_mode = AxisZoomMode::Manual;
        self.follow = FollowMode::Free;
        changed
    }

    pub fn set_manual_y_bounds(&mut self, y_min: f64, y_max: f64) -> bool {
        let fallback = sanitize_window_size(self.y_span(), DEFAULT_DEGENERATE_SPAN);
        let (y_min, y_max) = sanitize_range(y_min, y_max, fallback);

        let changed = !approx_eq(self.y_min, y_min) || !approx_eq(self.y_max, y_max);
        self.y_min = y_min;
        self.y_max = y_max;
        self.y_mode = AxisZoomMode::Manual;
        changed
    }

    pub fn apply_modes_for_dataset(
        &mut self,
        dataset: &TimeSeriesDataset,
        min_margin_x: f64,
    ) -> Option<DataBounds> {
        let data_bounds = compute_data_bounds(dataset)?;
        let visible_bounds = compute_visible_data_bounds(dataset).unwrap_or(data_bounds);

        match self.x_mode {
            AxisZoomMode::FitAll | AxisZoomMode::FitVisible => {
                self.fit_x_to_data_bounds(data_bounds, min_margin_x);
            }
            AxisZoomMode::Manual => {
                self.normalize_x_window_size();
                if self.follow == FollowMode::PinnedToLatest {
                    self.pin_x_to_latest(data_bounds.x_min, data_bounds.x_max, min_margin_x);
                } else {
                    self.clamp_x_to_data_bounds(data_bounds, min_margin_x);
                }
            }
        }

        match self.y_mode {
            AxisZoomMode::FitAll => {
                self.fit_y_to_data_bounds(visible_bounds);
            }
            AxisZoomMode::FitVisible => {
                if let Some(bounds) = compute_visible_window_bounds(dataset, self.x_min, self.x_max)
                    .or(Some(visible_bounds))
                {
                    self.fit_y_to_data_bounds(bounds);
                }
            }
            AxisZoomMode::Manual => {}
        }

        Some(data_bounds)
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
        DEFAULT_DEGENERATE_SPAN
    }
}

fn sanitize_range(min: f64, max: f64, fallback_span: f64) -> (f64, f64) {
    let (mut min, mut max) = ordered_pair(min, max).unwrap_or((0.0, fallback_span));
    if approx_eq(min, max) {
        let span = sanitize_window_size(fallback_span, DEFAULT_DEGENERATE_SPAN);
        min -= span * 0.5;
        max += span * 0.5;
    }

    (min, max)
}

fn finite_midpoint(a: f64, b: f64) -> Option<f64> {
    if a.is_finite() && b.is_finite() {
        Some((a + b) * 0.5)
    } else {
        None
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{PlotPoint, TimeSeries, TimeSeriesDataset};

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

    fn sparse_dataset() -> TimeSeriesDataset {
        TimeSeriesDataset::new(vec![
            TimeSeries::new("single", "Single").with_points(vec![PlotPoint::new(42.0, 5.0)]),
        ])
    }

    fn dense_dataset() -> TimeSeriesDataset {
        let points: Vec<PlotPoint> = (0..=200)
            .map(|x| {
                let x = x as f64;
                PlotPoint::new(x, (x * 0.1).sin() * 10.0)
            })
            .collect();

        TimeSeriesDataset::new(vec![TimeSeries::new("dense", "Dense").with_points(points)])
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

    #[test]
    fn pin_to_latest_preserves_window_size() {
        let mut viewport = ViewportState {
            x_min: -4.0,
            x_max: 0.0,
            x_window_size: 4.0,
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::PinnedToLatest,
            ..ViewportState::default()
        };

        let changed = viewport.pin_x_to_latest(10.0, 20.0, DEFAULT_MIN_MARGIN_X);

        assert!(changed);
        assert!((viewport.x_max - 20.0).abs() <= 1e-9);
        assert!((viewport.x_window_size - 4.0).abs() <= 1e-9);
        assert!((viewport.x_min - 16.0).abs() <= 1e-9);
    }

    #[test]
    fn resizing_window_preserves_pinned_follow_and_latest_anchor() {
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::PinnedToLatest,
            x_min: 16.0,
            x_max: 20.0,
            x_window_size: 4.0,
            ..ViewportState::default()
        };

        let changed =
            viewport.set_x_window_size_preserving_pin(2.0, 0.0, 20.0, DEFAULT_MIN_MARGIN_X);

        assert!(changed);
        assert_eq!(viewport.follow, FollowMode::PinnedToLatest);
        assert_eq!(viewport.x_mode, AxisZoomMode::Manual);
        assert!((viewport.x_max - 20.0).abs() <= 1e-9);
        assert!((viewport.x_window_size - 2.0).abs() <= 1e-9);
        assert!((viewport.x_min - 18.0).abs() <= 1e-9);
    }

    #[test]
    fn set_x_start_manual_switches_follow_to_free() {
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::PinnedToLatest,
            x_min: 16.0,
            x_max: 20.0,
            x_window_size: 4.0,
            ..ViewportState::default()
        };

        let changed = viewport.set_x_start_manual(12.0, 0.0, 20.0, DEFAULT_MIN_MARGIN_X);

        assert!(changed);
        assert_eq!(viewport.follow, FollowMode::Free);
        assert_eq!(viewport.x_mode, AxisZoomMode::Manual);
    }

    #[test]
    fn apply_modes_for_dataset_uses_visible_y_bounds_when_requested() {
        let dataset = sample_dataset();
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            y_mode: AxisZoomMode::FitVisible,
            x_min: 0.0,
            x_max: 2.0,
            x_window_size: 2.0,
            follow: FollowMode::Free,
            ..ViewportState::default()
        };

        let data_bounds = viewport
            .apply_modes_for_dataset(&dataset, DEFAULT_MIN_MARGIN_X)
            .expect("expected bounds");

        assert_eq!(data_bounds.x_min, 0.0);
        assert_eq!(data_bounds.x_max, 2.0);
        assert_eq!(viewport.y_min, 1.0);
        assert_eq!(viewport.y_max, 3.0);
    }

    #[test]
    fn set_x_start_manual_clamps_to_limits() {
        let mut viewport = ViewportState {
            x_min: 0.0,
            x_max: 2.0,
            x_window_size: 2.0,
            ..ViewportState::default()
        };

        let changed = viewport.set_x_start_manual(-100.0, 10.0, 20.0, DEFAULT_MIN_MARGIN_X);
        assert!(changed);

        let limits =
            compute_x_navigation_limits(10.0, 20.0, viewport.x_window_size, DEFAULT_MIN_MARGIN_X)
                .expect("limits");

        assert!(viewport.x_min >= limits.min_x - 1e-9);
        assert!(viewport.x_max <= limits.max_x + 1e-9);
    }

    #[test]
    fn apply_modes_noops_for_empty_dataset() {
        let dataset = TimeSeriesDataset::default();
        let mut viewport = ViewportState::default();
        let before = viewport;

        let bounds = viewport.apply_modes_for_dataset(&dataset, DEFAULT_MIN_MARGIN_X);

        assert!(bounds.is_none());
        assert_eq!(viewport, before);
    }

    #[test]
    fn sparse_dataset_clamps_x_window_to_margin_bounds() {
        let dataset = sparse_dataset();
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::Free,
            x_min: -100.0,
            x_max: -90.0,
            x_window_size: 10.0,
            ..ViewportState::default()
        };

        let bounds = viewport
            .apply_modes_for_dataset(&dataset, DEFAULT_MIN_MARGIN_X)
            .expect("expected data bounds");

        let limits =
            compute_x_navigation_limits(bounds.x_min, bounds.x_max, 10.0, DEFAULT_MIN_MARGIN_X)
                .expect("limits");

        assert!((viewport.x_min - limits.min_x).abs() <= 1e-9);
        assert!((viewport.x_max - limits.max_x).abs() <= 1e-9);
        assert!(viewport.x_max > viewport.x_min);
    }

    #[test]
    fn dense_dataset_manual_view_is_clamped_when_far_outside_data() {
        let dataset = dense_dataset();
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::Free,
            x_min: -80.0,
            x_max: -70.0,
            x_window_size: 10.0,
            ..ViewportState::default()
        };

        let bounds = viewport
            .apply_modes_for_dataset(&dataset, 0.5)
            .expect("expected data bounds");

        let limits =
            compute_x_navigation_limits(bounds.x_min, bounds.x_max, viewport.x_window_size, 0.5)
                .expect("limits");

        assert!((viewport.x_min - limits.min_x).abs() <= 1e-9);
        assert!((viewport.x_max - (limits.min_x + viewport.x_window_size)).abs() <= 1e-9);
    }

    #[test]
    fn manual_x_bounds_switches_follow_to_free() {
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::PinnedToLatest,
            x_min: 0.0,
            x_max: 4.0,
            x_window_size: 4.0,
            ..ViewportState::default()
        };

        let changed = viewport.set_manual_x_bounds(2.0, 6.0, 0.0, 20.0, DEFAULT_MIN_MARGIN_X);

        assert!(changed);
        assert_eq!(viewport.follow, FollowMode::Free);
        assert_eq!(viewport.x_mode, AxisZoomMode::Manual);
    }

    #[test]
    fn pinned_follow_tracks_newest_data_when_dataset_extends() {
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            y_mode: AxisZoomMode::FitVisible,
            follow: FollowMode::PinnedToLatest,
            x_min: 6.0,
            x_max: 10.0,
            x_window_size: 4.0,
            ..ViewportState::default()
        };

        let first =
            TimeSeriesDataset::new(vec![TimeSeries::new("live", "Live").with_points(vec![
                PlotPoint::new(0.0, 1.0),
                PlotPoint::new(5.0, 1.1),
                PlotPoint::new(10.0, 0.9),
            ])]);

        viewport.apply_modes_for_dataset(&first, DEFAULT_MIN_MARGIN_X);
        assert!((viewport.x_max - 10.0).abs() <= 1e-9);

        let second =
            TimeSeriesDataset::new(vec![TimeSeries::new("live", "Live").with_points(vec![
                PlotPoint::new(0.0, 1.0),
                PlotPoint::new(10.0, 0.9),
                PlotPoint::new(15.0, 1.2),
                PlotPoint::new(22.0, 1.4),
            ])]);

        viewport.apply_modes_for_dataset(&second, DEFAULT_MIN_MARGIN_X);

        assert!((viewport.x_max - 22.0).abs() <= 1e-9);
        assert!((viewport.x_window_size - 4.0).abs() <= 1e-9);
        assert!((viewport.x_min - 18.0).abs() <= 1e-9);
    }

    #[test]
    fn fit_all_y_excludes_hidden_series() {
        let dataset = sample_dataset();
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::FitAll,
            y_mode: AxisZoomMode::FitAll,
            ..ViewportState::default()
        };

        viewport.apply_modes_for_dataset(&dataset, DEFAULT_MIN_MARGIN_X);

        assert_eq!(viewport.y_min, 1.0);
        assert_eq!(viewport.y_max, 3.0);
    }

    #[test]
    fn fit_x_uses_all_series_extent_including_hidden() {
        let dataset = TimeSeriesDataset::new(vec![
            TimeSeries::new("visible", "Visible")
                .with_points(vec![PlotPoint::new(0.0, 1.0), PlotPoint::new(2.0, 2.0)]),
            TimeSeries::new("hidden", "Hidden")
                .visible(false)
                .with_points(vec![PlotPoint::new(40.0, -1.0), PlotPoint::new(50.0, -2.0)]),
        ]);

        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::FitAll,
            y_mode: AxisZoomMode::Manual,
            ..ViewportState::default()
        };

        viewport.apply_modes_for_dataset(&dataset, DEFAULT_MIN_MARGIN_X);

        assert_eq!(viewport.x_min, 0.0);
        assert_eq!(viewport.x_max, 50.0);
    }

    #[test]
    fn navigation_margin_uses_max_of_ratio_and_minimum() {
        let ratio_limits =
            compute_x_navigation_limits(10.0, 20.0, 4.0, 0.01).expect("ratio limits");
        assert!((ratio_limits.margin_x - 0.2).abs() <= 1e-9);

        let minimum_limits =
            compute_x_navigation_limits(10.0, 20.0, 1.0, 0.5).expect("minimum limits");
        assert!((minimum_limits.margin_x - 0.5).abs() <= 1e-9);
    }
}
