use crate::prelude::*;

use super::{super::state, shared};

pub(super) fn render_view_form<'a>(ui: &mut UiRoot<'a>, demo_state: &'a mut state::DemoState) {
    let change_summary_valid = demo_state.form_change_summary.trim().len() >= 12;
    let requester_email_valid = looks_like_email(&demo_state.form_requester_email);
    let runbook_path_valid = demo_state.form_runbook_path.trim().contains('/');
    let target_cluster_valid = demo_state.form_target_cluster_index < shared::CLUSTERS.len();
    let canary_percent_valid = (5.0..=50.0).contains(&demo_state.form_canary_percent);
    let freeze_window_confirmed = demo_state.form_freeze_approved;
    let login_username_valid = demo_state.form_login_username.trim().len() >= 3;
    let login_password_valid = login_password_looks_valid(&demo_state.form_login_password);
    let login_ready = login_username_valid && login_password_valid;

    let form_ready = change_summary_valid
        && requester_email_valid
        && runbook_path_valid
        && target_cluster_valid
        && canary_percent_valid
        && freeze_window_confirmed;

    let target_cluster_label = selected_cluster_label(demo_state.form_target_cluster_index);
    let canary_percent = demo_state.form_canary_percent;
    let requires_db_migration = demo_state.form_requires_db_migration;

    ui.card(
        Card::new()
            .title("Intro — production rollout request form")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "This scenario models an SRE completing a production rollout request for api-gateway after elevated latency alerts.",
            ));
            ui.text(Text::new(
                "The form demonstrates field-level guidance, live validation state, and safety checklist controls before queueing deployment work.",
            ));
        },
    );

    ui.card(
        Card::new()
            .title("Login section — operator authentication")
            .outline()
            .footer("Authenticate before approving or submitting production changes"),
        |ui| {
            ui.form_label(FormLabel::new("Username").is_required());
            ui.form_control(
                &mut demo_state.form_login_username,
                FormControl::new()
                    .placeholder("ops-admin")
                    .validation(validation_state(login_username_valid))
                    .on_change(on_change_login_username_updated),
            );
            ui.validation_feedback(
                if login_username_valid {
                    ValidationFeedback::valid("Username accepted")
                } else {
                    ValidationFeedback::invalid("Use at least 3 characters")
                }
                .variant(if login_username_valid {
                    Variant::Success
                } else {
                    Variant::Danger
                }),
            );

            ui.form_label(FormLabel::new("Password").is_required());
            ui.form_control(
                &mut demo_state.form_login_password,
                FormControl::new()
                    .placeholder("Enter password")
                    .as_password()
                    .validation(validation_state(login_password_valid))
                    .on_change(on_change_login_password_updated),
            );
            ui.validation_feedback(
                if login_password_valid {
                    ValidationFeedback::valid("Password meets minimum policy")
                } else {
                    ValidationFeedback::invalid(
                        "Use at least 8 characters with mixed letters and digits",
                    )
                }
                .variant(if login_password_valid {
                    Variant::Success
                } else {
                    Variant::Danger
                }),
            );

            ui.button(
                Button::primary("Sign in")
                    .enabled(login_ready)
                    .on_click(on_click_login),
            );
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(7), |ui| {
            ui.card(
                Card::new()
                    .title("Change request details")
                    .outline()
                    .footer("Required fields must be valid before queueing"),
                |ui| {
                    ui.form_label(FormLabel::new("Change summary").is_required());
                    ui.form_control(
                        &mut demo_state.form_change_summary,
                        FormControl::new()
                            .placeholder("Describe the production change")
                            .validation(validation_state(change_summary_valid))
                            .on_change(on_change_summary_updated),
                    );
                    ui.validation_feedback(
                        if change_summary_valid {
                            ValidationFeedback::valid(
                                "Summary is specific enough for release review",
                            )
                        } else {
                            ValidationFeedback::invalid(
                                "Use at least 12 characters and include impact scope",
                            )
                        }
                        .variant(if change_summary_valid {
                            Variant::Success
                        } else {
                            Variant::Danger
                        }),
                    );
                    ui.form_text(
                        FormText::new(
                            "Include affected service, objective, and rollback intent.",
                        )
                        .variant(Variant::Secondary),
                    );

                    ui.form_label(FormLabel::new("Requester email").is_required());
                    ui.floating_label(
                        &mut demo_state.form_requester_email,
                        FloatingLabel::new("Primary approver")
                            .placeholder("name@runnerd.dev")
                            .validation(validation_state(requester_email_valid))
                            .on_change(on_change_requester_email_updated),
                    );
                    ui.validation_feedback(
                        if requester_email_valid {
                            ValidationFeedback::valid("Approval contact format looks valid")
                        } else {
                            ValidationFeedback::invalid(
                                "Provide a reachable @runnerd.dev address",
                            )
                        }
                        .variant(if requester_email_valid {
                            Variant::Success
                        } else {
                            Variant::Danger
                        }),
                    );

                    ui.form_label(FormLabel::new("Target cluster").is_required());
                    ui.form_select(
                        &mut demo_state.form_target_cluster_index,
                        FormSelect::new()
                            .placeholder("Select a cluster")
                            .option("prod-a (us-east-1)")
                            .option("prod-b (us-west-2)")
                            .option("staging (us-central-1)")
                            .option(FormSelectOption::new("prod-eu (capacity freeze)").disabled(true))
                            .validation(validation_state(target_cluster_valid))
                            .on_change(on_change_target_cluster_updated),
                    );
                    ui.validation_feedback(
                        if target_cluster_valid {
                            ValidationFeedback::valid("Cluster selection captured")
                        } else {
                            ValidationFeedback::invalid(
                                "Select the cluster receiving this rollout",
                            )
                        }
                        .variant(if target_cluster_valid {
                            Variant::Success
                        } else {
                            Variant::Danger
                        }),
                    );

                    ui.form_label(FormLabel::new("Runbook path").is_required());
                    ui.form_control(
                        &mut demo_state.form_runbook_path,
                        FormControl::new()
                            .placeholder("runbooks/services/api-gateway/canary-rollout.yaml")
                            .validation(validation_state(runbook_path_valid))
                            .on_change(on_change_runbook_path_updated),
                    );
                    ui.validation_feedback(
                        if runbook_path_valid {
                            ValidationFeedback::valid("Runbook route is usable")
                        } else {
                            ValidationFeedback::invalid(
                                "Use a nested runbook path like service/name/topic",
                            )
                        }
                        .variant(if runbook_path_valid {
                            Variant::Success
                        } else {
                            Variant::Danger
                        }),
                    );
                    ui.form_text(
                        FormText::new(
                            "Resolved URL appends `?view=latest` when opened in the runbook portal.",
                        )
                        .variant(Variant::Secondary),
                    );
                },
            );
        });

        ui.col(Col::new().xs(12).lg(5), |ui| {
            ui.card(
                Card::new()
                    .title("Safety controls")
                    .outline()
                    .footer("Canary width and checklist gates"),
                |ui| {
                    ui.form_label(FormLabel::new("Canary traffic (%)").is_required());
                    ui.form_range(
                        &mut demo_state.form_canary_percent,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .label("Traffic")
                            .validation(validation_state(canary_percent_valid))
                            .on_change(on_change_canary_percent_updated),
                    );
                    ui.validation_feedback(
                        if canary_percent_valid {
                            ValidationFeedback::valid(
                                "Canary width is within policy (5-50%)",
                            )
                        } else {
                            ValidationFeedback::invalid(
                                "Policy violation: production canary must stay between 5% and 50%",
                            )
                        }
                        .variant(if canary_percent_valid {
                            Variant::Success
                        } else {
                            Variant::Danger
                        }),
                    );
                    ui.form_text(
                        FormText::new(
                            "Wider percentages increase blast radius. Start narrow for incident mitigation changes.",
                        )
                        .variant(Variant::Secondary),
                    );

                    ui.form_label(FormLabel::new("FormRange variant gallery").muted(true));
                    ui.form_range(
                        &mut demo_state.form_range_variant_primary,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Primary variant")
                            .variant(Variant::Primary),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_secondary,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Secondary variant")
                            .variant(Variant::Secondary),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_success,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Success variant")
                            .variant(Variant::Success),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_danger,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Danger variant")
                            .variant(Variant::Danger),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_warning,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Warning variant")
                            .variant(Variant::Warning),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_info,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Info variant")
                            .variant(Variant::Info),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_light,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Light variant")
                            .variant(Variant::Light),
                    );
                    ui.form_range(
                        &mut demo_state.form_range_variant_dark,
                        FormRange::new(0.0, 100.0)
                            .step(5.0)
                            .show_value(false)
                            .label("Dark variant")
                            .variant(Variant::Dark),
                    );

                    ui.form_label(FormLabel::new("Pre-flight checklist").is_required());
                    ui.form_check(
                        &mut demo_state.form_requires_db_migration,
                        FormCheck::checkbox("Requires database migration")
                            .variant(Variant::Warning)
                            .on_change(on_change_checklist_updated),
                    );
                    ui.form_check(
                        &mut demo_state.form_notify_oncall,
                        FormCheck::checkbox("Notify on-call channel before canary start")
                            .variant(Variant::Info)
                            .on_change(on_change_checklist_updated),
                    );
                    ui.form_check(
                        &mut demo_state.form_freeze_approved,
                        FormCheck::checkbox("Approved for current release freeze window")
                            .validation(validation_state(freeze_window_confirmed))
                            .variant(Variant::Danger)
                            .on_change(on_change_freeze_window_toggled),
                    );

                    ui.validation_feedback(
                        if freeze_window_confirmed {
                            ValidationFeedback::valid("Freeze-window approval confirmed")
                        } else {
                            ValidationFeedback::invalid(
                                "Approval is required before queueing this request",
                            )
                        }
                        .variant(if freeze_window_confirmed {
                            Variant::Success
                        } else {
                            Variant::Danger
                        }),
                    );
                },
            );
        });
    });

    ui.card(
        Card::new()
            .title("Submission state")
            .outline()
            .footer("Live snapshot of request readiness"),
        |ui| {
            ui.badge(Badge::secondary(format!(
                "Target cluster: {target_cluster_label}"
            )));
            ui.badge(Badge::info(format!("Canary: {canary_percent:.0}%")));
            ui.badge(Badge::dark(format!(
                "DB migration: {}",
                yes_no(requires_db_migration)
            )));

            ui.validation_feedback(
                if form_ready {
                    ValidationFeedback::valid("Request is ready to queue")
                } else {
                    ValidationFeedback::invalid(
                        "Resolve highlighted fields and confirm freeze approval",
                    )
                }
                .variant(if form_ready {
                    Variant::Success
                } else {
                    Variant::Danger
                }),
            );

            ui.button(
                Button::primary("Queue rollout request")
                    .enabled(form_ready)
                    .on_click(on_click_queue_rollout_request),
            );
            ui.button(
                Button::secondary("Save draft")
                    .hover_text("Persist this draft for later review")
                    .on_click(on_click_save_draft),
            );
            ui.badge(
                Badge::dark(format!("Last action: {}", state::last_action(ui.context()))).as_pill(),
            );
        },
    );
}

fn looks_like_email(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed.contains('@') && trimmed.contains('.')
}

fn login_password_looks_valid(value: &str) -> bool {
    let trimmed = value.trim();
    let has_digit = trimmed.chars().any(|ch| ch.is_ascii_digit());
    let has_alpha = trimmed.chars().any(|ch| ch.is_ascii_alphabetic());
    trimmed.len() >= 8 && has_digit && has_alpha
}

fn validation_state(valid: bool) -> FormValidationState {
    if valid {
        FormValidationState::Valid
    } else {
        FormValidationState::Invalid
    }
}

fn selected_cluster_label(index: usize) -> &'static str {
    match index {
        0 => shared::CLUSTERS[0].name,
        1 => shared::CLUSTERS[1].name,
        2 => shared::CLUSTERS[2].name,
        _ => "unselected",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn on_change_login_username_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: login username edited");
}

fn on_change_login_password_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: login password edited");
}

fn on_click_login(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: operator sign-in submitted");
}

fn on_change_summary_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: change summary edited");
}

fn on_change_requester_email_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: requester email edited");
}

fn on_change_target_cluster_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: target cluster changed");
}

fn on_change_runbook_path_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: runbook path edited");
}

fn on_change_canary_percent_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: canary percentage adjusted");
}

fn on_change_checklist_updated(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: checklist updated");
}

fn on_change_freeze_window_toggled(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: freeze approval toggled");
}

fn on_click_queue_rollout_request(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: rollout request queued");
}

fn on_click_save_draft(ctx: &egui::Context) {
    state::set_last_action(ctx, "Forms: draft saved");
}
