use std::sync::{Arc, Mutex};

use egui::{Color32, Frame, Id, Pos2, Rect, Sense, Stroke, Ui, UiBuilder, Vec2, Widget};
use walkers::{HttpTiles, Map as WalkersMap, MapMemory, sources::OpenStreetMap};

pub mod control_panel;

pub use control_panel::MapControlPanel;

pub use walkers::{Position, Projector, lat_lon, lon_lat};

const OSM_COPYRIGHT_URL: &str = "https://www.openstreetmap.org/copyright";

/// An interactive OpenStreetMap widget.
///
/// Map state and downloaded tiles are retained between frames for each `id`. The default tile
/// source is OpenStreetMap, so applications using this widget should have an internet connection
/// and preserve the attribution displayed on the map.
pub struct Map {
    id: Id,
    center: Position,
    height: f32,
    fill_height: bool,
    initial_zoom: Option<f64>,
    routes: Vec<MapRoute>,
}

/// A geographic route to draw over a [`Map`].
///
/// Route points are projected on every frame, so the line follows map panning and zooming.
pub struct MapRoute {
    points: Vec<Position>,
    stroke: Stroke,
    interpolation: MapRouteInterpolation,
}

/// How consecutive points in a [`MapRoute`] are connected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MapRouteInterpolation {
    /// Connect points with straight segments in projected map space.
    #[default]
    Line,
    /// Follow the shortest great-circle arc between points on the globe.
    Arc,
}

impl MapRoute {
    /// Creates a route from its ordered geographic points.
    pub fn new(points: impl IntoIterator<Item = Position>) -> Self {
        Self {
            points: points.into_iter().collect(),
            stroke: Stroke::new(3.0, Color32::from_rgb(40, 120, 240)),
            interpolation: MapRouteInterpolation::default(),
        }
    }

    /// Sets how consecutive geographic points are connected.
    ///
    /// Defaults to [`MapRouteInterpolation::Line`].
    pub fn interpolation(mut self, interpolation: MapRouteInterpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Sets the route line color.
    pub fn color(mut self, color: Color32) -> Self {
        self.stroke.color = color;
        self
    }

    /// Sets the route line width in points.
    pub fn width(mut self, width: f32) -> Self {
        self.stroke.width = if width.is_finite() {
            width.max(0.0)
        } else {
            0.0
        };
        self
    }
}

impl Map {
    /// Creates a map centered at the given longitude/latitude position.
    ///
    /// Use [`lon_lat`] to construct the position, for example `Map::new("site", lon_lat(-122.4,
    /// 37.8))`.
    pub fn new(id: impl std::hash::Hash, center: Position) -> Self {
        Self {
            id: Id::new(id),
            center,
            height: 280.0,
            fill_height: false,
            initial_zoom: None,
            routes: Vec::new(),
        }
    }

    /// Adds a geographic route to draw over the map.
    pub fn route(mut self, route: MapRoute) -> Self {
        self.routes.push(route);
        self
    }

    /// Adds multiple geographic routes to draw over the map.
    pub fn routes(mut self, routes: impl IntoIterator<Item = MapRoute>) -> Self {
        self.routes.extend(routes);
        self
    }

    /// Sets the map viewport height in points. The default is 280 points.
    pub fn height(mut self, height: f32) -> Self {
        self.height = if height.is_finite() {
            height.max(0.0)
        } else {
            0.0
        };
        self.fill_height = false;
        self
    }

    /// Fills the height remaining after the map controls in the containing UI.
    pub fn fill_height(mut self) -> Self {
        self.fill_height = true;
        self
    }

    /// Sets the zoom level used when this map's state is first created. Defaults to walkers' zoom.
    ///
    /// Zoom levels range from 0 (world view) to 26 (street level). The visible ground width also
    /// depends on the viewport size and latitude.
    pub fn initial_zoom(mut self, zoom: f64) -> Self {
        self.initial_zoom = zoom.is_finite().then(|| zoom.clamp(0.0, 26.0));
        self
    }

    /// Draws the map and invokes `draw_overlay` on every UI pass.
    ///
    /// The overlay receives the map UI, its response, and the geographic projector. Use the
    /// projector to place geographic features so they follow map panning and zooming.
    pub(crate) fn show_with_overlay(
        self,
        ui: &mut Ui,
        mut draw_overlay: impl FnMut(&mut Ui, &egui::Response, &Projector),
    ) -> egui::Response {
        self.show_inner(ui, &mut draw_overlay)
    }

    fn show_inner(
        self,
        ui: &mut Ui,
        draw_overlay: &mut impl FnMut(&mut Ui, &egui::Response, &Projector),
    ) -> egui::Response {
        let ctx = ui.ctx().clone();
        let state_id = self.id.with("bee_ui.map.state");
        let center = self.center.clone();
        let height = self.height;
        let fill_height = self.fill_height;
        let initial_zoom = self.initial_zoom;
        let routes = self.routes;
        let state = ctx.data_mut(|data| {
            data.get_temp_mut_or_insert_with(state_id, || {
                Arc::new(Mutex::new(MapState::new(ctx.clone(), initial_zoom)))
            })
            .clone()
        });
        let mut state = state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        Frame::group(ui.style())
            .show(ui, |ui| {
                MapControlPanel::show(ui, &mut state.memory);
                ui.add_space(4.0);
                let map_height = if fill_height {
                    ui.available_height().max(0.0)
                } else {
                    height
                };
                let map_size = Vec2::new(ui.available_width().max(0.0), map_height);
                let (map_rect, _) = ui.allocate_exact_size(map_size, Sense::hover());
                let MapState { tiles, memory } = &mut *state;

                ui.scope_builder(UiBuilder::new().max_rect(map_rect), |map_ui| {
                    WalkersMap::new(Some(tiles), memory, center.clone())
                        .zoom_with_ctrl(false)
                        .show(map_ui, |overlay_ui, response, projector, _memory| {
                            let painter = overlay_ui.painter().with_clip_rect(response.rect);
                            for route in &routes {
                                let points = project_route(route, projector);
                                if points.len() >= 2 {
                                    painter.add(egui::Shape::line(points, route.stroke));
                                }
                            }

                            draw_overlay(overlay_ui, &response, projector);

                            let attribution_size =
                                Vec2::new(response.rect.width().min(240.0), 30.0);
                            let attribution_rect = Rect::from_min_size(
                                response.rect.right_bottom() - attribution_size - Vec2::splat(6.0),
                                attribution_size,
                            );
                            overlay_ui.scope_builder(
                                UiBuilder::new().max_rect(attribution_rect),
                                |attribution_ui| {
                                    Frame::group(attribution_ui.style()).show(
                                        attribution_ui,
                                        |attribution_ui| {
                                            attribution_ui.hyperlink_to(
                                                "© OpenStreetMap contributors",
                                                OSM_COPYRIGHT_URL,
                                            );
                                        },
                                    );
                                },
                            );
                        })
                        .response
                })
                .inner
            })
            .inner
    }
}

fn project_route(route: &MapRoute, projector: &Projector) -> Vec<Pos2> {
    let Some(first) = route.points.first() else {
        return Vec::new();
    };

    let mut projected_points = vec![project_position(projector, first.clone())];
    for pair in route.points.windows(2) {
        let start = &pair[0];
        let end = &pair[1];
        match route.interpolation {
            MapRouteInterpolation::Line => {
                projected_points.push(project_position(projector, end.clone()));
            }
            MapRouteInterpolation::Arc => {
                let angle = central_angle(start, end);
                let steps = (angle.to_degrees() / 2.0).ceil().max(1.0) as usize;
                for step in 1..=steps {
                    let position = interpolate_great_circle(start, end, step as f64 / steps as f64);
                    projected_points.push(project_position(projector, position));
                }
            }
        }
    }
    projected_points
}

fn project_position(projector: &Projector, position: Position) -> Pos2 {
    let projected = projector.project(position);
    Pos2::new(projected.x, projected.y)
}

fn unit_vector(position: &Position) -> [f64; 3] {
    let longitude = position.x().to_radians();
    let latitude = position.y().to_radians();
    [
        latitude.cos() * longitude.cos(),
        latitude.cos() * longitude.sin(),
        latitude.sin(),
    ]
}

fn central_angle(start: &Position, end: &Position) -> f64 {
    let start = unit_vector(start);
    let end = unit_vector(end);
    (start[0] * end[0] + start[1] * end[1] + start[2] * end[2])
        .clamp(-1.0, 1.0)
        .acos()
}

fn interpolate_great_circle(start: &Position, end: &Position, t: f64) -> Position {
    if t <= 0.0 {
        return start.clone();
    }
    if t >= 1.0 {
        return end.clone();
    }

    let start_vector = unit_vector(start);
    let end_vector = unit_vector(end);
    let angle = central_angle(start, end);

    if angle < 1e-12 {
        return start.clone();
    }

    let sin_angle = angle.sin();
    let vector = if sin_angle.abs() < 1e-6 {
        // Antipodal points have no unique shortest arc; use a deterministic coordinate fallback.
        let longitude_delta = (end.x() - start.x() + 180.0).rem_euclid(360.0) - 180.0;
        return lon_lat(
            start.x() + longitude_delta * t,
            start.y() + (end.y() - start.y()) * t,
        );
    } else {
        let start_weight = ((1.0 - t) * angle).sin() / sin_angle;
        let end_weight = (t * angle).sin() / sin_angle;
        [
            start_weight * start_vector[0] + end_weight * end_vector[0],
            start_weight * start_vector[1] + end_weight * end_vector[1],
            start_weight * start_vector[2] + end_weight * end_vector[2],
        ]
    };

    let longitude = vector[1].atan2(vector[0]).to_degrees();
    let latitude = vector[2].atan2(vector[0].hypot(vector[1])).to_degrees();
    lon_lat(longitude, latitude)
}

impl Widget for Map {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        self.show_inner(ui, &mut |_, _, _| {})
    }
}

struct MapState {
    tiles: HttpTiles,
    memory: MapMemory,
}

impl MapState {
    fn new(ctx: egui::Context, initial_zoom: Option<f64>) -> Self {
        let mut memory = MapMemory::default();
        if let Some(zoom) = initial_zoom {
            let _ = memory.set_zoom(zoom);
        }

        Self {
            tiles: HttpTiles::new(OpenStreetMap, ctx),
            memory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_interpolation_follows_a_great_circle() {
        let start = lon_lat(0.0, 0.0);
        let end = lon_lat(90.0, 60.0);

        let midpoint = interpolate_great_circle(&start, &end, 0.5);

        assert!(midpoint.y() > 30.0);
    }

    #[test]
    fn arc_interpolation_preserves_endpoints() {
        let start = lon_lat(0.0, 0.0);
        let end = lon_lat(90.0, 60.0);

        let actual_start = interpolate_great_circle(&start, &end, 0.0);
        let actual_end = interpolate_great_circle(&start, &end, 1.0);

        assert_eq!(actual_start, start);
        assert_eq!(actual_end, end);
    }
}
