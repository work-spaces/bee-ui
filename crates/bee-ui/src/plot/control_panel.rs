use super::{
    AxisZoomMode, DEFAULT_MIN_MARGIN_X, FollowMode, TimeSeriesDataset, ViewportState,
    compute_data_bounds, compute_visible_data_bounds, compute_visible_window_bounds,
    compute_x_navigation_limits,
};
use crate::{
    components::FormValidationState,
    core::{UiTheme, Variant},
};

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

        let id_salt = ui.id().with("plot_control_panel");

        ui.horizontal_wrapped(|panel_ui| {
            let mut follow_newest = viewport.follow == FollowMode::PinnedToLatest;
            let follow_response = panel_ui
                .checkbox(&mut follow_newest, "Follow newest (L)")
                .on_hover_text(
                    "Pin the right edge to the latest sample. Toggle with keyboard shortcut `L`.",
                );
            if follow_response.changed() {
                viewport.follow = if follow_newest {
                    FollowMode::PinnedToLatest
                } else {
                    FollowMode::Free
                };

                if follow_newest {
                    viewport.x_mode = AxisZoomMode::Manual;
                    if let Some(bounds) = data_bounds {
                        viewport.pin_x_to_latest(bounds.x_min, bounds.x_max, props.min_margin_x);
                    }
                }

                response.changed = true;
            }

            panel_ui.separator();
            panel_ui.label("X mode");

            let mut x_mode = viewport.x_mode;
            egui::ComboBox::from_id_salt(id_salt.with("x_mode"))
                .selected_text(x_mode_label(x_mode))
                .show_ui(panel_ui, |combo_ui| {
                    combo_ui.selectable_value(&mut x_mode, AxisZoomMode::FitAll, "Fit all");
                    combo_ui.selectable_value(&mut x_mode, AxisZoomMode::Manual, "Manual");
                });

            if x_mode != viewport.x_mode {
                viewport.x_mode = x_mode;
                if let Some(bounds) = data_bounds {
                    match x_mode {
                        AxisZoomMode::FitAll | AxisZoomMode::FitVisible => {
                            viewport.fit_x_to_data_bounds(bounds, props.min_margin_x);
                        }
                        AxisZoomMode::Manual => {
                            if viewport.follow == FollowMode::PinnedToLatest {
                                viewport.pin_x_to_latest(
                                    bounds.x_min,
                                    bounds.x_max,
                                    props.min_margin_x,
                                );
                            } else {
                                viewport.clamp_x_to_data_bounds(bounds, props.min_margin_x);
                            }
                        }
                    }
                }
                response.changed = true;
            }

            panel_ui.separator();
            panel_ui.label("Y mode");

            let mut y_mode = viewport.y_mode;
            egui::ComboBox::from_id_salt(id_salt.with("y_mode"))
                .selected_text(y_mode_label(y_mode))
                .show_ui(panel_ui, |combo_ui| {
                    combo_ui.selectable_value(&mut y_mode, AxisZoomMode::FitVisible, "Fit visible");
                    combo_ui.selectable_value(&mut y_mode, AxisZoomMode::FitAll, "Fit all");
                    combo_ui.selectable_value(&mut y_mode, AxisZoomMode::Manual, "Manual");
                });

            if y_mode != viewport.y_mode {
                viewport.y_mode = y_mode;
                if let Some(bounds) =
                    y_bounds_for_mode(dataset, viewport, y_mode).or(visible_bounds)
                {
                    viewport.fit_y_to_data_bounds(bounds);
                }
                response.changed = true;
            }
        });

        ui.add_space(4.0);

        ui.horizontal(|panel_ui| {
            panel_ui.label("X window");
            let slider_width = panel_ui.available_width().max(1.0);

            if let Some(bounds) = data_bounds {
                let data_span = (bounds.x_max - bounds.x_min).abs();
                let min_window = if data_span > 0.0 {
                    (data_span / 500.0).max(0.0001)
                } else {
                    0.0001
                };
                let max_window = if data_span > 0.0 {
                    data_span * 1.1 + props.min_margin_x * 2.0
                } else {
                    10.0
                };

                let mut next_window = viewport
                    .x_window_size
                    .clamp(min_window, max_window.max(min_window));

                if add_bee_form_slider(
                    panel_ui,
                    &mut next_window,
                    min_window..=max_window.max(min_window),
                    BeeSliderOptions {
                        logarithmic: true,
                        enabled: true,
                        desired_width: Some(slider_width),
                    },
                )
                .on_hover_text("Visible X span. Changing this keeps live pin if currently pinned.")
                .changed()
                {
                    viewport.x_mode = AxisZoomMode::Manual;
                    response.changed |= viewport.set_x_window_size_preserving_pin(
                        next_window,
                        bounds.x_min,
                        bounds.x_max,
                        props.min_margin_x,
                    );
                }
            } else {
                let mut disabled_window = 0.0;
                let _ = add_bee_form_slider(
                    panel_ui,
                    &mut disabled_window,
                    0.0..=1.0,
                    BeeSliderOptions {
                        logarithmic: true,
                        enabled: false,
                        desired_width: Some(slider_width),
                    },
                );
            }
        });

        ui.add_space(6.0);

        ui.horizontal(|panel_ui| {
            panel_ui.label("X position");
            let slider_width = panel_ui.available_width().max(1.0);

            if let Some(bounds) = data_bounds {
                if let Some(limits) = compute_x_navigation_limits(
                    bounds.x_min,
                    bounds.x_max,
                    viewport.x_window_size,
                    props.min_margin_x,
                ) {
                    let max_start = (limits.max_x - viewport.x_window_size).max(limits.min_x);

                    if max_start > limits.min_x {
                        let mut next_start = viewport.x_min.clamp(limits.min_x, max_start);

                        if add_bee_form_slider(
                            panel_ui,
                            &mut next_start,
                            limits.min_x..=max_start,
                            BeeSliderOptions {
                                logarithmic: false,
                                enabled: true,
                                desired_width: Some(slider_width),
                            },
                        )
                        .on_hover_text("Pan horizontally within clamped data bounds. Changing this unpins follow mode.")
                        .changed()
                        {
                            viewport.x_mode = AxisZoomMode::Manual;
                            response.changed |= viewport.set_x_start_manual(
                                next_start,
                                bounds.x_min,
                                bounds.x_max,
                                props.min_margin_x,
                            );
                        }
                    } else {
                        let mut disabled_offset = viewport.x_min;
                        let _ = add_bee_form_slider(
                            panel_ui,
                            &mut disabled_offset,
                            limits.min_x..=limits.min_x,
                            BeeSliderOptions {
                                logarithmic: false,
                                enabled: false,
                                desired_width: Some(slider_width),
                            },
                        );
                    }
                }
            } else {
                let mut disabled_offset = viewport.x_min;
                let _ = add_bee_form_slider(
                    panel_ui,
                    &mut disabled_offset,
                    0.0..=1.0,
                    BeeSliderOptions {
                        logarithmic: false,
                        enabled: false,
                        desired_width: Some(slider_width),
                    },
                );
            }
        });

        response
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BeeSliderOptions {
    logarithmic: bool,
    enabled: bool,
    desired_width: Option<f32>,
}

fn add_bee_form_slider(
    ui: &mut egui::Ui,
    value: &mut f64,
    range: std::ops::RangeInclusive<f64>,
    options: BeeSliderOptions,
) -> egui::Response {
    ui.scope(|scope_ui| {
        apply_bee_form_range_visuals(scope_ui, !options.enabled);

        let mut slider = egui::Slider::new(value, range).show_value(false);
        if options.logarithmic {
            slider = slider.logarithmic(true);
        }

        let width = options
            .desired_width
            .unwrap_or_else(|| scope_ui.available_width().max(1.0))
            .max(1.0);
        scope_ui
            .add_enabled_ui(options.enabled, |enabled_ui| {
                enabled_ui.add_sized(egui::vec2(width, 0.0), slider)
            })
            .inner
    })
    .inner
}

fn apply_bee_form_range_visuals(ui: &mut egui::Ui, disabled: bool) {
    let mut theme = UiTheme::default();
    theme.sync_mode_from_dark_mode(matches!(ui.ctx().theme(), egui::Theme::Dark));

    let style =
        theme.resolve_form_range_style(Variant::Primary, FormValidationState::None, disabled);
    let tone = theme.resolve_variant(Variant::Primary);
    let accent = tone.fill;

    {
        let spacing = ui.spacing_mut();
        spacing.item_spacing.x = style.label_gap;
        spacing.interact_size.y = (spacing.interact_size.y * 0.6).max(8.0);
    }

    {
        let visuals = ui.visuals_mut();
        let rounded = egui::CornerRadius::same(theme.radius.pill);
        let border = egui::Stroke::new(1.0_f32, tone.border);

        visuals.handle_shape = egui::style::HandleShape::Circle;

        visuals.widgets.inactive.bg_fill = tone.bg_subtle;
        visuals.widgets.inactive.weak_bg_fill = tone.bg_subtle;
        visuals.widgets.inactive.bg_stroke = border;
        visuals.widgets.inactive.fg_stroke.color = style.text;
        visuals.widgets.inactive.corner_radius = rounded;

        visuals.widgets.hovered.bg_fill = accent;
        visuals.widgets.hovered.weak_bg_fill = tone.bg_subtle;
        visuals.widgets.hovered.bg_stroke = border;
        visuals.widgets.hovered.fg_stroke.color = style.text;
        visuals.widgets.hovered.corner_radius = rounded;

        visuals.widgets.active.bg_fill = accent;
        visuals.widgets.active.weak_bg_fill = tone.bg_subtle;
        visuals.widgets.active.bg_stroke = border;
        visuals.widgets.active.fg_stroke.color = style.text;
        visuals.widgets.active.corner_radius = rounded;

        visuals.widgets.open.bg_fill = accent;
        visuals.widgets.open.weak_bg_fill = tone.bg_subtle;
        visuals.widgets.open.bg_stroke = border;
        visuals.widgets.open.fg_stroke.color = style.text;
        visuals.widgets.open.corner_radius = rounded;

        visuals.widgets.noninteractive.bg_fill = tone.bg_subtle;
        visuals.widgets.noninteractive.weak_bg_fill = tone.bg_subtle;
        visuals.widgets.noninteractive.bg_stroke = border;
        visuals.widgets.noninteractive.fg_stroke.color = style.disabled_text;
        visuals.widgets.noninteractive.corner_radius = rounded;
    }
}

fn x_mode_label(mode: AxisZoomMode) -> &'static str {
    match mode {
        AxisZoomMode::FitAll => "Fit all",
        AxisZoomMode::FitVisible => "Fit visible",
        AxisZoomMode::Manual => "Manual",
    }
}

fn y_mode_label(mode: AxisZoomMode) -> &'static str {
    match mode {
        AxisZoomMode::FitVisible => "Fit visible",
        AxisZoomMode::FitAll => "Fit all",
        AxisZoomMode::Manual => "Manual",
    }
}

fn y_bounds_for_mode(
    dataset: &TimeSeriesDataset,
    viewport: &ViewportState,
    mode: AxisZoomMode,
) -> Option<super::DataBounds> {
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
