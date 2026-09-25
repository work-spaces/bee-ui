use crate::prelude::*;

use super::{super::state, shared};

pub(super) fn render_view_progress_spinners(ui: &mut UiRoot<'_>) {
    let progress_value = state::simulated_progress_value(ui.context());

    ui.card(
        Card::new()
            .title("Intro — rollout progress and background activity")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Progress bars communicate bounded task completion while spinners represent in-flight checks. Examples below use deployment and sync workflows.",
            ));
        },
    );

    ui.card(
        Card::new().title("Variant gallery — Progress").outline(),
        |ui| {
            shared::for_each_variant(|name, variant| {
                ui.progress(
                    Progress::new(progress_value)
                        .variant(variant)
                        .label(format!("{name} rollout"))
                        .with_percentage(),
                );
            });
        },
    );

    ui.card(
        Card::new().title("Variant gallery — Spinner").outline(),
        |ui| {
            let spinner_sizes = [14.0_f32, 18.0, 22.0, 26.0, 30.0, 34.0, 38.0, 42.0];
            for (index, &(name, variant)) in shared::variant_palette().iter().enumerate() {
                let spinner = Spinner::new().variant(variant).size(spinner_sizes[index]);
                if index % 2 == 0 {
                    ui.spinner(spinner.label(format!("{name} check")));
                } else {
                    ui.spinner(spinner);
                }
            }
        },
    );

    ui.card(
        Card::new()
            .title("Composed example — rollout + sync tasks")
            .outline(),
        |ui| {
            ui.progress(
                Progress::new(progress_value)
                    .variant(Variant::Info)
                    .label("api-gateway canary rollout")
                    .with_percentage(),
            );
            ui.progress(
                Progress::new(0.72)
                    .range(0.0, 1.0)
                    .variant(Variant::Success)
                    .label("cluster sync completion (0.0..1.0)")
                    .with_percentage(),
            );
            ui.progress(
                Progress::new(47.0)
                    .range(20.0, 80.0)
                    .variant(Variant::Warning)
                    .label("background checks completed (20..80 range)")
                    .with_percentage(),
            );

            ui.spinner(
                Spinner::new()
                    .variant(Variant::Secondary)
                    .label("indexing incident timeline"),
            );
            ui.spinner(Spinner::new().variant(Variant::Dark).size(30.0));

            ui.button(
                Button::primary("Advance rollout (+10%)")
                    .hover_text("Cycles 0% → 100% and updates persisted state")
                    .on_click(state::feedback_advance_progress),
            );
        },
    );

    ui.card(Card::new().title("Interaction state").outline(), |ui| {
        ui.badge(Badge::secondary(format!("Rollout progress: {:.0}%", progress_value)).as_pill());
        ui.badge(
            Badge::info(format!("Last action: {}", state::last_action(ui.context()))).as_pill(),
        );
    });
}
