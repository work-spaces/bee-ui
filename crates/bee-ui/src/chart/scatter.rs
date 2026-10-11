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

/// Stroke used to connect a series' points in order, e.g. to draw a route or outline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterLine {
    pub width: f32,
    pub color: egui::Color32,
}

impl ScatterLine {
    pub const fn new(width: f32, color: egui::Color32) -> Self {
        Self { width, color }
    }
}

/// A vector drawn from `origin` to `tip` in plot coordinates.
///
/// The shaft width and head length are in screen points, so the arrow keeps its
/// appearance regardless of the plot's zoom or aspect ratio. A fixed-length arrow
/// ([`Self::fixed_length`]) instead points from `origin` towards `tip` with a constant
/// on-screen length, e.g. to show a heading that stays readable at every zoom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScatterArrow {
    pub origin_x: f64,
    pub origin_y: f64,
    pub tip_x: f64,
    pub tip_y: f64,
    pub width: f32,
    pub head_length: f32,
    pub color: egui::Color32,
    /// When set, the arrow is drawn this many screen points long from the origin towards the
    /// tip, and only the origin counts towards the plot bounds.
    pub screen_length: Option<f32>,
}

impl ScatterArrow {
    pub const fn new((origin_x, origin_y): (f64, f64), (tip_x, tip_y): (f64, f64)) -> Self {
        Self {
            origin_x,
            origin_y,
            tip_x,
            tip_y,
            width: 2.0,
            head_length: 12.0,
            color: egui::Color32::from_rgb(80, 120, 255),
            screen_length: None,
        }
    }

    /// An arrow `length` screen points long, starting at `origin` and pointing along
    /// `direction` (a plot-coordinate vector, so it follows any axis scaling).
    pub fn fixed_length(
        (origin_x, origin_y): (f64, f64),
        (direction_x, direction_y): (f64, f64),
        length: f32,
    ) -> Self {
        Self {
            screen_length: Some(length),
            ..Self::new(
                (origin_x, origin_y),
                (origin_x + direction_x, origin_y + direction_y),
            )
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn head_length(mut self, head_length: f32) -> Self {
        self.head_length = head_length;
        self
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = color;
        self
    }

    fn is_finite(&self) -> bool {
        self.origin_x.is_finite()
            && self.origin_y.is_finite()
            && self.tip_x.is_finite()
            && self.tip_y.is_finite()
            && self
                .screen_length
                .is_none_or(|length| length.is_finite() && length > 0.0)
    }

    /// Plot coordinates the arrow spans; a fixed-length arrow only occupies its origin.
    fn data_endpoints(&self) -> impl Iterator<Item = (f64, f64)> {
        let tip = self
            .screen_length
            .is_none()
            .then_some((self.tip_x, self.tip_y));
        std::iter::once((self.origin_x, self.origin_y)).chain(tip)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScatterSeries {
    pub id: SeriesId,
    pub label: String,
    pub points: Vec<ScatterPoint>,
    pub visible: bool,
    /// Draws a marker for each point. Points still contribute to bounds when hidden.
    pub show_markers: bool,
    /// Connects the points in order when set.
    pub line: Option<ScatterLine>,
    pub arrows: Vec<ScatterArrow>,
}

impl ScatterSeries {
    pub fn new(id: impl Into<SeriesId>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            points: Vec::new(),
            visible: true,
            show_markers: true,
            line: None,
            arrows: Vec::new(),
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

    pub fn show_markers(mut self, show_markers: bool) -> Self {
        self.show_markers = show_markers;
        self
    }

    pub fn line(mut self, line: ScatterLine) -> Self {
        self.line = Some(line);
        self
    }

    pub fn with_arrows(mut self, arrows: Vec<ScatterArrow>) -> Self {
        self.arrows = arrows;
        self
    }

    pub fn push_point(&mut self, point: ScatterPoint) {
        self.points.push(point);
    }

    pub fn push_arrow(&mut self, arrow: ScatterArrow) {
        self.arrows.push(arrow);
    }

    /// Every plotted coordinate in the series: points plus the data-space ends of each arrow.
    fn coordinates(&self) -> impl Iterator<Item = (f64, f64)> + '_ {
        self.points
            .iter()
            .map(|point| (point.x, point.y))
            .chain(self.arrows.iter().flat_map(ScatterArrow::data_endpoints))
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
        self.series
            .iter()
            .all(|series| series.points.is_empty() && series.arrows.is_empty())
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
    compute_scatter_data_bounds_where(dataset, visible_only, |(_, y)| {
        visible_y_range.is_none_or(|(y_min, y_max)| y >= y_min && y <= y_max)
    })
}

pub(super) fn compute_scatter_y_bounds(
    dataset: &ScatterDataset,
    visible_only: bool,
    visible_x_range: Option<(f64, f64)>,
) -> Option<super::DataBounds> {
    compute_scatter_data_bounds_where(dataset, visible_only, |(x, _)| {
        visible_x_range.is_none_or(|(x_min, x_max)| x >= x_min && x <= x_max)
    })
}

fn compute_scatter_data_bounds_where(
    dataset: &ScatterDataset,
    visible_only: bool,
    mut include_point: impl FnMut((f64, f64)) -> bool,
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
        for (x, y) in series.coordinates() {
            if x.is_finite() && y.is_finite() && include_point((x, y)) {
                bounds.x_min = bounds.x_min.min(x);
                bounds.x_max = bounds.x_max.max(x);
                bounds.y_min = bounds.y_min.min(y);
                bounds.y_max = bounds.y_max.max(y);
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
    /// Disables egui_plot's own pan and zoom so the view is controlled only by the caller's
    /// viewport (or by auto-bounds when no viewport is supplied). Combined with the viewport's
    /// `lock_aspect_ratio`, the shorter axis is expanded so the requested area stays fully
    /// visible at equal scale while the plot fills its allotted size. [`Self::uniform_zoom`]
    /// and [`Self::drag_to_pan`] add navigation that edits the caller's viewport instead.
    pub lock_view: bool,
    /// Zooms both axes by the same factor about the pointer when scrolling or pinching over
    /// the plot, updating the caller's viewport so its aspect ratio is preserved. egui_plot's
    /// own zoom and scroll handling is disabled. Requires a viewport
    /// ([`ScatterPlot::show_with_viewport`]); pair it with [`Self::lock_view`] so the
    /// viewport stays authoritative.
    pub uniform_zoom: bool,
    /// Pans the caller's viewport by dragging with the primary button, keeping both spans
    /// (and so the aspect ratio) unchanged. egui_plot's own dragging is disabled. Requires a
    /// viewport; pair it with [`Self::lock_view`] like [`Self::uniform_zoom`].
    pub drag_to_pan: bool,
}

impl ScatterPlotProps {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            plot: PlotArea::new(id),
            include_hidden_series: false,
            lock_view: false,
            uniform_zoom: false,
            drag_to_pan: false,
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

    pub fn lock_view(mut self, lock_view: bool) -> Self {
        self.lock_view = lock_view;
        self
    }

    pub fn uniform_zoom(mut self, uniform_zoom: bool) -> Self {
        self.uniform_zoom = uniform_zoom;
        self
    }

    pub fn drag_to_pan(mut self, drag_to_pan: bool) -> Self {
        self.drag_to_pan = drag_to_pan;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScatterPlotResponse {
    pub changed: bool,
    pub plotted_series: usize,
    pub plotted_points: usize,
    pub plotted_arrows: usize,
    /// Bounds actually shown, after any aspect-ratio adjustment.
    pub plotted_bounds: Option<super::DataBounds>,
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
                .allow_double_click_reset(false);
            // A locked view sets its bounds every frame, so it keeps egui_plot's default
            // auto-bounds. With those, `data_aspect` expands the shorter axis instead of
            // shrinking Y, which keeps the requested area visible.
            if !props.lock_view {
                plot = plot.auto_bounds(egui::Vec2b::new(false, false));
            }
        }
        if props.lock_view {
            plot = plot
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .allow_axis_zoom_drag(false)
                .allow_boxed_zoom(false)
                .allow_double_click_reset(false);
        }
        let uniform_zoom = props.uniform_zoom && viewport.is_some();
        if uniform_zoom {
            plot = plot
                .allow_zoom(false)
                .allow_scroll(false)
                .allow_axis_zoom_drag(false)
                .allow_boxed_zoom(false);
        }
        let drag_to_pan = props.drag_to_pan && viewport.is_some();
        if drag_to_pan {
            plot = plot.allow_drag(false).allow_boxed_zoom(false);
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

                // Every item shares the series label (and therefore its plot item id), so the
                // series has one legend entry and toggling it hides markers, line, and arrows.
                let mut has_plotted_points = false;
                if let Some(line) = series.line {
                    let line_points: Vec<[f64; 2]> = series
                        .points
                        .iter()
                        .filter(|point| point.x.is_finite() && point.y.is_finite())
                        .map(|point| [point.x, point.y])
                        .collect();
                    if line_points.len() >= 2 {
                        plot_ui.line(
                            egui_plot::Line::new(series.label.clone(), line_points)
                                .width(line.width.max(0.0))
                                .color(line.color),
                        );
                        has_plotted_points = true;
                    }
                }

                for point in series.points.iter().filter(|_| series.show_markers) {
                    if !point.x.is_finite() || !point.y.is_finite() {
                        continue;
                    }

                    let radius = if point.size.is_finite() {
                        point.size.max(0.0)
                    } else {
                        0.0
                    };
                    plot_ui.points(
                        egui_plot::Points::new(series.label.clone(), vec![[point.x, point.y]])
                            .shape(point.shape.into())
                            .radius(radius)
                            .color(point.color),
                    );

                    has_plotted_points = true;
                    response.plotted_points += 1;
                }

                for arrow in series.arrows.iter().filter(|arrow| arrow.is_finite()) {
                    plot_ui.add(ArrowItem::new(series.label.clone(), arrow));
                    has_plotted_points = true;
                    response.plotted_arrows += 1;
                }

                if has_plotted_points {
                    response.plotted_series += 1;
                }
            }
        });

        if let (
            false,
            Some(viewport),
            Some((requested_x_min, requested_x_max, requested_y_min, requested_y_max)),
        ) = (props.lock_view, viewport.as_deref_mut(), requested_bounds)
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

        // Applied after showing, so the moved bounds are drawn on the next (requested) frame.
        if let Some(viewport) = viewport {
            let mut moved = false;
            if drag_to_pan
                && plot_response
                    .response
                    .dragged_by(egui::PointerButton::Primary)
            {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                let drag = plot_response.response.drag_delta();
                let [dx_per_point, dy_per_point] = plot_response.transform.dvalue_dpos();
                // Content follows the pointer, so the view moves the opposite way.
                moved |= viewport.pan_by(
                    -f64::from(drag.x) * dx_per_point,
                    -f64::from(drag.y) * dy_per_point,
                );
            }
            if uniform_zoom
                && let Some((factor, pointer)) = uniform_zoom_input(ui, &plot_response.response)
            {
                let center = plot_response.transform.value_from_position(pointer);
                moved |= viewport.zoom_about(factor, (center.x, center.y));
            }
            if moved {
                response.changed = true;
                ui.ctx().request_repaint();
            }
        }

        let plotted_bounds = plot_response.transform.bounds();
        response.plotted_bounds = Some(super::DataBounds {
            x_min: plotted_bounds.min()[0],
            x_max: plotted_bounds.max()[0],
            y_min: plotted_bounds.min()[1],
            y_max: plotted_bounds.max()[1],
        });
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

/// Zoom factor (above one zooms in) and pointer position while the pointer is over the plot.
/// Pinch and zoom-modifier scrolling arrive as egui's zoom delta; plain vertical scrolling
/// zooms at egui's scroll zoom speed and is consumed so enclosing scroll areas stay put.
fn uniform_zoom_input(ui: &egui::Ui, response: &egui::Response) -> Option<(f64, egui::Pos2)> {
    if !response.contains_pointer() {
        return None;
    }
    let pointer = ui.input(|input| input.pointer.hover_pos())?;
    let (zoom_delta, scroll_delta) =
        ui.input(|input| (input.zoom_delta(), input.smooth_scroll_delta));
    if scroll_delta != egui::Vec2::ZERO {
        ui.input_mut(|input| input.smooth_scroll_delta = egui::Vec2::ZERO);
    }
    let scroll_zoom_speed = ui
        .ctx()
        .options(|options| options.input_options.scroll_zoom_speed);
    let factor = f64::from(zoom_delta) * f64::from(scroll_zoom_speed * scroll_delta.y).exp();
    (factor.is_finite() && factor > 0.0 && !approx_eq(factor, 1.0)).then_some((factor, pointer))
}

/// Plot item for a [`ScatterArrow`]. egui_plot's built-in arrows use a fixed 1 px stroke,
/// so this draws a configurable shaft and a filled screen-space head instead.
struct ArrowItem {
    base: egui_plot::PlotItemBase,
    endpoints: [egui_plot::PlotPoint; 2],
    screen_length: Option<f32>,
    width: f32,
    head_length: f32,
    color: egui::Color32,
}

impl ArrowItem {
    fn new(name: String, arrow: &ScatterArrow) -> Self {
        Self {
            base: egui_plot::PlotItemBase::new(name),
            endpoints: [
                egui_plot::PlotPoint::new(arrow.origin_x, arrow.origin_y),
                egui_plot::PlotPoint::new(arrow.tip_x, arrow.tip_y),
            ],
            screen_length: arrow.screen_length,
            width: arrow.width,
            head_length: arrow.head_length,
            color: arrow.color,
        }
    }

    /// Plot-space points the arrow occupies; a fixed-length tip has no data position.
    fn data_endpoints(&self) -> &[egui_plot::PlotPoint] {
        if self.screen_length.is_some() {
            &self.endpoints[..1]
        } else {
            &self.endpoints
        }
    }
}

impl egui_plot::PlotItem for ArrowItem {
    fn shapes(
        &self,
        _ui: &egui::Ui,
        transform: &egui_plot::PlotTransform,
        shapes: &mut Vec<egui::Shape>,
    ) {
        let width = if self.highlighted() {
            self.width + 1.0
        } else {
            self.width
        };
        let (origin, tip) = screen_endpoints(
            transform.position_from_point(&self.endpoints[0]),
            transform.position_from_point(&self.endpoints[1]),
            self.screen_length,
        );
        shapes.extend(arrow_shapes(
            origin,
            tip,
            width,
            self.head_length,
            self.color,
        ));
    }

    fn initialize(&mut self, _x_range: std::ops::RangeInclusive<f64>) {}

    fn color(&self) -> egui::Color32 {
        self.color
    }

    fn geometry(&self) -> egui_plot::PlotGeometry<'_> {
        egui_plot::PlotGeometry::Points(self.data_endpoints())
    }

    fn bounds(&self) -> egui_plot::PlotBounds {
        let mut bounds = egui_plot::PlotBounds::NOTHING;
        for point in self.data_endpoints() {
            bounds.extend_with(point);
        }
        bounds
    }

    fn base(&self) -> &egui_plot::PlotItemBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut egui_plot::PlotItemBase {
        &mut self.base
    }
}

/// Screen positions of an arrow's ends. A fixed-length arrow keeps the direction from the
/// origin to the tip but is rescaled to `screen_length` points; a zero direction stays
/// degenerate so nothing is drawn.
fn screen_endpoints(
    origin: egui::Pos2,
    tip: egui::Pos2,
    screen_length: Option<f32>,
) -> (egui::Pos2, egui::Pos2) {
    let Some(length) = screen_length else {
        return (origin, tip);
    };
    let vector = tip - origin;
    let current = vector.length();
    if !current.is_finite() || current <= f32::EPSILON {
        return (origin, origin);
    }
    (origin, origin + vector * (length / current))
}

/// Screen-space shapes for an arrow: a shaft ending at the head's base and a filled head.
/// The head is shortened to fit arrows shorter than the configured head length.
fn arrow_shapes(
    origin: egui::Pos2,
    tip: egui::Pos2,
    width: f32,
    head_length: f32,
    color: egui::Color32,
) -> Vec<egui::Shape> {
    let vector = tip - origin;
    let length = vector.length();
    if !length.is_finite() || length <= f32::EPSILON {
        return Vec::new();
    }

    let direction = vector / length;
    let head_length = if head_length.is_finite() {
        head_length.clamp(0.0, length)
    } else {
        0.0
    };
    let head_base = tip - direction * head_length;
    let head_half_width = head_length * 0.5;
    let normal = direction.rot90() * head_half_width;
    let width = if width.is_finite() {
        width.max(0.0)
    } else {
        0.0
    };

    let mut shapes = vec![egui::Shape::line_segment(
        [origin, head_base],
        egui::Stroke::new(width, color),
    )];
    if head_length > 0.0 {
        shapes.push(egui::Shape::convex_polygon(
            vec![tip, head_base + normal, head_base - normal],
            color,
            egui::Stroke::NONE,
        ));
    }
    shapes
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

    #[test]
    fn arrows_count_as_data_and_extend_bounds() {
        let series = ScatterSeries::new("vector", "Vector")
            .with_arrows(vec![ScatterArrow::new((1.0, 2.0), (-3.0, 8.0))]);
        let dataset = ScatterDataset::new(vec![series]);

        assert!(!dataset.is_empty());
        let bounds = compute_scatter_data_bounds(&dataset, false).expect("arrow has bounds");
        assert_eq!((bounds.x_min, bounds.x_max), (-3.0, 1.0));
        assert_eq!((bounds.y_min, bounds.y_max), (2.0, 8.0));
    }

    #[test]
    fn fixed_length_arrows_only_bound_their_origin() {
        let arrow = ScatterArrow::fixed_length((1.0, 2.0), (0.0, 1_000.0), 40.0);
        assert_eq!(arrow.screen_length, Some(40.0));
        assert_eq!((arrow.tip_x, arrow.tip_y), (1.0, 1_002.0));
        assert!(arrow.is_finite());

        let dataset = ScatterDataset::new(vec![
            ScatterSeries::new("heading", "Heading").with_arrows(vec![arrow]),
        ]);
        let bounds = compute_scatter_data_bounds(&dataset, false).expect("origin has bounds");
        assert_eq!((bounds.x_min, bounds.x_max), (1.0, 1.0));
        assert_eq!((bounds.y_min, bounds.y_max), (2.0, 2.0));

        for length in [0.0, -5.0, f32::NAN, f32::INFINITY] {
            assert!(!ScatterArrow::fixed_length((0.0, 0.0), (1.0, 0.0), length).is_finite());
        }
    }

    #[test]
    fn fixed_length_arrows_keep_their_direction_at_a_constant_screen_length() {
        let origin = egui::pos2(10.0, 20.0);
        let close = |a: egui::Pos2, b: egui::Pos2| (a - b).length() < 1e-4;

        let (start, tip) = screen_endpoints(origin, egui::pos2(13.0, 24.0), Some(50.0));
        assert_eq!(start, origin);
        assert!(close(tip, egui::pos2(40.0, 60.0)), "{tip:?}");

        let (_, tip) = screen_endpoints(origin, egui::pos2(310.0, 420.0), Some(50.0));
        assert!(close(tip, egui::pos2(40.0, 60.0)), "{tip:?}");

        assert_eq!(
            screen_endpoints(origin, egui::pos2(13.0, 24.0), None),
            (origin, egui::pos2(13.0, 24.0))
        );
        assert_eq!(
            screen_endpoints(origin, origin, Some(50.0)),
            (origin, origin)
        );
    }

    #[test]
    fn series_defaults_draw_markers_without_lines_or_arrows() {
        let series = ScatterSeries::new("route", "Route");
        assert!(series.show_markers);
        assert!(series.line.is_none());
        assert!(series.arrows.is_empty());

        let line = ScatterLine::new(3.0, egui::Color32::RED);
        let series = series.line(line).show_markers(false);
        assert_eq!(series.line, Some(line));
        assert!(!series.show_markers);
        assert!(!ScatterPlotProps::new("plot").lock_view);
        assert!(ScatterPlotProps::new("plot").lock_view(true).lock_view);
    }

    #[test]
    fn arrow_shapes_end_the_shaft_at_the_head_and_point_at_the_tip() {
        let origin = egui::pos2(0.0, 0.0);
        let tip = egui::pos2(100.0, 0.0);
        let shapes = arrow_shapes(origin, tip, 2.0, 10.0, egui::Color32::RED);

        assert_eq!(shapes.len(), 2);
        let egui::Shape::LineSegment { points, stroke } = &shapes[0] else {
            panic!("expected the shaft first");
        };
        assert_eq!(*points, [origin, egui::pos2(90.0, 0.0)]);
        assert_eq!(stroke.width, 2.0);
        let egui::Shape::Path(head) = &shapes[1] else {
            panic!("expected a filled head");
        };
        assert_eq!(head.points[0], tip);
        assert!(head.points[1..].iter().all(|point| point.x == 90.0));
    }

    #[test]
    fn arrow_heads_shrink_to_fit_short_arrows_and_skip_zero_length() {
        let shapes = arrow_shapes(
            egui::pos2(0.0, 0.0),
            egui::pos2(0.0, 4.0),
            2.0,
            10.0,
            egui::Color32::RED,
        );
        let egui::Shape::Path(head) = &shapes[1] else {
            panic!("expected a filled head");
        };
        assert!(head.points[1..].iter().all(|point| point.y == 0.0));

        let point = egui::pos2(5.0, 5.0);
        assert!(arrow_shapes(point, point, 2.0, 10.0, egui::Color32::RED).is_empty());
    }

    fn square_viewport() -> crate::chart::ViewportState {
        crate::chart::ViewportState {
            x_min: -10.0,
            x_max: 10.0,
            y_min: -10.0,
            y_max: 10.0,
            x_mode: crate::chart::AxisZoomMode::Manual,
            y_mode: crate::chart::AxisZoomMode::Manual,
            follow: crate::chart::FollowMode::Free,
            x_window_size: 20.0,
            lock_aspect_ratio: true,
        }
    }

    /// Runs one egui frame per entry of `frames`, feeding that entry's events, and returns the
    /// plotted bounds of every frame. The viewport persists across frames like caller state.
    fn run_plot_frames(
        props: &ScatterPlotProps,
        viewport: &mut crate::chart::ViewportState,
        frames: Vec<Vec<egui::Event>>,
    ) -> Vec<DataBounds> {
        let ctx = egui::Context::default();
        let dataset = ScatterDataset::new(vec![
            ScatterSeries::new("points", "Points").with_points(vec![ScatterPoint::new(1.0, 1.0)]),
        ]);
        let mut plotted = Vec::new();
        for events in frames {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_000.0, 1_000.0),
                )),
                events,
                ..egui::RawInput::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let response = ScatterPlot::show_with_viewport(ui, props, &dataset, viewport);
                    plotted.push(response.plotted_bounds.expect("plot was shown"));
                });
            });
        }
        plotted
    }

    fn show_locked_square(width: f32, height: f32) -> DataBounds {
        let props = ScatterPlotProps::new("locked")
            .size(width, height)
            .lock_view(true);
        let mut viewport = square_viewport();
        let plotted = run_plot_frames(&props, &mut viewport, vec![Vec::new(), Vec::new()]);
        assert_eq!(viewport, square_viewport(), "a locked view never moves");
        plotted[1]
    }

    #[test]
    fn locked_equal_aspect_views_expand_the_short_axis_to_fill_the_plot() {
        let close = |a: f64, b: f64| (a - b).abs() < 1e-6;

        let wide = show_locked_square(600.0, 300.0);
        assert!(
            close(wide.y_min, -10.0) && close(wide.y_max, 10.0),
            "{wide:?}"
        );
        assert!(wide.x_min < -10.0 && wide.x_max > 10.0, "{wide:?}");
        assert!(close(wide.x_min, -wide.x_max), "{wide:?}");

        let tall = show_locked_square(300.0, 600.0);
        assert!(
            close(tall.x_min, -10.0) && close(tall.x_max, 10.0),
            "{tall:?}"
        );
        assert!(tall.y_min < -10.0 && tall.y_max > 10.0, "{tall:?}");
        assert!(close(tall.y_min, -tall.y_max), "{tall:?}");
    }

    #[test]
    fn uniform_zoom_scales_both_axes_about_the_pointer() {
        let close = |a: f64, b: f64| (a - b).abs() < 1e-6;
        let props = ScatterPlotProps::new("zoom")
            .size(600.0, 300.0)
            .lock_view(true)
            .uniform_zoom(true);
        let mut viewport = square_viewport();
        let pointer = egui::Event::PointerMoved(egui::pos2(150.0, 100.0));
        let plotted = run_plot_frames(
            &props,
            &mut viewport,
            vec![
                Vec::new(),
                vec![pointer, egui::Event::Zoom(2.0)],
                Vec::new(),
            ],
        );
        // The first frame settles axis widths; the zoom applies after the second is drawn.
        let (before, after) = (plotted[1], plotted[2]);

        assert!(close(viewport.x_span(), 10.0), "{viewport:?}");
        assert!(close(viewport.y_span(), 10.0), "{viewport:?}");
        assert!(viewport.lock_aspect_ratio);
        // Both axes halve, so the aspect ratio is unchanged and the plot stays filled.
        assert!(close(after.x_span() * 2.0, before.x_span()), "{after:?}");
        assert!(close(after.y_span() * 2.0, before.y_span()), "{after:?}");
        // Halving about a fixed point c maps each edge e to c + (e - c) / 2, so c = 2e' - e;
        // both edges of an axis agree on the point that stayed under the pointer.
        let fixed_x = 2.0 * after.x_min - before.x_min;
        let fixed_y = 2.0 * after.y_min - before.y_min;
        assert!(close(fixed_x, 2.0 * after.x_max - before.x_max));
        assert!(close(fixed_y, 2.0 * after.y_max - before.y_max));
        assert!(
            fixed_x < 0.0 && fixed_y > 0.0,
            "pointer is in the upper left"
        );
    }

    #[test]
    fn uniform_zoom_uses_plain_scrolling_and_ignores_input_outside_the_plot() {
        let props = ScatterPlotProps::new("scroll")
            .size(600.0, 300.0)
            .lock_view(true)
            .uniform_zoom(true);
        let scroll = |y: f32| egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, y),
            modifiers: egui::Modifiers::NONE,
        };

        let mut viewport = square_viewport();
        let outside = egui::Event::PointerMoved(egui::pos2(900.0, 900.0));
        run_plot_frames(
            &props,
            &mut viewport,
            vec![
                Vec::new(),
                vec![outside, egui::Event::Zoom(2.0), scroll(100.0)],
            ],
        );
        assert_eq!(viewport, square_viewport());

        let inside = egui::Event::PointerMoved(egui::pos2(300.0, 150.0));
        let mut frames = vec![Vec::new(), vec![inside.clone(), scroll(100.0)]];
        frames.extend((0..10).map(|_| Vec::new()));
        run_plot_frames(&props, &mut viewport, frames);
        assert!(
            viewport.x_span() < 20.0,
            "scrolling up zooms in: {viewport:?}"
        );
        assert!((viewport.x_span() - viewport.y_span()).abs() < 1e-9);

        let unzoomed = ScatterPlotProps::new("plain")
            .size(600.0, 300.0)
            .lock_view(true);
        let mut viewport = square_viewport();
        run_plot_frames(
            &unzoomed,
            &mut viewport,
            vec![Vec::new(), vec![inside, egui::Event::Zoom(2.0)]],
        );
        assert_eq!(viewport, square_viewport());
    }

    /// Presses at `start`, drags in `steps` moves of `step` points, then releases.
    fn drag_frames(start: egui::Pos2, step: egui::Vec2, steps: usize) -> Vec<Vec<egui::Event>> {
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let end = start + step * steps as f32;
        let mut frames = vec![
            Vec::new(),
            vec![egui::Event::PointerMoved(start), button(start, true)],
        ];
        frames
            .extend((1..=steps).map(|i| vec![egui::Event::PointerMoved(start + step * i as f32)]));
        frames.push(vec![button(end, false)]);
        frames.push(Vec::new());
        frames
    }

    #[test]
    fn drag_to_pan_moves_the_view_with_the_pointer_and_keeps_the_scale() {
        let close = |a: f64, b: f64| (a - b).abs() < 1e-6;
        let props = ScatterPlotProps::new("pan")
            .size(600.0, 300.0)
            .lock_view(true)
            .drag_to_pan(true);
        let mut viewport = square_viewport();
        let plotted = run_plot_frames(
            &props,
            &mut viewport,
            drag_frames(egui::pos2(300.0, 150.0), egui::vec2(10.0, 5.0), 10),
        );
        let (before, after) = (plotted[1], plotted[plotted.len() - 1]);
        let center = |bounds: DataBounds| {
            (
                (bounds.x_min + bounds.x_max) * 0.5,
                (bounds.y_min + bounds.y_max) * 0.5,
            )
        };

        assert!(close(viewport.x_span(), 20.0) && close(viewport.y_span(), 20.0));
        // Y is the requested (short) axis of the wide plot; X is widened to fill the frame,
        // whose width varies slightly with the tick label widths as the view moves.
        assert!(close(after.y_span(), before.y_span()), "{after:?}");
        // Dragging right and down moves the content with the pointer: the view goes left and up.
        let shift_x = center(after).0 - center(before).0;
        let shift_y = center(after).1 - center(before).1;
        assert!(shift_x < 0.0 && shift_y > 0.0, "{shift_x}, {shift_y}");
        // Equal scale on both axes, so the 2:1 drag in points is a 2:1 shift in data units.
        assert!(close(-shift_x, 2.0 * shift_y), "{shift_x}, {shift_y}");

        let locked = ScatterPlotProps::new("no_pan")
            .size(600.0, 300.0)
            .lock_view(true);
        let mut viewport = square_viewport();
        run_plot_frames(
            &locked,
            &mut viewport,
            drag_frames(egui::pos2(300.0, 150.0), egui::vec2(10.0, 5.0), 10),
        );
        assert_eq!(viewport, square_viewport());
    }
}
