use crate::prelude::*;

use super::{super::state, shared};

pub(super) fn render_view_buttons(ui: &mut UiRoot<'_>) {
    ui.card(
        Card::new()
            .title("Intro — operator action buttons")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Buttons model high-confidence control-plane actions. Hover text explains intent, and each button returns an egui::Response, so clicks are handled with `.clicked()` in the same pass.",
            ));
        },
    );

    ui.card(
        Card::new()
            .title("Variant gallery — enabled + disabled states")
            .outline(),
        |ui| {
            shared::for_each_variant(|name, variant| {
                ui.button(
                    Button::new(format!("{name} Action"))
                        .icon(SvgIcon::FlagFill)
                        .variant(variant)
                        .hover_text(format!("Variant::{name} enabled state")),
                );
                ui.button(
                    Button::new(format!("{name} disabled"))
                        .variant(variant)
                        .disabled(),
                );
            });
        },
    );

    ui.card(
        Card::new()
            .title("Composed example — deployment controls")
            .outline()
            .footer("Deploy, pause, rollback, drain, and retry actions"),
        |ui| {
            let actions = [
                (
                    Button::primary("Deploy canary")
                        .hover_text("Deploy scheduler canary to prod-b"),
                    "Button: Deploy canary triggered",
                ),
                (
                    Button::warning("Pause rollout")
                        .hover_text("Pause rollout while investigating metrics"),
                    "Button: Pause rollout triggered",
                ),
                (
                    Button::danger("Rollback").hover_text("Rollback to last healthy release"),
                    "Button: Rollback triggered",
                ),
                (
                    Button::dark("Drain node pool").hover_text("Drain prod-b worker node pool"),
                    "Button: Drain node pool triggered",
                ),
                (
                    Button::info("Retry failed checks")
                        .hover_text("Retry health checks after mitigation"),
                    "Button: Retry failed checks triggered",
                ),
            ];

            for (button, action) in actions {
                if ui.button(button).clicked() {
                    state::set_last_action(ui.context(), action);
                }
            }

            ui.button(
                Button::secondary("Escalate")
                    .hover_text("Escalation disabled until runbook owner confirms")
                    .disabled(),
            );
        },
    );

    ui.card(Card::new().title("Interaction state").outline(), |ui| {
        ui.badge(
            Badge::dark(format!("Last action: {}", state::last_action(ui.context()))).as_pill(),
        );
    });
}
