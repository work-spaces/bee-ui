use crate::prelude::*;

use super::{super::state, shared};

const UNCONTROLLED_OBSERVABILITY_ID: &str = "runnerd_bs.demo.dropdowns.uncontrolled_observability";

/// Items of the controlled "Service actions" dropdown, in menu order. `None` marks items that
/// can't be selected.
const SERVICE_ACTIONS: [Option<&str>; 4] = [
    Some("Restart worker"),
    Some("Reschedule pod"),
    None,
    Some("Scale deployment"),
];

/// Selectable items of the uncontrolled "Observability" dropdown, in menu order.
const OBSERVABILITY_ACTIONS: [&str; 2] = ["Tail logs", "Inspect events"];

pub(super) fn render_view_dropdowns(ui: &mut UiRoot<'_>, demo_state: &mut state::DemoState) {
    let primary_selection = state::dropdown_primary_selection(ui.context());
    let secondary_selection = state::dropdown_secondary_selection(ui.context());

    ui.card(
        Card::new()
            .title("Intro — operational dropdown menus")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Dropdowns expose contextual actions and observability shortcuts with controlled/uncontrolled disclosure, alignment options, and item-level state.",
            ));
        },
    );

    ui.card(
        Card::new()
            .title("Variant gallery + disabled patterns")
            .outline(),
        |ui| {
            let mut variant_entries = shared::variant_palette().iter();

            for row in 0..2 {
                ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                    for col in 0..4 {
                        if let Some((name, variant)) = variant_entries.next() {
                            let dropdown_id = format!(
                                "runnerd_bs.demo.dropdowns.variant_gallery.{row}.{col}.{}",
                                name.to_lowercase()
                            );

                            ui.col(Col::new().xs(12).md(6).lg(3), |ui| {
                                ui.dropdown(
                                    DisclosureState::uncontrolled(dropdown_id, false),
                                    Dropdown::new(*name)
                                        .variant(*variant)
                                        .item("View details")
                                        .item("Open runbook")
                                        .item(DropdownItem::new("Restricted").disabled(true)),
                                );
                            });
                        }
                    }
                });
            }

            ui.dropdown(
                DisclosureState::uncontrolled("runnerd_bs.demo.dropdowns.disabled", false),
                Dropdown::new("Disabled dropdown")
                    .variant(Variant::Secondary)
                    .disabled(true)
                    .item("No-op"),
            );
        },
    );

    ui.card(Card::new().title("Interaction state").outline(), |ui| {
        ui.badge(
            Badge::secondary(format!(
                "Controlled open: {}",
                demo_state.dropdowns_controlled_open
            ))
            .as_pill(),
        );
        ui.badge(Badge::info(format!("Primary selection: {primary_selection}")).as_pill());
        ui.badge(Badge::info(format!("Secondary selection: {secondary_selection}")).as_pill());
        ui.badge(
            Badge::dark(format!("Last action: {}", state::last_action(ui.context()))).as_pill(),
        );
    });

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — controlled disclosure (`&mut bool`)")
                    .outline(),
                |ui| {
                    let clicked = ui
                        .dropdown(
                            &mut demo_state.dropdowns_controlled_open,
                            Dropdown::new("Service actions")
                                .variant(Variant::Primary)
                                .align_end(false)
                                .item(
                                    DropdownItem::new("Restart worker")
                                        .active(primary_selection == "Restart worker"),
                                )
                                .item(
                                    DropdownItem::new("Reschedule pod")
                                        .active(primary_selection == "Reschedule pod"),
                                )
                                .item(DropdownItem::new("Delete cluster").disabled(true))
                                .item(
                                    DropdownItem::new("Scale deployment")
                                        .active(primary_selection == "Scale deployment"),
                                ),
                        )
                        .clicked_index();

                    if let Some(item) =
                        clicked.and_then(|index| SERVICE_ACTIONS.get(index).copied().flatten())
                    {
                        let ctx = ui.context();
                        state::set_dropdown_primary_selection(ctx, item);
                        state::set_last_action(ctx, format!("Dropdown: {item} selected"));
                    }

                    ui.label(
                        Label::new("align_end(false) keeps menu start-aligned")
                            .small()
                            .subtle(),
                    );
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — uncontrolled disclosure")
                    .outline(),
                |ui| {
                    let clicked = ui
                        .dropdown(
                            DisclosureState::uncontrolled(UNCONTROLLED_OBSERVABILITY_ID, false),
                            Dropdown::new("Observability")
                                .variant(Variant::Info)
                                .align_end(true)
                                .item(
                                    DropdownItem::new(OBSERVABILITY_ACTIONS[0])
                                        .active(secondary_selection == OBSERVABILITY_ACTIONS[0]),
                                )
                                .item(
                                    DropdownItem::new(OBSERVABILITY_ACTIONS[1])
                                        .active(secondary_selection == OBSERVABILITY_ACTIONS[1]),
                                )
                                .item(DropdownItem::new("Live traces").current())
                                .item(DropdownItem::new("Debug shell").disabled(true)),
                        )
                        .clicked_index();

                    if let Some(&item) = clicked.and_then(|index| OBSERVABILITY_ACTIONS.get(index))
                    {
                        let ctx = ui.context();
                        state::set_dropdown_primary_selection(ctx, item);
                        state::set_dropdown_secondary_selection(ctx, item);
                        state::set_last_action(
                            ctx,
                            format!("Dropdown (uncontrolled): {item} selected"),
                        );
                    }

                    ui.label(
                        Label::new("align_end(true) pins the menu to the right edge")
                            .small()
                            .subtle(),
                    );
                },
            );
        });
    });
}
