use crate::prelude::*;

use super::shared;

pub(super) fn render_view_labels(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — label patterns").outline(), |ui| {
        ui.text(Text::new(
            "Labels define semantic metadata and hierarchy around sections, metrics, and operational context.",
        ));
    });

    ui.card(
        Card::new()
            .title("Variant gallery — size and muted matrix")
            .outline(),
        |ui| {
            shared::for_each_variant(|name, variant| {
                ui.row(Row::new().gutter_x(10.0).gutter_y(4.0), |ui| {
                    ui.col(Col::new().xs(12).md(4), |ui| {
                        ui.label(Label::new(format!("{name} small")).small().variant(variant));
                    });
                    ui.col(Col::new().xs(12).md(4), |ui| {
                        ui.label(Label::new(format!("{name} medium")).variant(variant));
                    });
                    ui.col(Col::new().xs(12).md(4), |ui| {
                        ui.label(
                            Label::new(format!("{name} large muted"))
                                .large()
                                .variant(variant)
                                .muted(true),
                        );
                    });
                });
            });
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — section metadata")
                    .outline(),
                |ui| {
                    ui.label(Label::new("DEPLOY STRATEGY").small().variant(Variant::Info));
                    ui.text(Text::new("canary on prod-b, 25% traffic allocation"));

                    ui.label(
                        Label::new("INCIDENT STATE")
                            .small()
                            .variant(Variant::Warning),
                    );
                    ui.text(Text::new(
                        "Investigating elevated latency and CPU throttling",
                    ));

                    ui.label(Label::new("SLO BUDGET").small().variant(Variant::Danger));
                    ui.text(Text::new(
                        "Remaining budget: 13% for the current 30-day window",
                    ));
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — muted vs non-muted intent")
                    .outline(),
                |ui| {
                    ui.label(Label::new("Primary operator directive").variant(Variant::Primary));
                    ui.label(
                        Label::new("Secondary note: runbook updates pending review")
                            .variant(Variant::Primary)
                            .subtle(),
                    );

                    ui.label(Label::new("Escalation deadline").variant(Variant::Dark));
                    ui.label(
                        Label::new("Muted context: escalation routed through on-call bridge")
                            .variant(Variant::Dark)
                            .subtle(),
                    );
                },
            );
        });
    });
}
