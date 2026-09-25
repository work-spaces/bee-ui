use crate::prelude::*;

use super::{super::state, shared};

pub(super) fn render_view_overview(ui: &mut UiRoot<'_>) {
    let selected_cluster = state::selected_cluster(ui.context());
    let selected_service = state::selected_service(ui.context());
    let cluster = shared::cluster_by_index(selected_cluster);
    let service = shared::service_by_index(selected_service);

    ui.card(
        Card::new()
            .title("Runnerd control plane scenario")
            .outline()
            .footer("Use each view as a methodical reference for one component family"),
        |ui| {
            ui.text(Text::new(
                "This demo models day-2 operations across prod-a, prod-b, and staging clusters.",
            ));
            ui.text(Text::new(
                "Every example is intentionally operational: deployments, incidents, SLO trends, and workflow actions.",
            ));
            ui.badge(Badge::secondary(format!("Focus cluster: {}", cluster.name)).as_pill());
            ui.badge(Badge::info(format!("Focus service: {}", service.name)).as_pill());
            ui.badge(Badge::dark(format!("Last action: {}", state::last_action(ui.context()))).as_pill());
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(Card::new().title("Cluster focus").outline(), |ui| {
                ui.label(
                    Label::new("Select the active cluster context")
                        .small()
                        .subtle(),
                );
                ui.button(
                    Button::new("prod-a")
                        .variant(if selected_cluster == 0 {
                            Variant::Primary
                        } else {
                            Variant::Secondary
                        })
                        .hover_text("Set focus to prod-a")
                        .on_click(state::cluster_prod_a_clicked),
                );
                ui.button(
                    Button::new("prod-b")
                        .variant(if selected_cluster == 1 {
                            Variant::Primary
                        } else {
                            Variant::Secondary
                        })
                        .hover_text("Set focus to prod-b")
                        .on_click(state::cluster_prod_b_clicked),
                );
                ui.button(
                    Button::new("staging")
                        .variant(if selected_cluster == 2 {
                            Variant::Primary
                        } else {
                            Variant::Secondary
                        })
                        .hover_text("Set focus to staging")
                        .on_click(state::cluster_staging_clicked),
                );

                ui.badge(
                    Badge::new(cluster.status)
                        .variant(cluster.status_variant)
                        .as_pill(),
                );
                ui.text(Text::new(format!(
                    "Region {} • {} nodes • control-plane {}",
                    cluster.region, cluster.nodes, cluster.control_plane_version
                )));
            });
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(Card::new().title("Service focus").outline(), |ui| {
                ui.label(
                    Label::new("Pick the service used by interactive examples")
                        .small()
                        .subtle(),
                );

                ui.button(
                    Button::new("scheduler")
                        .variant(if selected_service == 0 {
                            Variant::Info
                        } else {
                            Variant::Secondary
                        })
                        .hover_text("Inspect scheduler behavior")
                        .on_click(state::service_scheduler_clicked),
                );
                ui.button(
                    Button::new("api-gateway")
                        .variant(if selected_service == 1 {
                            Variant::Info
                        } else {
                            Variant::Secondary
                        })
                        .hover_text("Inspect api-gateway behavior")
                        .on_click(state::service_api_gateway_clicked),
                );
                ui.button(
                    Button::new("worker")
                        .variant(if selected_service == 2 {
                            Variant::Info
                        } else {
                            Variant::Secondary
                        })
                        .hover_text("Inspect worker behavior")
                        .on_click(state::service_worker_clicked),
                );

                ui.badge(
                    Badge::new(service.status)
                        .variant(service.status_variant)
                        .as_pill(),
                );
                ui.text(Text::new(format!(
                    "Strategy {} • replicas {}/{} • p95 {:.0} ms • errors {:.2}%",
                    service.deploy_strategy,
                    service.ready_replicas,
                    service.desired_replicas,
                    service.p95_latency_ms,
                    service.error_rate_pct,
                )));
            });
        });
    });

    ui.card(
        Card::new().title("How to read each demo view").outline(),
        |ui| {
            ui.label(Label::new("1) Intro card: scope and operator intent").small());
            ui.label(Label::new("2) Variant gallery: complete API/state coverage").small());
            ui.label(Label::new("3) Composed example: realistic runnerd workflow").small());
            ui.label(Label::new("4) Interaction badges: persisted state traces").small());
        },
    );
}
