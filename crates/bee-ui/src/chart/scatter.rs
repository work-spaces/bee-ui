use super::{PlotArea, SeriesId};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ScatterShape {
    #[default]
    Circle,
    Square,
    Diamond,
    Cross,
    Plus,
}

impl From<ScatterShape> for egui_plot::MarkerShape {
    fn from(shape: ScatterShape) -> Self {
        match shape {
            ScatterShape::Circle => Self::Circle,
            ScatterShape::Square => Self::Square,
            ScatterShape::Diamond => Self::Diamond,
            ScatterShape::Cross => Self::Cross,
            ScatterShape::Plus => Self::Plus,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub shape: ScatterShape,
    pub size: f32,
    pub color: egui::Color32,
}

impl ScatterPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            shape: ScatterShape::Circle,
            size: 3.5,
            color: egui::Color32::from_rgb(80, 120, 255),
        }
    }

    pub fn shape(mut self, shape: ScatterShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = color;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScatterSeries {
    pub id: SeriesId,
    pub label: String,
    pub points: Vec<ScatterPoint>,
    pub visible: bool,
}

impl ScatterSeries {
    pub fn new(id: impl Into<SeriesId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            points: Vec::new(),
            visible: true,
        }
    }

    pub fn with_points(mut self, points: Vec<ScatterPoint>) -> Self {
        self.points = points;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn push_point(&mut self, point: ScatterPoint) {
        self.points.push(point);
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScatterDataset {
    pub series: Vec<ScatterSeries>,
}

impl ScatterDataset {
    pub fn new(series: Vec<ScatterSeries>) -> Self {
        Self { series }
    }

    pub fn is_empty(&self) -> bool {
        self.series.iter().all(|series| series.points.is_empty())
    }

    pub fn push_series(&mut self, series: ScatterSeries) {
        self.series.push(series);
    }
}

pub(super) fn compute_scatter_data_bounds(
    dataset: &ScatterDataset,
    visible_only: bool,
) -> Option<super::DataBounds> {
    compute_scatter_data_bounds_where(dataset, visible_only, |_| true)
}

pub(super) fn compute_scatter_x_bounds(
    dataset: &ScatterDataset,
    visible_only: bool,
    visible_y_range: Option<(f64, f64)>,
) -> Option<super::DataBounds> {
    compute_scatter_data_bounds_where(dataset, visible_only, |point| {
        visible_y_range.map_or(true, |(y_min, y_max)| point.y >= y_min && point.y <= y_max)
    })
}

pub(super) fn compute_scatter_y_bounds(
    dataset: &ScatterDataset,
    visible_only: bool,
    visible_x_range: Option<(f64, f64)>,
) -> Option<super::DataBounds> {
    compute_scatter_data_bounds_where(dataset, visible_only, |point| {
        visible_x_range.map_or(true, |(x_min, x_max)| point.x >= x_min && point.x <= x_max)
    })
}

fn compute_scatter_data_bounds_where(
    dataset: &ScatterDataset,
    visible_only: bool,
    mut include_point: impl FnMut(&ScatterPoint) -> bool,
) -> Option<super::DataBounds> {
    let mut bounds = super::DataBounds {
        x_min: f64::INFINITY,
        x_max: f64::NEG_INFINITY,
        y_min: f64::INFINITY,
        y_max: f64::NEG_INFINITY,
    };

    for series in &dataset.series {
        if visible_only && !series.visible {
            continue;
        }
        for point in &series.points {
            if point.x.is_finite() && point.y.is_finite() && include_point(point) {
                bounds.x_min = bounds.x_min.min(point.x);
                bounds.x_max = bounds.x_max.max(point.x);
                bounds.y_min = bounds.y_min.min(point.y);
                bounds.y_max = bounds.y_max.max(point.y);
            }
        }
    }

    (bounds.x_min.is_finite()
        && bounds.x_max.is_finite()
        && bounds.y_min.is_finite()
        && bounds.y_max.is_finite())
    .then_some(bounds)
}

const SCATTER_FIT_MARGIN: f64 = 0.05;

pub(super) fn padded_scatter_bounds(bounds: super::DataBounds) -> super::DataBounds {
    fn pad_range(min: f64, max: f64) -> (f64, f64) {
        let span = max - min;
        let margin = if span == 0.0 {
            min.abs().max(1.0) * SCATTER_FIT_MARGIN
        } else {
            span.abs() * SCATTER_FIT_MARGIN
        };
        let padded_min = min - margin;
        let padded_max = max + margin;
        (
            if padded_min.is_finite() {
                padded_min
            } else {
                min
            },
            if padded_max.is_finite() {
                padded_max
            } else {
                max
            },
        )
    }

    let (x_min, x_max) = pad_range(bounds.x_min, bounds.x_max);
    let (y_min, y_max) = pad_range(bounds.y_min, bounds.y_max);
    super::DataBounds {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScatterPlotProps {
    pub plot: PlotArea,
    pub include_hidden_series: bool,
}

impl ScatterPlotProps {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            plot: PlotArea::new(id),
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

    pub fn include_hidden_series(mut self, include_hidden_series: bool) -> Self {
        self.include_hidden_series = include_hidden_series;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScatterPlotResponse {
    pub changed: bool,
    pub plotted_series: usize,
    pub plotted_points: usize,
}

pub struct ScatterPlot;

impl ScatterPlot {
    pub fn show(
        ui: &mut egui::Ui,
        props: &ScatterPlotProps,
        dataset: &ScatterDataset,
    ) -> ScatterPlotResponse {
        Self::show_internal(ui, props, dataset, None)
    }

    pub fn show_with_viewport(
        ui: &mut egui::Ui,
        props: &ScatterPlotProps,
        dataset: &ScatterDataset,
        viewport: &mut super::ViewportState,
    ) -> ScatterPlotResponse {
        Self::show_internal(ui, props, dataset, Some(viewport))
    }

    fn show_internal(
        ui: &mut egui::Ui,
        props: &ScatterPlotProps,
        dataset: &ScatterDataset,
        mut viewport: Option<&mut super::ViewportState>,
    ) -> ScatterPlotResponse {
        let mut response = ScatterPlotResponse::default();
        let all_bounds = compute_scatter_data_bounds(dataset, false);
        let visible_bounds = compute_scatter_data_bounds(dataset, true);
        let displayed_bounds = if props.include_hidden_series {
            all_bounds
        } else {
            visible_bounds
        };

        if let Some(viewport) = viewport.as_deref_mut() {
            let (x_min, x_max, y_min, y_max) = viewport.normalized_bounds();
            let visible_x_bounds = compute_scatter_x_bounds(dataset, true, Some((y_min, y_max)));
            let visible_y_bounds = compute_scatter_y_bounds(dataset, true, Some((x_min, x_max)));

            if viewport.x_mode != super::AxisZoomMode::Manual
                && let Some(bounds) = bounds_for_mode(viewport.x_mode, visible_x_bounds, all_bounds)
            {
                viewport.fit_x_to_data_bounds(
                    padded_scatter_bounds(bounds),
                    super::DEFAULT_MIN_MARGIN_X,
                );
            }
            if viewport.y_mode != super::AxisZoomMode::Manual
                && let Some(bounds) = bounds_for_mode(viewport.y_mode, visible_y_bounds, all_bounds)
            {
                viewport.fit_y_to_data_bounds(padded_scatter_bounds(bounds));
            }
        }

        if let Some(title) = &props.plot.title {
            ui.label(egui::RichText::new(title).strong());
            ui.add_space(4.0);
        }

        let requested_bounds = viewport
            .as_deref()
            .map(super::ViewportState::normalized_bounds);
        let axis_manual = viewport.as_deref().map(|viewport| {
            egui::Vec2b::new(
                viewport.x_mode == super::AxisZoomMode::Manual,
                viewport.y_mode == super::AxisZoomMode::Manual,
            )
        });
        let lock_aspect_ratio = viewport
            .as_deref()
            .is_some_and(|viewport| viewport.lock_aspect_ratio);

        let mut plot = props.plot.build_plot(ui.available_width());
        if lock_aspect_ratio {
            plot = plot.data_aspect(1.0);
        }
        if let Some(axis_manual) = axis_manual {
            plot = plot
                .allow_zoom(axis_manual)
                .allow_drag(axis_manual)
                .allow_boxed_zoom(false)
                .allow_double_click_reset(false)
                .auto_bounds(egui::Vec2b::new(false, false));
        }

        let plot_response = plot.show(ui, |plot_ui| {
            if let Some((x_min, x_max, y_min, y_max)) = requested_bounds {
                plot_ui.set_auto_bounds(egui::Vec2b::new(false, false));
                plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                    [x_min, y_min],
                    [x_max, y_max],
                ));
            }

            for series in &dataset.series {
                if !props.include_hidden_series && !series.visible {
                    continue;
                }

                let mut has_plotted_points = false;
                for point in &series.points {
                    if !point.x.is_finite() || !point.y.is_finite() {
                        continue;
                    }

                    let radius = if point.size.is_finite() {
                        point.size.max(0.0)
                    } else {
                        0.0
                    };
                    let name = if has_plotted_points {
                        String::new()
                    } else {
                        series.label.clone()
                    };
                    plot_ui.points(
                        egui_plot::Points::new(name, vec![[point.x, point.y]])
                            .shape(point.shape.into())
                            .radius(radius)
                            .color(point.color),
                    );

                    has_plotted_points = true;
                    response.plotted_points += 1;
                }

                if has_plotted_points {
                    response.plotted_series += 1;
                }
            }
        });

        if let (
            Some(viewport),
            Some((requested_x_min, requested_x_max, requested_y_min, requested_y_max)),
        ) = (viewport, requested_bounds)
        {
            let plotted_bounds = plot_response.transform.bounds();
            let plotted_min = plotted_bounds.min();
            let plotted_max = plotted_bounds.max();
            if let Some(bounds) = displayed_bounds {
                if viewport.x_mode == super::AxisZoomMode::Manual
                    && (!approx_eq(plotted_min[0], requested_x_min)
                        || !approx_eq(plotted_max[0], requested_x_max))
                {
                    viewport.set_manual_x_bounds(
                        plotted_min[0],
                        plotted_max[0],
                        bounds.x_min,
                        bounds.x_max,
                        super::DEFAULT_MIN_MARGIN_X,
                    );
                    response.changed = true;
                }
                if viewport.y_mode == super::AxisZoomMode::Manual
                    && (!approx_eq(plotted_min[1], requested_y_min)
                        || !approx_eq(plotted_max[1], requested_y_max))
                {
                    viewport.set_manual_y_bounds_clamped(
                        plotted_min[1],
                        plotted_max[1],
                        bounds.y_min,
                        bounds.y_max,
                        super::DEFAULT_MIN_MARGIN_X,
                    );
                    response.changed = true;
                }
            }
        }

        response.changed |= plot_response.response.changed();
        response
    }
}

fn bounds_for_mode(
    mode: super::AxisZoomMode,
    visible_bounds: Option<super::DataBounds>,
    all_bounds: Option<super::DataBounds>,
) -> Option<super::DataBounds> {
    match mode {
        super::AxisZoomMode::FitVisible => visible_bounds,
        super::AxisZoomMode::FitAll => all_bounds,
        super::AxisZoomMode::Manual => None,
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-9
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::DataBounds;

    #[test]
    fn fit_bounds_only_include_points_visible_on_the_opposite_axis() {
        let dataset =
            ScatterDataset::new(vec![ScatterSeries::new("visible", "Visible").with_points(
                vec![
                    ScatterPoint::new(1.0, 1.0),
                    ScatterPoint::new(5.0, 5.0),
                    ScatterPoint::new(9.0, 9.0),
                ],
            )]);

        let visible_x = compute_scatter_x_bounds(&dataset, true, Some((0.0, 6.0)))
            .expect("points are visible in the y window");
        let visible_y = compute_scatter_y_bounds(&dataset, true, Some((0.0, 6.0)))
            .expect("points are visible in the x window");
        let all = compute_scatter_data_bounds(&dataset, false).expect("dataset has points");

        assert_eq!((visible_x.x_min, visible_x.x_max), (1.0, 5.0));
        assert_eq!((visible_y.y_min, visible_y.y_max), (1.0, 5.0));
        assert_eq!((all.x_min, all.x_max), (1.0, 9.0));
        assert_eq!((all.y_min, all.y_max), (1.0, 9.0));
    }

    #[test]
    fn fit_bounds_add_five_percent_padding_and_handle_single_points() {
        let bounds = super::padded_scatter_bounds(DataBounds {
            x_min: 1.0,
            x_max: 9.0,
            y_min: -10.0,
            y_max: 10.0,
        });
        assert_eq!((bounds.x_min, bounds.x_max), (0.6, 9.4));
        assert_eq!((bounds.y_min, bounds.y_max), (-11.0, 11.0));

        let point_bounds = super::padded_scatter_bounds(DataBounds {
            x_min: 0.0,
            x_max: 0.0,
            y_min: 2.0,
            y_max: 2.0,
        });
        assert!(point_bounds.x_min < 0.0 && point_bounds.x_max > 0.0);
        assert!(point_bounds.y_min < 2.0 && point_bounds.y_max > 2.0);
    }
}
