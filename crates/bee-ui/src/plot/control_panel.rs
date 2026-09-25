use super::{
    AxisZoomMode, DEFAULT_MIN_MARGIN_X, DataBounds, FollowMode, TimeSeriesDataset, ViewportState,
    compute_data_bounds, compute_visible_data_bounds, compute_visible_window_bounds,
    compute_x_navigation_limits,
};
use crate::{
    UiRoot,
    components::{Col, Container, FormCheck, FormLabel, FormRange, FormSelect, Row},
    show_in_ui,
    theme::bootstrap_tokens,
};

const X_MODE_OPTIONS: &[(AxisZoomMode, &str)] = &[
    (AxisZoomMode::FitAll, "Fit all"),
    (AxisZoomMode::Manual, "Manual"),
];

const Y_MODE_OPTIONS: &[(AxisZoomMode, &str)] = &[
    (AxisZoomMode::FitVisible, "Fit visible"),
    (AxisZoomMode::FitAll, "Fit all"),
    (AxisZoomMode::Manual, "Manual"),
];

const FOLLOW_HINT: &str =
    "Pin the right edge to the latest sample. Toggle with keyboard shortcut `L`.";
const X_WINDOW_HINT: &str = "Visible X span. Changing this keeps live pin if currently pinned.";
const X_POSITION_HINT: &str =
    "Pan horizontally within clamped data bounds. Changing this unpins follow mode.";

#[derive(Clone, Debug, PartialEq)]
pub struct PlotControlPanelProps {
    pub min_margin_x: f64,
    pub has_new_data: bool,
}

impl Default for PlotControlPanelProps {
    fn default() -> Self {
        Self {
            min_margin_x: DEFAULT_MIN_MARGIN_X,
            has_new_data: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlotControlPanelResponse {
    pub changed: bool,
    pub go_live_clicked: bool,
}

pub struct PlotControlPanel;

impl PlotControlPanel {
    pub fn show(
        ui: &mut egui::Ui,
        viewport: &mut ViewportState,
        dataset: &TimeSeriesDataset,
        props: &PlotControlPanelProps,
    ) -> PlotControlPanelResponse {
        let mut response = PlotControlPanelResponse::default();

        let data_bounds = compute_data_bounds(dataset);
        let visible_bounds = compute_visible_data_bounds(dataset).or(data_bounds);

        let window_scale =
            data_bounds.and_then(|bounds| x_window_scale(bounds, props.min_margin_x));
        let position_scale =
            data_bounds.and_then(|bounds| x_position_scale(bounds, viewport, props.min_margin_x));

        let initial = ControlValues {
            follow_newest: viewport.follow == FollowMode::PinnedToLatest,
            x_mode_index: mode_index(X_MODE_OPTIONS, viewport.x_mode),
            y_mode_index: mode_index(Y_MODE_OPTIONS, viewport.y_mode),
            x_window: window_scale.map_or(0.0, |scale| scale.normalize(viewport.x_window_size)),
            x_position: position_scale.map_or(0.0, |scale| scale.normalize(viewport.x_min)),
        };
        let view = PanelView {
            x_mode_placeholder: mode_label(viewport.x_mode),
            y_mode_placeholder: mode_label(viewport.y_mode),
            x_window_enabled: window_scale.is_some(),
            x_position_enabled: position_scale.is_some(),
        };

        let mut edited = initial;
        let values = &mut edited;
        show_in_ui(ui, move |root| build_panel(root, values, view));

        if edited.follow_newest != initial.follow_newest {
            viewport.follow = if edited.follow_newest {
                FollowMode::PinnedToLatest
            } else {
                FollowMode::Free
            };

            if edited.follow_newest {
                viewport.x_mode = AxisZoomMode::Manual;
                if let Some(bounds) = data_bounds {
                    viewport.pin_x_to_latest(bounds.x_min, bounds.x_max, props.min_margin_x);
                }
            }

            response.changed = true;
        }

        if edited.x_mode_index != initial.x_mode_index
            && let Some(&(x_mode, _)) = X_MODE_OPTIONS.get(edited.x_mode_index)
        {
            viewport.x_mode = x_mode;
            if let Some(bounds) = data_bounds {
                apply_x_mode(viewport, x_mode, bounds, props.min_margin_x);
            }
            response.changed = true;
        }

        if edited.y_mode_index != initial.y_mode_index
            && let Some(&(y_mode, _)) = Y_MODE_OPTIONS.get(edited.y_mode_index)
        {
            viewport.y_mode = y_mode;
            if let Some(bounds) = y_bounds_for_mode(dataset, viewport, y_mode).or(visible_bounds) {
                viewport.fit_y_to_data_bounds(bounds);
            }
            response.changed = true;
        }

        if edited.x_window != initial.x_window
            && let (Some(scale), Some(bounds)) = (window_scale, data_bounds)
        {
            viewport.x_mode = AxisZoomMode::Manual;
            response.changed |= viewport.set_x_window_size_preserving_pin(
                scale.value_at(edited.x_window),
                bounds.x_min,
                bounds.x_max,
                props.min_margin_x,
            );
        }

        if edited.x_position != initial.x_position
            && let (Some(scale), Some(bounds)) = (position_scale, data_bounds)
        {
            viewport.x_mode = AxisZoomMode::Manual;
            response.changed |= viewport.set_x_start_manual(
                scale.value_at(edited.x_position),
                bounds.x_min,
                bounds.x_max,
                props.min_margin_x,
            );
        }

        response
    }
}

/// Controlled values bound to the panel's form components. The panel diffs these against
/// their initial snapshot after rendering to detect user edits.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ControlValues {
    follow_newest: bool,
    x_mode_index: usize,
    y_mode_index: usize,
    /// Normalized (`0.0..=1.0`) position on the X window slider.
    x_window: f32,
    /// Normalized (`0.0..=1.0`) position on the X position slider.
    x_position: f32,
}

/// Read-only presentation state for the panel.
#[derive(Clone, Copy, Debug)]
struct PanelView {
    x_mode_placeholder: &'static str,
    y_mode_placeholder: &'static str,
    x_window_enabled: bool,
    x_position_enabled: bool,
}

fn build_panel<'a>(root: &mut UiRoot<'a>, values: &'a mut ControlValues, view: PanelView) {
    let ControlValues {
        follow_newest,
        x_mode_index,
        y_mode_index,
        x_window,
        x_position,
    } = values;

    let spacing = bootstrap_tokens::spacing();
    let row = Row::new().gutter_x(spacing.md).gutter_y(spacing.sm);

    root.container(Container::fluid(), move |root| {
        root.row(row, move |root| {
            root.col(Col::new().xs(12).md(4), move |root| {
                root.form_label(FormLabel::new("Live"));
                root.form_check(
                    follow_newest,
                    FormCheck::checkbox("Follow newest (L)").hover_text(FOLLOW_HINT),
                );
            });

            root.col(Col::new().xs(6).md(4), move |root| {
                root.form_label(FormLabel::new("X mode"));
                root.form_select(
                    x_mode_index,
                    mode_select(X_MODE_OPTIONS, view.x_mode_placeholder),
                );
            });

            root.col(Col::new().xs(6).md(4), move |root| {
                root.form_label(FormLabel::new("Y mode"));
                root.form_select(
                    y_mode_index,
                    mode_select(Y_MODE_OPTIONS, view.y_mode_placeholder),
                );
            });

            root.col(Col::new().xs(12).md(6), move |root| {
                root.form_label(FormLabel::new("X window").hover_text(X_WINDOW_HINT));
                root.form_range(x_window, normalized_range(view.x_window_enabled));
            });

            root.col(Col::new().xs(12).md(6), move |root| {
                root.form_label(FormLabel::new("X position").hover_text(X_POSITION_HINT));
                root.form_range(x_position, normalized_range(view.x_position_enabled));
            });
        });
    });
}

fn mode_select(options: &[(AxisZoomMode, &str)], placeholder: &str) -> FormSelect {
    options.iter().fold(
        FormSelect::new().placeholder(placeholder),
        |select, (_, text)| select.option(*text),
    )
}

/// Slider over `0.0..=1.0`; `SliderScale` maps it onto the real f64 domain.
fn normalized_range(enabled: bool) -> FormRange {
    FormRange::new(0.0, 1.0)
        .step_any()
        .show_value(false)
        .fill_width(true)
        .disabled(!enabled)
}

fn mode_index(options: &[(AxisZoomMode, &str)], mode: AxisZoomMode) -> usize {
    options
        .iter()
        .position(|(candidate, _)| *candidate == mode)
        .unwrap_or(usize::MAX)
}

fn mode_label(mode: AxisZoomMode) -> &'static str {
    match mode {
        AxisZoomMode::FitAll => "Fit all",
        AxisZoomMode::FitVisible => "Fit visible",
        AxisZoomMode::Manual => "Manual",
    }
}

fn apply_x_mode(
    viewport: &mut ViewportState,
    x_mode: AxisZoomMode,
    bounds: DataBounds,
    min_margin_x: f64,
) {
    match x_mode {
        AxisZoomMode::FitAll | AxisZoomMode::FitVisible => {
            viewport.fit_x_to_data_bounds(bounds, min_margin_x);
        }
        AxisZoomMode::Manual => {
            if viewport.follow == FollowMode::PinnedToLatest {
                viewport.pin_x_to_latest(bounds.x_min, bounds.x_max, min_margin_x);
            } else {
                viewport.clamp_x_to_data_bounds(bounds, min_margin_x);
            }
        }
    }
}

fn x_window_scale(bounds: DataBounds, min_margin_x: f64) -> Option<SliderScale> {
    let data_span = (bounds.x_max - bounds.x_min).abs();
    let (min_window, max_window) = if data_span > 0.0 {
        (
            (data_span / 500.0).max(0.0001),
            data_span * 1.1 + min_margin_x * 2.0,
        )
    } else {
        (0.0001, 10.0)
    };

    SliderScale::new(min_window, max_window.max(min_window), true)
}

fn x_position_scale(
    bounds: DataBounds,
    viewport: &ViewportState,
    min_margin_x: f64,
) -> Option<SliderScale> {
    let limits = compute_x_navigation_limits(
        bounds.x_min,
        bounds.x_max,
        viewport.x_window_size,
        min_margin_x,
    )?;
    let max_start = (limits.max_x - viewport.x_window_size).max(limits.min_x);

    SliderScale::new(limits.min_x, max_start, false)
}

/// Maps an f64 domain onto the normalized `0.0..=1.0` range driven by `FormRange`.
///
/// Normalizing keeps full f64 precision for large X values (e.g. timestamps) that the
/// f32-backed `FormRange` can't represent directly.
#[derive(Clone, Copy, Debug, PartialEq)]
struct SliderScale {
    min: f64,
    max: f64,
    logarithmic: bool,
}

impl SliderScale {
    /// Returns `None` when the domain is empty or non-finite (the slider should be disabled).
    fn new(min: f64, max: f64, logarithmic: bool) -> Option<Self> {
        let (min, max) = if min <= max { (min, max) } else { (max, min) };
        let scale = Self {
            min,
            max,
            logarithmic: logarithmic && min > 0.0,
        };

        let (lo, hi) = scale.domain();
        let span = hi - lo;
        (span.is_finite() && span > f64::EPSILON).then_some(scale)
    }

    fn domain(self) -> (f64, f64) {
        if self.logarithmic {
            (self.min.ln(), self.max.ln())
        } else {
            (self.min, self.max)
        }
    }

    fn normalize(self, value: f64) -> f32 {
        let (lo, hi) = self.domain();
        let value = value.clamp(self.min, self.max);
        let value = if self.logarithmic { value.ln() } else { value };
        ((value - lo) / (hi - lo)).clamp(0.0, 1.0) as f32
    }

    fn value_at(self, normalized: f32) -> f64 {
        let (lo, hi) = self.domain();
        let value = lo + f64::from(normalized.clamp(0.0, 1.0)) * (hi - lo);
        let value = if self.logarithmic { value.exp() } else { value };
        value.clamp(self.min, self.max)
    }
}

fn y_bounds_for_mode(
    dataset: &TimeSeriesDataset,
    viewport: &ViewportState,
    mode: AxisZoomMode,
) -> Option<DataBounds> {
    match mode {
        AxisZoomMode::FitVisible => {
            compute_visible_window_bounds(dataset, viewport.x_min, viewport.x_max)
                .or_else(|| compute_visible_data_bounds(dataset))
        }
        AxisZoomMode::FitAll => compute_visible_data_bounds(dataset),
        AxisZoomMode::Manual => {
            compute_visible_window_bounds(dataset, viewport.x_min, viewport.x_max)
                .or_else(|| compute_visible_data_bounds(dataset))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_round_trips_large_values() {
        let scale = SliderScale::new(1.7e9, 1.7e9 + 3_600.0, false).expect("non-empty domain");

        let value = 1.7e9 + 900.0;
        let round_trip = scale.value_at(scale.normalize(value));
        assert!(
            (round_trip - value).abs() < 1e-2,
            "round trip was {round_trip}"
        );
    }

    #[test]
    fn logarithmic_scale_round_trips_and_clamps() {
        let scale = SliderScale::new(0.01, 100.0, true).expect("non-empty domain");
        assert!(scale.logarithmic);
        assert!((scale.normalize(1.0) - 0.5).abs() < 1e-6);

        let round_trip = scale.value_at(scale.normalize(5.0));
        assert!((round_trip - 5.0).abs() < 1e-4);

        assert_eq!(scale.normalize(1_000.0), 1.0);
        assert!((scale.value_at(-1.0) - 0.01).abs() < 1e-12);
    }

    #[test]
    fn scale_rejects_empty_domains_and_falls_back_to_linear() {
        assert!(SliderScale::new(5.0, 5.0, false).is_none());
        assert!(SliderScale::new(f64::NEG_INFINITY, 5.0, false).is_none());

        let scale = SliderScale::new(0.0, 10.0, true).expect("non-empty domain");
        assert!(!scale.logarithmic);
    }

    #[test]
    fn mode_index_uses_sentinel_for_unlisted_modes() {
        assert_eq!(mode_index(X_MODE_OPTIONS, AxisZoomMode::Manual), 1);
        assert_eq!(
            mode_index(X_MODE_OPTIONS, AxisZoomMode::FitVisible),
            usize::MAX
        );
    }
}
