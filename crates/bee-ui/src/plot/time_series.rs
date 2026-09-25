use super::{
    AxisZoomMode, DEFAULT_DECIMATION_POINTS_PER_PIXEL, FollowMode, PlotArea, PlotControlPanel,
    PlotControlPanelProps, TimeSeriesDataset, ViewportState, compute_data_bounds,
    compute_visible_data_bounds, compute_visible_window_bounds, prepare_line_points,
    viewport_point_budget,
};

#[derive(Clone, Debug, PartialEq)]
pub struct TimeSeriesPlotProps {
    pub plot: PlotArea,
    pub control_panel: PlotControlPanelProps,
    pub show_control_panel: bool,
    pub include_hidden_series: bool,
}

impl TimeSeriesPlotProps {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            plot: PlotArea::new(id),
            control_panel: PlotControlPanelProps::default(),
            show_control_panel: true,
            include_hidden_series: false,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.plot = self.plot.title(title);
        self
    }

    pub fn x_axis_label(mut self, label: impl Into<String>) -> Self {
        self.plot = self.plot.x_axis_label(label);
        self
    }

    pub fn y_axis_label(mut self, label: impl Into<String>) -> Self {
        self.plot = self.plot.y_axis_label(label);
        self
    }

    pub fn axis_labels(mut self, x_axis: impl Into<String>, y_axis: impl Into<String>) -> Self {
        self.plot = self.plot.axis_labels(x_axis, y_axis);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.plot = self.plot.width(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.plot = self.plot.height(height);
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.plot = self.plot.size(width, height);
        self
    }

    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.plot = self.plot.fill_width(fill_width);
        self
    }

    pub fn legend(mut self, legend: bool) -> Self {
        self.plot = self.plot.legend(legend);
        self
    }

    pub fn show_control_panel(mut self, show: bool) -> Self {
        self.show_control_panel = show;
        self
    }

    pub fn include_hidden_series(mut self, include_hidden_series: bool) -> Self {
        self.include_hidden_series = include_hidden_series;
        self
    }

    pub fn min_margin_x(mut self, min_margin_x: f64) -> Self {
        self.control_panel.min_margin_x = min_margin_x;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimeSeriesPlotResponse {
    pub changed: bool,
    pub has_data: bool,
    pub has_new_data: bool,
    pub control_panel_changed: bool,
}

pub struct TimeSeriesPlot;

impl TimeSeriesPlot {
    pub fn show(
        ui: &mut egui::Ui,
        props: &TimeSeriesPlotProps,
        dataset: &TimeSeriesDataset,
        viewport: &mut ViewportState,
    ) -> TimeSeriesPlotResponse {
        let mut response = TimeSeriesPlotResponse::default();
        let previous_x_max = viewport.x_max;

        let data_bounds = compute_data_bounds(dataset);
        let visible_bounds = compute_visible_data_bounds(dataset).or(data_bounds);
        response.has_data = data_bounds.is_some();
        response.has_new_data = data_bounds
            .map(|bounds| bounds.x_max > previous_x_max + 1e-9)
            .unwrap_or(false);

        viewport.apply_modes_for_dataset(dataset, props.control_panel.min_margin_x);

        if let Some(title) = &props.plot.title {
            ui.label(egui::RichText::new(title).strong());
            ui.add_space(4.0);
        }

        let (requested_x_min, requested_x_max, requested_y_min, requested_y_max) =
            viewport.normalized_bounds();

        let axis_manual = egui::Vec2b::new(
            viewport.x_mode == AxisZoomMode::Manual,
            viewport.y_mode == AxisZoomMode::Manual,
        );

        let available_width = ui.available_width();
        let plot_width = props.plot.width.unwrap_or(available_width).max(1.0);
        let point_budget = viewport_point_budget(plot_width, DEFAULT_DECIMATION_POINTS_PER_PIXEL);

        let plot_widget = props
            .plot
            .build_plot(available_width)
            .allow_zoom(axis_manual)
            .allow_scroll(axis_manual)
            .allow_drag(axis_manual)
            .allow_boxed_zoom(false)
            .allow_double_click_reset(false)
            .auto_bounds(egui::Vec2b::new(false, false));

        let plot_response = plot_widget.show(ui, |plot_ui| {
            plot_ui.set_auto_bounds(egui::Vec2b::new(false, false));
            plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                [requested_x_min, requested_y_min],
                [requested_x_max, requested_y_max],
            ));

            for series in &dataset.series {
                if !props.include_hidden_series && !series.visible {
                    continue;
                }

                let points = prepare_line_points(
                    &series.points,
                    requested_x_min,
                    requested_x_max,
                    point_budget,
                );

                if points.is_empty() {
                    continue;
                }

                plot_ui.line(egui_plot::Line::new(series.label.clone(), points));
            }
        });

        let plotted_bounds = plot_response.transform.bounds();
        let plotted_min = plotted_bounds.min();
        let plotted_max = plotted_bounds.max();

        let x_changed = !approx_eq(plotted_min[0], requested_x_min)
            || !approx_eq(plotted_max[0], requested_x_max);
        let y_changed = !approx_eq(plotted_min[1], requested_y_min)
            || !approx_eq(plotted_max[1], requested_y_max);

        if let Some(bounds) = data_bounds {
            if viewport.x_mode == AxisZoomMode::Manual && x_changed {
                viewport.set_manual_x_bounds(
                    plotted_min[0],
                    plotted_max[0],
                    bounds.x_min,
                    bounds.x_max,
                    props.control_panel.min_margin_x,
                );
                response.changed = true;
            } else {
                viewport.clamp_x_to_data_bounds(bounds, props.control_panel.min_margin_x);
            }

            if viewport.y_mode == AxisZoomMode::Manual && y_changed {
                viewport.set_manual_y_bounds(plotted_min[1], plotted_max[1]);
                response.changed = true;
            }
        }

        if viewport.x_mode == AxisZoomMode::Manual
            && viewport.follow == FollowMode::PinnedToLatest
            && plot_navigation_detected(ui.ctx(), &plot_response.response)
        {
            viewport.follow = FollowMode::Free;
            response.changed = true;
        }

        let shortcuts = collect_plot_shortcuts(
            ui.ctx(),
            plot_response.response.hovered() || plot_response.response.has_focus(),
        );

        if shortcuts.toggle_follow {
            if viewport.follow == FollowMode::PinnedToLatest {
                viewport.follow = FollowMode::Free;
            } else {
                viewport.follow = FollowMode::PinnedToLatest;
                viewport.x_mode = AxisZoomMode::Manual;
                if let Some(bounds) = data_bounds {
                    viewport.pin_x_to_latest(
                        bounds.x_min,
                        bounds.x_max,
                        props.control_panel.min_margin_x,
                    );
                }
            }
            response.changed = true;
        }

        if shortcuts.fit_x
            && let Some(bounds) = data_bounds
        {
            viewport.follow = FollowMode::Free;
            viewport.x_mode = AxisZoomMode::FitAll;
            viewport.fit_x_to_data_bounds(bounds, props.control_panel.min_margin_x);
            response.changed = true;
        }

        if shortcuts.fit_y
            && let Some(bounds) =
                y_bounds_for_mode(dataset, viewport, viewport.y_mode).or(visible_bounds)
        {
            viewport.fit_y_to_data_bounds(bounds);
            if viewport.y_mode == AxisZoomMode::Manual {
                viewport.y_mode = AxisZoomMode::FitVisible;
            }
            response.changed = true;
        }

        if shortcuts.fit_xy
            && let Some(bounds) = data_bounds
        {
            viewport.follow = FollowMode::Free;
            viewport.x_mode = AxisZoomMode::FitAll;
            viewport.fit_x_to_data_bounds(bounds, props.control_panel.min_margin_x);

            if let Some(y_bounds) =
                compute_visible_window_bounds(dataset, viewport.x_min, viewport.x_max)
                    .or(visible_bounds)
            {
                viewport.y_mode = AxisZoomMode::FitVisible;
                viewport.fit_y_to_data_bounds(y_bounds);
            }
            response.changed = true;
        }

        if props.show_control_panel {
            ui.add_space(6.0);

            let control_panel_open_id = ui
                .id()
                .with(("time_series_control_panel_open", props.plot.id.as_str()));

            let mut control_panel_open = ui
                .ctx()
                .data_mut(|data| data.get_persisted::<bool>(control_panel_open_id))
                .unwrap_or(true);

            let toggle_label = if control_panel_open {
                "Hide controls"
            } else {
                "Show controls"
            };

            if ui.small_button(toggle_label).clicked() {
                control_panel_open = !control_panel_open;
                response.changed = true;
            }

            ui.ctx()
                .data_mut(|data| data.insert_persisted(control_panel_open_id, control_panel_open));

            if control_panel_open {
                ui.add_space(4.0);

                let mut control_panel_props = props.control_panel.clone();
                control_panel_props.has_new_data =
                    control_panel_props.has_new_data || response.has_new_data;

                let control_panel_response = ui
                    .push_id(
                        ("time_series_control_panel", props.plot.id.as_str()),
                        |scoped_ui| {
                            PlotControlPanel::show(
                                scoped_ui,
                                viewport,
                                dataset,
                                &control_panel_props,
                            )
                        },
                    )
                    .inner;

                response.control_panel_changed = control_panel_response.changed;
                response.changed |= control_panel_response.changed;
                response.changed |= control_panel_response.go_live_clicked;
            }
        }

        response
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PlotShortcutActions {
    toggle_follow: bool,
    fit_x: bool,
    fit_y: bool,
    fit_xy: bool,
}

fn collect_plot_shortcuts(ctx: &egui::Context, enabled: bool) -> PlotShortcutActions {
    if !enabled {
        return PlotShortcutActions::default();
    }

    ctx.input_mut(|input| PlotShortcutActions {
        toggle_follow: input.consume_key(egui::Modifiers::NONE, egui::Key::L),
        fit_x: input.consume_key(egui::Modifiers::NONE, egui::Key::X),
        fit_y: input.consume_key(egui::Modifiers::NONE, egui::Key::Y),
        fit_xy: input.consume_key(egui::Modifiers::NONE, egui::Key::F),
    })
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

fn plot_navigation_detected(ctx: &egui::Context, response: &egui::Response) -> bool {
    if response.dragged() {
        return true;
    }

    if !response.hovered() {
        return false;
    }

    ctx.input(|input| {
        input.smooth_scroll_delta.length_sq() > 0.0
            || (input.zoom_delta() - 1.0).abs() > f32::EPSILON
    })
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9
}
