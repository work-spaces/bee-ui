use crate::prelude::*;

pub(super) fn render_view_map(ui: &mut UiRoot<'_>) {
    ui.card(
        Card::new().title("Intro — interactive map").outline(),
        |ui| {
            ui.text(Text::new(
                "Map embeds walkers in a semantic bee-ui layout. Drag to pan, use the wheel or zoom controls to change scale, and recenter to return to the selected site.",
            ));
            ui.label(
                Label::new("Tiles are provided by OpenStreetMap and require an internet connection.")
                    .small()
                    .subtle(),
            );
        },
    );

    ui.card(
        Card::new()
            .title("Mission area — San Francisco")
            .outline()
            .footer("Drag to explore • scroll to zoom • © OpenStreetMap contributors"),
        |ui| {
            let center = lon_lat(-122.4194, 37.7749);
            let route = [
                center.clone(),
                lon_lat(-122.4145, 37.7785),
                lon_lat(-122.4085, 37.7760),
                lon_lat(-122.4120, 37.7690),
            ];
            ui.map_with_overlay(
                Map::new("bee_ui.demo.map.san_francisco", center.clone()).height(360.0),
                |overlay_ui, response, projector| {
                    let painter = overlay_ui.painter().with_clip_rect(response.rect);
                    let route_points = route
                        .iter()
                        .map(|point| {
                            let projected = projector.project(point.clone());
                            egui::pos2(projected.x, projected.y)
                        })
                        .collect::<Vec<_>>();
                    painter.add(egui::Shape::line(
                        route_points,
                        egui::Stroke::new(6.0, egui::Color32::from_rgb(40, 120, 240)),
                    ));

                    let projected_center = projector.project(center.clone());
                    let center_screen = egui::pos2(projected_center.x, projected_center.y);
                    let radius = 15.0 * projector.scale_pixel_per_meter(center.clone());
                    painter.circle_stroke(
                        center_screen,
                        radius,
                        egui::Stroke::new(2.5, egui::Color32::MAGENTA),
                    );
                    painter.circle_filled(center_screen, 2.0, egui::Color32::MAGENTA);
                },
            );
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).md(4), |ui| {
            ui.card(Card::new().title("Viewport controls").outline(), |ui| {
                ui.label(Label::new("− / + buttons adjust zoom").small());
                ui.label(Label::new("Recenter follows the configured position").small());
                ui.label(Label::new("Map state is retained by its stable widget ID").small());
            });
        });
        ui.col(Col::new().xs(12).md(4), |ui| {
            ui.card(Card::new().title("Tile source").outline(), |ui| {
                ui.text(Text::new("OpenStreetMap standard raster tiles"));
                ui.text(Text::new("Attribution links to the OSM copyright page."));
            });
        });
        ui.col(Col::new().xs(12).md(4), |ui| {
            ui.card(Card::new().title("Integration").outline(), |ui| {
                ui.text(Text::new(
                    "Draw every frame with ui.map_with_overlay(map, |ui, response, projector| ...).",
                ));
                ui.text(Text::new("Project geographic points so overlays track pan and zoom."));
            });
        });
    });
}
