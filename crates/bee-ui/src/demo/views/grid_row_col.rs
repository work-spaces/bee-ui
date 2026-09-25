use crate::{
    layout::{Breakpoints, resolve_active_breakpoint},
    prelude::*,
};

pub(super) fn render_view_grid_row_col(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — grid layout behavior").outline(), |ui| {
        ui.raw(|egui_ui| {
            let width = egui_ui.available_width();
            let bp = resolve_active_breakpoint(width, Breakpoints::bootstrap());
            egui_ui.label(
                egui::RichText::new(format!(
                    "content width: {:.0}px • active breakpoint: {}",
                    width,
                    breakpoint_name(bp)
                ))
                .small(),
            );
        });

        ui.text(Text::new(
            "These examples use operator dashboard panels to demonstrate gutters, responsive spans, offsets, and ordering.",
        ));
    });

    ui.card(
        Card::new().title("Variant gallery — Row gutters").outline(),
        |ui| {
            ui.label(
                Label::new("Default Row::new() (gutter_x=0, gutter_y=0)")
                    .small()
                    .subtle(),
            );
            ui.row(Row::new(), |ui| {
                dashboard_panel(
                    ui,
                    Col::new().xs(12).md(4),
                    "Incident queue",
                    Variant::Warning,
                );
                dashboard_panel(ui, Col::new().xs(12).md(4), "Deploy queue", Variant::Info);
                dashboard_panel(
                    ui,
                    Col::new().xs(12).md(4),
                    "Node drains",
                    Variant::Secondary,
                );
            });

            ui.label(
                Label::new("Custom `gutter(Spacing::new(16, 16))`")
                    .small()
                    .subtle(),
            );
            ui.row(Row::new().gutter(Spacing::new(16.0, 16.0)), |ui| {
                dashboard_panel(
                    ui,
                    Col::new().xs(12).md(4),
                    "Scheduler health",
                    Variant::Success,
                );
                dashboard_panel(
                    ui,
                    Col::new().xs(12).md(4),
                    "Gateway latency",
                    Variant::Warning,
                );
                dashboard_panel(
                    ui,
                    Col::new().xs(12).md(4),
                    "Worker backlog",
                    Variant::Danger,
                );
            });

            ui.label(
                Label::new("Independent `gutter_x(24)` and `gutter_y(6)`")
                    .small()
                    .subtle(),
            );
            ui.row(Row::new().gutter_x(24.0).gutter_y(6.0), |ui| {
                dashboard_panel(
                    ui,
                    Col::new().xs(12).md(6),
                    "Rollout stages",
                    Variant::Primary,
                );
                dashboard_panel(ui, Col::new().xs(12).md(6), "SLO burn rate", Variant::Dark);
                dashboard_panel(ui, Col::new().xs(12), "Audit events", Variant::Light);
            });
        },
    );

    ui.card(
        Card::new()
            .title("Variant gallery — responsive Col spans + fallback")
            .outline(),
        |ui| {
            ui.label(
                Label::new(
                    "Panels intentionally define different breakpoint subsets so you can see fallback resolution as the container width changes.",
                )
                .small()
                .subtle(),
            );

            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                ui.col(Col::new().xs(12).sm(6).md(4).lg(3).xl(3).xxl(2), |ui| {
                    ui.card(Card::new().variant(Variant::Primary), |ui| {
                        ui.label(Label::new("Incident stream"));
                        ui.label(Label::new("xs12 sm6 md4 lg3 xl3 xxl2").small().subtle());
                    });
                });
                ui.col(Col::new().xs(12).md(8), |ui| {
                    ui.card(Card::new().variant(Variant::Info), |ui| {
                        ui.label(Label::new("Latency timeline"));
                        ui.label(Label::new("xs12 md8 (lg/xl/xxl fallback)").small().subtle());
                    });
                });
                ui.col(Col::new().xs(12).sm(6).md(4).lg(6).xl(5), |ui| {
                    ui.card(Card::new().variant(Variant::Secondary), |ui| {
                        ui.label(Label::new("Rollout plan"));
                        ui.label(Label::new("xs12 sm6 md4 lg6 xl5").small().subtle());
                    });
                });
            });
        },
    );

    ui.card(
        Card::new()
            .title("Composed example — order + offset dashboard layout")
            .outline(),
        |ui| {
            ui.label(
                Label::new(
                    "On larger breakpoints, order changes visual priority and offset reserves space for side panels.",
                )
                .small()
                .subtle(),
            );

            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                ui.col(Col::new().xs(12).md(4).order(2), |ui| {
                    ui.card(Card::new().variant(Variant::Warning), |ui| {
                        ui.label(Label::new("2) Incident detail"));
                        ui.label(Label::new("xs12 md4 order2").small().subtle());
                    });
                });
                ui.col(Col::new().xs(12).md(5).order(1), |ui| {
                    ui.card(Card::new().variant(Variant::Success), |ui| {
                        ui.label(Label::new("1) Cluster summary"));
                        ui.label(Label::new("xs12 md5 order1").small().subtle());
                    });
                });
                ui.col(Col::new().xs(10).md(2).offset(1).order(3), |ui| {
                    ui.card(Card::new().variant(Variant::Dark), |ui| {
                        ui.label(Label::new("3) Escalations"));
                        ui.label(Label::new("xs10 md2 offset1 order3").small().subtle());
                    });
                });
            });
        },
    );
}

fn dashboard_panel(ui: &mut UiRoot<'_>, col: Col, title: &str, variant: Variant) {
    ui.col(col, |ui| {
        ui.card(Card::new().variant(variant), |ui| {
            ui.label(Label::new(title));
        });
    });
}

fn breakpoint_name(breakpoint: Breakpoint) -> &'static str {
    match breakpoint {
        Breakpoint::Xs => "xs",
        Breakpoint::Sm => "sm",
        Breakpoint::Md => "md",
        Breakpoint::Lg => "lg",
        Breakpoint::Xl => "xl",
        Breakpoint::Xxl => "xxl",
    }
}
