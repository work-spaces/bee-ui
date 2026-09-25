use crate::prelude::*;

use super::shared;

pub(super) fn render_view_badges(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — badge usage").outline(), |ui| {
        ui.text(Text::new(
            "Badges work best as compact status chips, counters, and tags attached to operational entities.",
        ));
    });

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).md(6), |ui| {
            ui.card(
                Card::new()
                    .title("Variant gallery — default badges")
                    .outline(),
                |ui| {
                    shared::for_each_variant(|name, variant| {
                        ui.badge(Badge::new(name).variant(variant));
                    });
                },
            );
        });

        ui.col(Col::new().xs(12).md(6), |ui| {
            ui.card(
                Card::new().title("Variant gallery — pill badges").outline(),
                |ui| {
                    shared::for_each_variant(|name, variant| {
                        ui.badge(Badge::new(name).variant(variant).as_pill());
                    });
                },
            );
        });
    });

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — cluster status chips")
                    .outline(),
                |ui| {
                    for cluster in shared::CLUSTERS {
                        ui.label(Label::new(cluster.name).small().subtle());
                        ui.badge(
                            Badge::new(cluster.status)
                                .variant(cluster.status_variant)
                                .as_pill(),
                        );
                        ui.badge(Badge::secondary(format!("{} nodes", cluster.nodes)));
                        ui.badge(Badge::info(cluster.region));
                    }
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — service tags and counts")
                    .outline(),
                |ui| {
                    for service in shared::SERVICES {
                        ui.label(Label::new(service.name));
                        ui.badge(
                            Badge::new(service.status)
                                .variant(service.status_variant)
                                .as_pill(),
                        );
                        ui.badge(Badge::primary(format!(
                            "ready {}/{}",
                            service.ready_replicas, service.desired_replicas
                        )));
                        ui.badge(Badge::warning(format!(
                            "p95 {:.0}ms",
                            service.p95_latency_ms
                        )));
                        ui.badge(Badge::danger(format!(
                            "{:.2}% errors",
                            service.error_rate_pct
                        )));
                    }
                },
            );
        });
    });
}
