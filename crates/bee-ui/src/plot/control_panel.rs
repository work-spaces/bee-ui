use super::{
    AxisZoomMode, DEFAULT_MIN_MARGIN_X, FollowMode, TimeSeriesDataset, ViewportState,
    compute_data_bounds, compute_visible_data_bounds, compute_visible_window_bounds,
    compute_x_navigation_limits,
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
            if panel_ui
                .checkbox(&mut follow_newest, "Follow newest")
                .changed()
            {
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

            if viewport.follow == FollowMode::Free
                && props.has_new_data
                && let Some(bounds) = data_bounds
                && panel_ui.button("Go Live").clicked()
            {
                viewport.follow = FollowMode::PinnedToLatest;
                viewport.x_mode = AxisZoomMode::Manual;
                viewport.pin_x_to_latest(bounds.x_min, bounds.x_max, props.min_margin_x);
                response.changed = true;
                response.go_live_clicked = true;
            }
        });

        ui.horizontal_wrapped(|panel_ui| {
            panel_ui.label("X window");

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
                let slider =
                    egui::Slider::new(&mut next_window, min_window..=max_window.max(min_window))
                        .text("range")
                        .logarithmic(true);

                if panel_ui.add(slider).changed() {
                    viewport.follow = FollowMode::Free;
                    viewport.x_mode = AxisZoomMode::Manual;
                    viewport.set_x_window_size_manual(
                        next_window,
                        bounds.x_min,
                        bounds.x_max,
                        props.min_margin_x,
                    );
                    response.changed = true;
                }
            } else {
                let mut disabled_window = 0.0;
                panel_ui.add_enabled(
                    false,
                    egui::Slider::new(&mut disabled_window, 0.0..=1.0).text("range"),
                );
            }

            panel_ui.separator();
            panel_ui.label("X position");

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
                        let slider = egui::Slider::new(&mut next_start, limits.min_x..=max_start)
                            .text("offset");

                        if panel_ui.add(slider).changed() {
                            viewport.follow = FollowMode::Free;
                            viewport.x_mode = AxisZoomMode::Manual;
                            viewport.set_x_start_manual(
                                next_start,
                                bounds.x_min,
                                bounds.x_max,
                                props.min_margin_x,
                            );
                            response.changed = true;
                        }
                    } else {
                        panel_ui.add_enabled(
                            false,
                            egui::Slider::new(&mut viewport.x_min, limits.min_x..=limits.min_x)
                                .text("offset"),
                        );
                    }
                }
            } else {
                panel_ui.add_enabled(
                    false,
                    egui::Slider::new(&mut viewport.x_min, 0.0..=1.0).text("offset"),
                );
            }
        });

        ui.horizontal(|panel_ui| {
            if panel_ui.button("Fit X").clicked()
                && let Some(bounds) = data_bounds
            {
                viewport.follow = FollowMode::Free;
                viewport.x_mode = AxisZoomMode::FitAll;
                viewport.fit_x_to_data_bounds(bounds, props.min_margin_x);
                response.changed = true;
            }

            if panel_ui.button("Fit Y").clicked() {
                if let Some(bounds) =
                    y_bounds_for_mode(dataset, viewport, viewport.y_mode).or(visible_bounds)
                {
                    viewport.fit_y_to_data_bounds(bounds);
                    if viewport.y_mode == AxisZoomMode::Manual {
                        viewport.y_mode = AxisZoomMode::FitVisible;
                    }
                    response.changed = true;
                }
            }

            if panel_ui.button("Fit XY").clicked()
                && let Some(bounds) = data_bounds
            {
                viewport.follow = FollowMode::Free;
                viewport.x_mode = AxisZoomMode::FitAll;
                viewport.fit_x_to_data_bounds(bounds, props.min_margin_x);

                if let Some(y_bounds) =
                    compute_visible_window_bounds(dataset, viewport.x_min, viewport.x_max)
                        .or(visible_bounds)
                {
                    viewport.y_mode = AxisZoomMode::FitVisible;
                    viewport.fit_y_to_data_bounds(y_bounds);
                }

                response.changed = true;
            }
        });

        response
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
