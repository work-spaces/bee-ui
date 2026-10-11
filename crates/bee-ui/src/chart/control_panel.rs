use super::{
    AxisZoomMode, DEFAULT_MIN_MARGIN_X, DataBounds, FollowMode, TimeSeriesDataset, ViewportState,
    compute_data_bounds, compute_visible_data_bounds, compute_visible_window_bounds,
    compute_x_navigation_limits,
};
use crate::{
    UiRoot,
    components::{
        Button, Col, Container, DisclosureState, Dropdown, DropdownItem, FormRange, HStack, Row,
    },
    prelude, show_in_ui,
    theme::bootstrap_tokens,
};

const Y_MODE_OPTIONS: &[(AxisZoomMode, &str)] = &[
    (AxisZoomMode::FitVisible, "Fit "),
    (AxisZoomMode::FitAll, "All "),
    (AxisZoomMode::Manual, "Free"),
];
const SCATTER_MODE_OPTIONS: &[(AxisZoomMode, &str)] = &[
    (AxisZoomMode::FitVisible, "Fit"),
    (AxisZoomMode::FitAll, "All"),
    (AxisZoomMode::Manual, "Free"),
];

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
    /// The Download button was clicked this pass. The panel doesn't export anything itself;
    /// the host decides what to download.
    pub download_clicked: bool,
}

pub struct TimeSeriesControlPanel;

pub struct ScatterControlPanel;

impl ScatterControlPanel {
    pub fn show(
        ui: &mut egui::Ui,
        viewport: &mut ViewportState,
        dataset: &super::ScatterDataset,
    ) -> PlotControlPanelResponse {
        let mut response = PlotControlPanelResponse::default();
        let all_bounds = super::scatter::compute_scatter_data_bounds(dataset, false);
        let (x_min, x_max, y_min, y_max) = viewport.normalized_bounds();
        let visible_x_bounds =
            super::scatter::compute_scatter_x_bounds(dataset, true, Some((y_min, y_max)));
        let visible_y_bounds =
            super::scatter::compute_scatter_y_bounds(dataset, true, Some((x_min, x_max)));
        if viewport.x_mode != AxisZoomMode::Manual
            && let Some(bounds) =
                scatter_bounds_for_mode(viewport.x_mode, visible_x_bounds, all_bounds)
        {
            viewport.fit_x_to_data_bounds(
                super::scatter::padded_scatter_bounds(bounds),
                DEFAULT_MIN_MARGIN_X,
            );
        }
        if viewport.y_mode != AxisZoomMode::Manual
            && let Some(bounds) =
                scatter_bounds_for_mode(viewport.y_mode, visible_y_bounds, all_bounds)
        {
            viewport.fit_y_to_data_bounds(super::scatter::padded_scatter_bounds(bounds));
        }

        let (x_min, x_max, y_min, y_max) = viewport.normalized_bounds();
        let visible_x_bounds =
            super::scatter::compute_scatter_x_bounds(dataset, true, Some((y_min, y_max)));
        let visible_y_bounds =
            super::scatter::compute_scatter_y_bounds(dataset, true, Some((x_min, x_max)));
        let x_bounds = scatter_bounds_for_mode(viewport.x_mode, visible_x_bounds, all_bounds);
        let y_bounds = scatter_bounds_for_mode(viewport.y_mode, visible_y_bounds, all_bounds);

        let x_zoom_scale = x_bounds
            .and_then(|bounds| axis_window_scale(bounds.x_min, bounds.x_max, DEFAULT_MIN_MARGIN_X));
        let x_range_scale = x_bounds.and_then(|bounds| {
            axis_position_scale(
                bounds.x_min,
                bounds.x_max,
                viewport.x_window_size,
                DEFAULT_MIN_MARGIN_X,
            )
        });
        let y_zoom_scale =
            y_bounds.and_then(|bounds| axis_window_scale(bounds.y_min, bounds.y_max, 0.0));
        let y_range_scale = y_bounds.and_then(|bounds| {
            axis_position_scale(bounds.y_min, bounds.y_max, viewport.y_span(), 0.0)
        });

        let initial = ScatterControlValues {
            x_mode_index: mode_index(SCATTER_MODE_OPTIONS, viewport.x_mode),
            y_mode_index: mode_index(SCATTER_MODE_OPTIONS, viewport.y_mode),
            x_zoom: x_zoom_scale.map_or(0.0, |scale| scale.normalize(viewport.x_window_size)),
            x_range: x_range_scale.map_or(0.0, |scale| scale.normalize(viewport.x_min)),
            y_zoom: y_zoom_scale.map_or(0.0, |scale| scale.normalize(viewport.y_span())),
            y_range: y_range_scale.map_or(0.0, |scale| scale.normalize(viewport.y_min)),
        };
        let visibility_id = ui.next_auto_id().with("scatter_controls_visible");
        let is_visible = ui
            .ctx()
            .data(|data| data.get_temp::<bool>(visibility_id))
            .unwrap_or(true);
        let view = ScatterPanelView {
            is_visible,
            x_mode_placeholder: mode_label(viewport.x_mode),
            y_mode_placeholder: mode_label(viewport.y_mode),
            lock_aspect_ratio: viewport.lock_aspect_ratio,
            x_mode_dropdown_id: ui.next_auto_id().with("scatter_x_mode_dropdown"),
            y_mode_dropdown_id: ui.next_auto_id().with("scatter_y_mode_dropdown"),
            x_zoom_enabled: x_zoom_scale.is_some(),
            x_range_enabled: x_range_scale.is_some(),
            y_zoom_enabled: y_zoom_scale.is_some(),
            y_range_enabled: y_range_scale.is_some(),
        };

        let mut edited = initial;
        let clicks = show_in_ui(ui, |root| build_scatter_panel(root, &mut edited, view));
        if clicks.visibility_toggled {
            ui.ctx()
                .data_mut(|data| data.insert_temp(visibility_id, !is_visible));
            ui.ctx().request_repaint();
        }
        if clicks.aspect_ratio_toggled {
            viewport.lock_aspect_ratio = !viewport.lock_aspect_ratio;
            if viewport.lock_aspect_ratio {
                viewport.x_mode = AxisZoomMode::Manual;
                viewport.y_mode = AxisZoomMode::Manual;
                viewport.follow = FollowMode::Free;
            }
            response.changed = true;
        }

        if edited.x_mode_index != initial.x_mode_index
            && let Some(&(mode, _)) = SCATTER_MODE_OPTIONS.get(edited.x_mode_index)
        {
            viewport.x_mode = mode;
            if viewport.lock_aspect_ratio {
                viewport.y_mode = mode;
            }
            if mode == AxisZoomMode::Manual {
                viewport.follow = FollowMode::Free;
            } else {
                if let Some(bounds) = scatter_bounds_for_mode(mode, visible_x_bounds, all_bounds) {
                    viewport.fit_x_to_data_bounds(
                        super::scatter::padded_scatter_bounds(bounds),
                        DEFAULT_MIN_MARGIN_X,
                    );
                }
                if viewport.lock_aspect_ratio
                    && let Some(bounds) =
                        scatter_bounds_for_mode(mode, visible_y_bounds, all_bounds)
                {
                    viewport.fit_y_to_data_bounds(super::scatter::padded_scatter_bounds(bounds));
                }
            }
            response.changed = true;
        }

        if edited.y_mode_index != initial.y_mode_index
            && let Some(&(mode, _)) = SCATTER_MODE_OPTIONS.get(edited.y_mode_index)
        {
            viewport.y_mode = mode;
            if viewport.lock_aspect_ratio {
                viewport.x_mode = mode;
            }
            if mode == AxisZoomMode::Manual {
                if viewport.lock_aspect_ratio {
                    viewport.follow = FollowMode::Free;
                }
            } else {
                if let Some(bounds) = scatter_bounds_for_mode(mode, visible_y_bounds, all_bounds) {
                    viewport.fit_y_to_data_bounds(super::scatter::padded_scatter_bounds(bounds));
                }
                if viewport.lock_aspect_ratio
                    && let Some(bounds) =
                        scatter_bounds_for_mode(mode, visible_x_bounds, all_bounds)
                {
                    viewport.fit_x_to_data_bounds(
                        super::scatter::padded_scatter_bounds(bounds),
                        DEFAULT_MIN_MARGIN_X,
                    );
                }
            }
            response.changed = true;
        }

        if edited.x_zoom != initial.x_zoom
            && let (Some(scale), Some(bounds)) = (x_zoom_scale, x_bounds)
        {
            let window_size = scale.value_at(edited.x_zoom);
            if viewport.lock_aspect_ratio {
                let current_x_span = viewport.x_window_size;
                let zoom_factor = if current_x_span.is_finite() && current_x_span > 0.0 {
                    window_size / current_x_span
                } else {
                    1.0
                };
                let y_span = viewport.y_span() * zoom_factor;
                let y_center = viewport.y_min * 0.5 + viewport.y_max * 0.5;
                viewport.set_manual_y_bounds(y_center - y_span * 0.5, y_center + y_span * 0.5);
            }
            viewport.x_mode = AxisZoomMode::Manual;
            viewport.follow = FollowMode::Free;
            response.changed |= viewport.set_x_window_size_manual(
                window_size,
                bounds.x_min,
                bounds.x_max,
                DEFAULT_MIN_MARGIN_X,
            );
            response.changed = true;
        }

        if edited.x_range != initial.x_range
            && let (Some(scale), Some(bounds)) = (x_range_scale, x_bounds)
        {
            viewport.x_mode = AxisZoomMode::Manual;
            viewport.follow = FollowMode::Free;
            response.changed |= viewport.set_x_start_manual(
                scale.value_at(edited.x_range),
                bounds.x_min,
                bounds.x_max,
                DEFAULT_MIN_MARGIN_X,
            );
            response.changed = true;
        }

        if edited.y_zoom != initial.y_zoom
            && let Some(scale) = y_zoom_scale
        {
            let window_size = scale.value_at(edited.y_zoom);
            let current_y_span = viewport.y_span();
            let zoom_factor = if current_y_span.is_finite() && current_y_span > 0.0 {
                window_size / current_y_span
            } else {
                1.0
            };
            let center = viewport.y_min * 0.5 + viewport.y_max * 0.5;
            if viewport.lock_aspect_ratio
                && let Some(bounds) = x_bounds
            {
                viewport.x_mode = AxisZoomMode::Manual;
                viewport.follow = FollowMode::Free;
                response.changed |= viewport.set_x_window_size_manual(
                    viewport.x_window_size * zoom_factor,
                    bounds.x_min,
                    bounds.x_max,
                    DEFAULT_MIN_MARGIN_X,
                );
            }
            viewport.set_manual_y_bounds(center - window_size * 0.5, center + window_size * 0.5);
            response.changed = true;
        }

        if edited.y_range != initial.y_range
            && let Some(scale) = y_range_scale
        {
            let y_min = scale.value_at(edited.y_range);
            let y_span = viewport.y_span();
            viewport.set_manual_y_bounds(y_min, y_min + y_span);
            response.changed = true;
        }

        response
    }
}

impl TimeSeriesControlPanel {
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

        let visibility_id = ui.next_auto_id().with("visible");
        let is_visible = ui
            .ctx()
            .data(|data| data.get_temp::<bool>(visibility_id))
            .unwrap_or(true);

        let following = viewport.follow == FollowMode::PinnedToLatest;
        let initial = ControlValues {
            y_mode_index: mode_index(Y_MODE_OPTIONS, viewport.y_mode),
            x_window: window_scale.map_or(0.0, |scale| scale.normalize(viewport.x_window_size)),
            x_position: match position_scale {
                Some(_) if following => X_POSITION_FOLLOW,
                Some(scale) => scale.normalize(viewport.x_min),
                None => 0.0,
            },
        };

        let view = PanelView {
            is_visible,
            y_mode_placeholder: mode_label(viewport.y_mode),
            y_mode_dropdown_id: ui.next_auto_id().with("y_mode_dropdown"),
            x_window_enabled: window_scale.is_some(),
            x_position_enabled: position_scale.is_some(),
        };

        let mut edited = initial;
        let clicks = show_in_ui(ui, |root| build_panel(root, &mut edited, view));

        if clicks.visibility_toggled {
            ui.ctx()
                .data_mut(|data| data.insert_temp(visibility_id, !is_visible));
            ui.ctx().request_repaint();
        }

        response.download_clicked = clicks.download;

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
            if edited.x_position >= X_POSITION_FOLLOW {
                viewport.follow = FollowMode::PinnedToLatest;
                viewport.pin_x_to_latest(bounds.x_min, bounds.x_max, props.min_margin_x);
                response.changed = true;
            } else {
                response.changed |= viewport.set_x_start_manual(
                    scale.value_at(edited.x_position),
                    bounds.x_min,
                    bounds.x_max,
                    props.min_margin_x,
                );
            }
        }

        response
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ScatterControlValues {
    x_mode_index: usize,
    y_mode_index: usize,
    x_zoom: f32,
    x_range: f32,
    y_zoom: f32,
    y_range: f32,
}

#[derive(Clone, Copy, Debug)]
struct ScatterPanelView {
    is_visible: bool,
    lock_aspect_ratio: bool,
    x_mode_placeholder: &'static str,
    y_mode_placeholder: &'static str,
    x_mode_dropdown_id: egui::Id,
    y_mode_dropdown_id: egui::Id,
    x_zoom_enabled: bool,
    x_range_enabled: bool,
    y_zoom_enabled: bool,
    y_range_enabled: bool,
}

/// Controlled values bound to the panel's inputs. The panel diffs these against their initial
/// snapshot after rendering to detect user edits.
#[derive(Clone, Copy, Debug, PartialEq)]
struct ControlValues {
    y_mode_index: usize,
    /// Normalized (`0.0..=1.0`) position on the X window slider.
    x_window: f32,
    /// Normalized (`0.0..=1.0`) position on the X position slider. `X_POSITION_FOLLOW`
    /// (fully right) means the viewport follows the newest sample.
    x_position: f32,
}

/// Normalized X position slider value that pins the viewport to the latest sample.
const X_POSITION_FOLLOW: f32 = 1.0;

/// Read-only presentation state for the panel.
#[derive(Clone, Copy, Debug)]
struct PanelView {
    is_visible: bool,
    y_mode_placeholder: &'static str,
    y_mode_dropdown_id: egui::Id,
    x_window_enabled: bool,
    x_position_enabled: bool,
}

/// Button clicks from one pass of the panel.
#[derive(Clone, Copy, Debug, Default)]
struct PanelClicks {
    visibility_toggled: bool,
    aspect_ratio_toggled: bool,
    download: bool,
}

fn build_scatter_panel(
    root: &mut UiRoot<'_>,
    values: &mut ScatterControlValues,
    view: ScatterPanelView,
) -> PanelClicks {
    let spacing = bootstrap_tokens::spacing();
    let row = Row::new().gutter_x(spacing.md).gutter_y(spacing.sm);
    let mut clicks = PanelClicks::default();

    root.container(Container::fluid(), |root| {
        root.row(row, |root| {
            root.col(Col::new().xs(12).md(12), |root| {
                root.hstack(HStack::new().gap(spacing.md), |root| {
                    let button_icon = if view.is_visible {
                        prelude::SvgIcon::EyeSlash
                    } else {
                        prelude::SvgIcon::List
                    };
                    clicks.visibility_toggled = root
                        .button(
                            Button::new("")
                                .icon(button_icon)
                                .variant(prelude::Variant::Primary),
                        )
                        .clicked();

                    if view.is_visible {
                        let (icon, hover_text) = if view.lock_aspect_ratio {
                            (
                                prelude::SvgIcon::AspectRatioFill,
                                "Unlock the scatter plot's 1:1 X/Y aspect ratio",
                            )
                        } else {
                            (
                                prelude::SvgIcon::AspectRatio,
                                "Lock the scatter plot to a 1:1 X/Y aspect ratio",
                            )
                        };
                        clicks.aspect_ratio_toggled = root
                            .button(
                                Button::new("")
                                    .icon(icon)
                                    .variant(prelude::Variant::Secondary)
                                    .hover_text(hover_text),
                            )
                            .clicked();

                        if let Some(selected) = root
                            .dropdown(
                                DisclosureState::uncontrolled(view.x_mode_dropdown_id, false),
                                mode_dropdown(
                                    SCATTER_MODE_OPTIONS,
                                    values.x_mode_index,
                                    view.x_mode_placeholder,
                                ),
                            )
                            .clicked_index()
                        {
                            values.x_mode_index = selected;
                        }

                        if let Some(selected) = root
                            .dropdown(
                                DisclosureState::uncontrolled(view.y_mode_dropdown_id, false),
                                mode_dropdown(
                                    SCATTER_MODE_OPTIONS,
                                    values.y_mode_index,
                                    view.y_mode_placeholder,
                                ),
                            )
                            .clicked_index()
                        {
                            values.y_mode_index = selected;
                        }
                    }
                });
            });
        });

        if view.is_visible {
            root.row(row, |root| {
                root.col(Col::new().xs(12).md(12), |root| {
                    slider_pair(
                        root,
                        prelude::SvgIcon::Arrows,
                        (&mut values.x_zoom, view.x_zoom_enabled),
                        (&mut values.y_zoom, view.y_zoom_enabled),
                    );
                });
            });
            root.row(row, |root| {
                root.col(Col::new().xs(12).md(12), |root| {
                    slider_pair(
                        root,
                        prelude::SvgIcon::InputCursor,
                        (&mut values.x_range, view.x_range_enabled),
                        (&mut values.y_range, view.y_range_enabled),
                    );
                });
            });
        }
    });
    clicks
}

/// Draws an icon label followed by X and Y sliders that split the remaining width evenly.
fn slider_pair(
    root: &mut UiRoot<'_>,
    icon: prelude::SvgIcon,
    (x_value, x_enabled): (&mut f32, bool),
    (y_value, y_enabled): (&mut f32, bool),
) {
    let gap = bootstrap_tokens::spacing().md;
    root.hstack(HStack::new().gap(gap), |root| {
        root.label(prelude::Label::new(prelude::RichText::new().icon(icon)));
        let slider_width = root.raw(|ui| ((ui.available_width() - gap) * 0.5).max(0.0));
        for (value, enabled) in [(x_value, x_enabled), (y_value, y_enabled)] {
            root.form_range(
                value,
                normalized_range(enabled)
                    .fill_width(false)
                    .width(slider_width)
                    .variant(prelude::Variant::Secondary),
            );
        }
    });
}

/// Draws the time-series panel, writing input edits into `values` and returning button clicks.
fn build_panel(root: &mut UiRoot<'_>, values: &mut ControlValues, view: PanelView) -> PanelClicks {
    let spacing = bootstrap_tokens::spacing();
    let row = Row::new().gutter_x(spacing.md).gutter_y(spacing.sm);
    let mut clicks = PanelClicks::default();

    root.container(Container::fluid(), |root| {
        root.row(row, |root| {
            root.col(Col::new().xs(12).md(12), |root| {
                root.hstack(HStack::new().gap(spacing.md), |root| {
                    let button_icon = if view.is_visible {
                        prelude::SvgIcon::EyeSlash
                    } else {
                        prelude::SvgIcon::List
                    };
                    clicks.visibility_toggled = root
                        .button(
                            Button::new("")
                                .icon(button_icon)
                                .variant(prelude::Variant::Primary),
                        )
                        .clicked();

                    if view.is_visible {
                        if let Some(index) = root
                            .dropdown(
                                DisclosureState::uncontrolled(view.y_mode_dropdown_id, false),
                                mode_dropdown(
                                    Y_MODE_OPTIONS,
                                    values.y_mode_index,
                                    view.y_mode_placeholder,
                                ),
                            )
                            .clicked_index()
                        {
                            values.y_mode_index = index;
                        }

                        clicks.download = root
                            .button(
                                Button::new("")
                                    .icon(prelude::SvgIcon::Download)
                                    .variant(prelude::Variant::Secondary),
                            )
                            .clicked();
                    }
                });
            });

            if view.is_visible {
                root.col(Col::new().xs(12).md(12), |root| {
                    let position_variant = if values.x_position >= X_POSITION_FOLLOW {
                        prelude::Variant::Info
                    } else {
                        prelude::Variant::Secondary
                    };

                    root.hstack(HStack::new().gap(spacing.md), |root| {
                        root.label(prelude::Label::new(
                            prelude::RichText::new().icon(prelude::SvgIcon::Arrows),
                        ));
                        root.form_range(
                            &mut values.x_window,
                            normalized_range(view.x_window_enabled)
                                .variant(prelude::Variant::Primary),
                        );
                    });
                    root.hstack(HStack::new().gap(spacing.md), |root| {
                        root.label(prelude::Label::new(
                            prelude::RichText::new().icon(prelude::SvgIcon::InputCursor),
                        ));
                        root.form_range(
                            &mut values.x_position,
                            normalized_range(view.x_position_enabled).variant(position_variant),
                        );
                    });
                });
            }
        });
    });

    clicks
}

fn mode_dropdown(options: &[(AxisZoomMode, &str)], selected: usize, placeholder: &str) -> Dropdown {
    let label = options.get(selected).map_or(placeholder, |(_, text)| *text);
    options.iter().enumerate().fold(
        Dropdown::new(label).variant(prelude::Variant::Secondary),
        |dropdown, (index, (_, text))| {
            dropdown.item(DropdownItem::new(*text).active(index == selected))
        },
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

fn scatter_bounds_for_mode(
    mode: AxisZoomMode,
    visible_bounds: Option<DataBounds>,
    all_bounds: Option<DataBounds>,
) -> Option<DataBounds> {
    match mode {
        AxisZoomMode::FitVisible => visible_bounds.or(all_bounds),
        AxisZoomMode::FitAll | AxisZoomMode::Manual => all_bounds,
    }
}

fn x_window_scale(bounds: DataBounds, min_margin_x: f64) -> Option<SliderScale> {
    axis_window_scale(bounds.x_min, bounds.x_max, min_margin_x)
}

fn axis_window_scale(data_min: f64, data_max: f64, min_margin: f64) -> Option<SliderScale> {
    let data_span = (data_max - data_min).abs();
    let (min_window, max_window) = if data_span > 0.0 {
        (
            (data_span / 500.0).max(0.0001),
            data_span * 1.1 + min_margin * 2.0,
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
    axis_position_scale(
        bounds.x_min,
        bounds.x_max,
        viewport.x_window_size,
        min_margin_x,
    )
}

fn axis_position_scale(
    data_min: f64,
    data_max: f64,
    window_size: f64,
    min_margin: f64,
) -> Option<SliderScale> {
    let limits = compute_x_navigation_limits(data_min, data_max, window_size, min_margin)?;
    let max_start = (data_max - window_size).max(limits.min_x);

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
    fn manual_scatter_controls_use_stable_full_data_bounds() {
        let visible_bounds = DataBounds {
            x_min: 2.0,
            x_max: 4.0,
            y_min: 3.0,
            y_max: 5.0,
        };
        let all_bounds = DataBounds {
            x_min: 0.0,
            x_max: 10.0,
            y_min: -5.0,
            y_max: 15.0,
        };

        assert_eq!(
            scatter_bounds_for_mode(AxisZoomMode::Manual, Some(visible_bounds), Some(all_bounds),),
            Some(all_bounds),
        );
    }

    #[test]
    fn axis_scales_map_y_zoom_and_range_values() {
        let zoom_scale = axis_window_scale(-10.0, 10.0, 0.0).expect("non-empty Y domain");
        let zoom = 4.0;
        let zoom_round_trip = zoom_scale.value_at(zoom_scale.normalize(zoom));
        assert!((zoom_round_trip - zoom).abs() < 1e-6);

        let range_scale =
            axis_position_scale(-10.0, 10.0, zoom, 0.0).expect("non-empty Y range domain");
        let range = -2.0;
        let range_round_trip = range_scale.value_at(range_scale.normalize(range));
        assert!((range_round_trip - range).abs() < 1e-6);
    }

    #[test]
    fn position_scale_right_end_matches_pinned_window() {
        let bounds = DataBounds {
            x_min: 0.0,
            x_max: 100.0,
            y_min: 0.0,
            y_max: 1.0,
        };
        let mut viewport = ViewportState {
            x_mode: AxisZoomMode::Manual,
            follow: FollowMode::PinnedToLatest,
            x_window_size: 20.0,
            ..ViewportState::default()
        };
        viewport.pin_x_to_latest(bounds.x_min, bounds.x_max, DEFAULT_MIN_MARGIN_X);

        let scale =
            x_position_scale(bounds, &viewport, DEFAULT_MIN_MARGIN_X).expect("non-empty domain");
        assert_eq!(scale.normalize(viewport.x_min), X_POSITION_FOLLOW);
        assert!((scale.value_at(X_POSITION_FOLLOW) - viewport.x_min).abs() < 1e-9);
    }
}
