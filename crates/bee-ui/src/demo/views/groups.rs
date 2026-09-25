use crate::prelude::*;

use super::super::state;

pub(super) fn render_view_groups(ui: &mut UiRoot<'_>) {
    let active_item = state::groups_active_list_item(ui.context()).min(2);

    ui.card(
        Card::new()
            .title("Intro — grouped operator workflows")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "ButtonGroup and ListGroup support compact action clusters and view/workflow selection across deployments, logs, and events.",
            ));
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new().title("Variant gallery — ButtonGroup").outline(),
                |ui| {
                    ui.label(
                        Label::new("Horizontal deployment controls")
                            .small()
                            .subtle(),
                    );
                    ui.button_group(
                        ButtonGroup::new()
                            .button(
                                Button::primary("Deploy")
                                    .hover_text("Deploy rollout")
                                    .on_click(state::groups_button_deploy_clicked),
                            )
                            .button(
                                Button::warning("Pause")
                                    .hover_text("Pause rollout")
                                    .on_click(state::groups_button_pause_clicked),
                            )
                            .button(
                                Button::danger("Rollback")
                                    .hover_text("Rollback release")
                                    .on_click(state::groups_button_rollback_clicked),
                            )
                            .button(Button::secondary("Drain blocked").disabled()),
                    );

                    ui.label(Label::new("Vertical controls (`stack()`)").small().subtle());
                    ui.button_group(
                        ButtonGroup::new()
                            .stack()
                            .button(
                                Button::info("Overview")
                                    .on_click(state::groups_button_logs_clicked),
                            )
                            .button(
                                Button::success("Logs")
                                    .on_click(state::groups_button_metrics_clicked),
                            )
                            .button(
                                Button::dark("Events")
                                    .on_click(state::groups_button_events_clicked),
                            )
                            .button(Button::secondary("Audit").disabled()),
                    );
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — ListGroup workflow tabs")
                    .outline(),
                |ui| {
                    ui.label(Label::new("Non-flush list").small().subtle());
                    ui.list_group(
                        ListGroup::new()
                            .item(
                                ListGroupItem::new("Overview")
                                    .variant(Variant::Primary)
                                    .active(active_item == 0)
                                    .on_click(state::list_group_overview_clicked),
                            )
                            .item(
                                ListGroupItem::new("Logs")
                                    .variant(Variant::Info)
                                    .active(active_item == 1)
                                    .on_click(state::list_group_replicas_clicked),
                            )
                            .item(
                                ListGroupItem::new("Events")
                                    .variant(Variant::Success)
                                    .active(active_item == 2)
                                    .on_click(state::list_group_rollout_clicked),
                            )
                            .item(
                                ListGroupItem::new("Audit trail (disabled)")
                                    .variant(Variant::Secondary)
                                    .disabled(true),
                            ),
                    );

                    ui.label(Label::new("Flush list").small().subtle());
                    ui.list_group(
                        ListGroup::new()
                            .flush(true)
                            .item(
                                ListGroupItem::new("Overview")
                                    .variant(Variant::Dark)
                                    .active(active_item == 0)
                                    .on_click(state::list_group_overview_clicked),
                            )
                            .item(
                                ListGroupItem::new("Logs")
                                    .variant(Variant::Warning)
                                    .active(active_item == 1)
                                    .on_click(state::list_group_replicas_clicked),
                            )
                            .item(
                                ListGroupItem::new("Events")
                                    .variant(Variant::Danger)
                                    .active(active_item == 2)
                                    .on_click(state::list_group_rollout_clicked),
                            ),
                    );
                },
            );
        });
    });

    ui.card(Card::new().title("Interaction state").outline(), |ui| {
        ui.badge(Badge::secondary(format!("Active workflow index: {active_item}")).as_pill());
        ui.badge(
            Badge::info(format!("Last action: {}", state::last_action(ui.context()))).as_pill(),
        );
    });
}
