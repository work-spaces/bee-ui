use crate::prelude::*;

use super::{super::state, shared};

pub(super) fn render_view_tables(ui: &mut UiRoot<'_>) {
    let selected_row = state::selected_table_row(ui.context()).min(shared::SERVICES.len() - 1);

    ui.card(
        Card::new()
            .title("Intro — operator datasets")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Tables in runnerd surface service-level health with visual hierarchy, row interaction, and sizing behavior tuned for responsive dashboard panels.",
            ));
        },
    );

    ui.card(
        Card::new()
            .title("Variant gallery")
            .outline()
            .footer("Representative mini-table rendered once per variant"),
        |ui| {
            for variants in shared::variant_palette().chunks(4) {
                ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                    for &(name, variant) in variants {
                        ui.col(Col::new().xs(12).md(6).lg(3), |ui| {
                            ui.label(Label::new(name).small().subtle());
                            ui.table(
                                Table::new()
                                    .variant(variant)
                                    .headers(["Service", "State"])
                                    .row(["scheduler", "Healthy"])
                                    .row(["api-gateway", "Degraded"]),
                            );
                        });
                    }
                });
            }
        },
    );

    ui.card(
        Card::new()
            .title("Composed example — interactive operations table")
            .outline()
            .footer(
                "Features: striped, hover, active row, click callbacks, sticky columns, resizable columns, auto_size_mode(OnParentResize)",
            ),
        |ui| {
            ui.table(
                Table::new()
                    .variant(Variant::Primary)
                    .headers([
                        "Cluster",
                        "Service",
                        "Strategy",
                        "Replicas",
                        "P95 ms ↓",
                        "Error % ↓",
                        "Status",
                    ])
                    .striped_rows()
                    .hoverable()
                    .sticky_columns(2)
                    .resizable_columns(true)
                    .auto_size_mode(TableAutoSizeMode::OnParentResize)
                    .row(service_row(0, selected_row))
                    .row(service_row(1, selected_row))
                    .row(service_row(2, selected_row)),
            );
        },
    );

    ui.card(
        Card::new()
            .title("Composed example — auto-size mode coverage")
            .outline(),
        |ui| {
            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.label(Label::new("Never").small().subtle());
                    ui.table(
                        Table::new()
                            .variant(Variant::Info)
                            .headers(["Mode", "Behavior"])
                            .auto_size_mode(TableAutoSizeMode::Never)
                            .row(["Never", "fixed sizing"])
                            .row(["Use case", "stable column widths"]),
                    );
                });

                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.label(Label::new("Always").small().subtle());
                    ui.table(
                        Table::new()
                            .variant(Variant::Success)
                            .headers(["Mode", "Behavior"])
                            .auto_size_mode(TableAutoSizeMode::Always)
                            .row(["Always", "resize every frame"])
                            .row(["Use case", "highly dynamic cells"]),
                    );
                });

                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.label(Label::new("OnParentResize").small().subtle());
                    ui.table(
                        Table::new()
                            .variant(Variant::Warning)
                            .headers(["Mode", "Behavior"])
                            .auto_size_mode(TableAutoSizeMode::OnParentResize)
                            .row(["OnParentResize", "resize when layout changes"])
                            .row(["Use case", "dashboard panels"]),
                    );
                });
            });
        },
    );

    ui.card(Card::new().title("Interaction state").outline(), |ui| {
        ui.badge(Badge::secondary(format!("Selected row: {selected_row}")).as_pill());
        ui.badge(
            Badge::info(format!("Last action: {}", state::last_action(ui.context()))).as_pill(),
        );
    });
}

fn service_row(index: usize, selected_row: usize) -> TableRow {
    let service = shared::SERVICES[index];
    let callback = match index {
        0 => state::table_row_scheduler_clicked,
        1 => state::table_row_api_gateway_clicked,
        _ => state::table_row_worker_clicked,
    };

    TableRow::new()
        .cells([
            service.cluster.to_owned(),
            service.name.to_owned(),
            service.deploy_strategy.to_owned(),
            format!("{}/{}", service.ready_replicas, service.desired_replicas),
            format!("{:.0}", service.p95_latency_ms),
            format!("{:.2}", service.error_rate_pct),
            service.status.to_owned(),
        ])
        .active(selected_row == index)
        .on_click(callback)
}
