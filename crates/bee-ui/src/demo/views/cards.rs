use crate::prelude::*;

use super::shared;

pub(super) fn render_view_cards(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — card composition patterns").outline(), |ui| {
        ui.text(Text::new(
            "Cards package status, details, and action context. This view demonstrates variants, outline styles, and compositional layouts.",
        ));
    });

    ui.card(
        Card::new()
            .title("Variant gallery — filled cards")
            .outline(),
        |ui| {
            for variants in shared::variant_palette().chunks(4) {
                ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                    for &(name, variant) in variants {
                        ui.col(Col::new().xs(12).md(6).lg(3), |ui| {
                            ui.card(
                                Card::new()
                                    .title(format!("{name} filled"))
                                    .variant(variant)
                                    .footer("Cluster summary"),
                                |ui| {
                                    ui.label(Label::new("Deployments in sync").small());
                                },
                            );
                        });
                    }
                });
            }
        },
    );

    ui.card(
        Card::new()
            .title("Variant gallery — outlined cards")
            .outline(),
        |ui| {
            for variants in shared::variant_palette().chunks(4) {
                ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                    for &(name, variant) in variants {
                        ui.col(Col::new().xs(12).md(6).lg(3), |ui| {
                            ui.card(
                                Card::new()
                                    .title(format!("{name} outlined"))
                                    .variant(variant)
                                    .outline()
                                    .footer("Read-only diagnostics"),
                                |ui| {
                                    ui.label(Label::new("Operator metadata").small().subtle());
                                },
                            );
                        });
                    }
                });
            }
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(4), |ui| {
            ui.card(Card::new().title("Header only"), |ui| {
                ui.text(Text::new("Minimal cluster summary block."));
            });
        });

        ui.col(Col::new().xs(12).lg(4), |ui| {
            ui.card(
                Card::new()
                    .title("Header + footer")
                    .footer("Updated 45s ago"),
                |ui| {
                    ui.text(Text::new("Deployment health snapshot for prod-a."));
                },
            );
        });

        ui.col(Col::new().xs(12).lg(4), |ui| {
            ui.card(Card::new().footer("Footer-only annotation"), |ui| {
                ui.text(Text::new("Useful for note blocks and compact annotations."));
            });
        });
    });

    ui.card(
        Card::new()
            .title("Composed example — operational dashboard")
            .outline()
            .footer("Summary cards + detail card + nested content"),
        |ui| {
            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                for cluster in shared::CLUSTERS {
                    ui.col(Col::new().xs(12).md(4), |ui| {
                        ui.card(
                            Card::new()
                                .title(cluster.name)
                                .variant(cluster.status_variant)
                                .outline(),
                            |ui| {
                                ui.badge(
                                    Badge::new(cluster.status)
                                        .variant(cluster.status_variant)
                                        .as_pill(),
                                );
                                ui.text(Text::new(format!(
                                    "{} nodes • {}",
                                    cluster.nodes, cluster.region
                                )));
                            },
                        );
                    });
                }
            });

            ui.card(
                Card::new()
                    .title("Incident detail")
                    .variant(Variant::Dark)
                    .outline(),
                |ui| {
                    ui.text(Text::new(
                        "INC-1042 investigation includes rollback readiness, traffic shedding, and queue-depth monitoring.",
                    ));

                    ui.card(
                        Card::new()
                            .title("Nested timeline")
                            .variant(Variant::Secondary)
                            .outline()
                            .footer("runbook: rollback/api-gateway"),
                        |ui| {
                            ui.label(Label::new("13:04 UTC • traffic reduced to 75%").small());
                            ui.label(Label::new("13:09 UTC • canary paused").small());
                            ui.label(Label::new("13:14 UTC • error budget stabilized").small());
                        },
                    );
                },
            );
        },
    );
}
