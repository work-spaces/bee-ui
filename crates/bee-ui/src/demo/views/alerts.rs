use crate::prelude::*;

use super::shared;

pub(super) fn render_view_alerts(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — alert patterns").outline(), |ui| {
        ui.text(Text::new(
            "Alerts communicate actionable operational context: incidents, degradation, quota pressure, and deployment outcomes.",
        ));
    });

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Variant gallery — filled alerts")
                    .outline(),
                |ui| {
                    shared::for_each_variant(|name, variant| {
                        ui.alert(Alert::new(alert_message(name)).variant(variant));
                    });
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Variant gallery — outlined alerts")
                    .outline(),
                |ui| {
                    shared::for_each_variant(|name, variant| {
                        ui.alert(Alert::new(alert_message(name)).variant(variant).outline());
                    });
                },
            );
        });
    });

    ui.card(
        Card::new()
            .title("Composed example — incident feed")
            .outline()
            .footer("Real-world messages instead of placeholder text"),
        |ui| {
            ui.alert(Alert::warning(
                "INC-1042: api-gateway p95 latency exceeded SLO for 12m in prod-b",
            ));
            ui.alert(Alert::danger(
                RichText::new().text(
                "INC-1043: scheduler admission queue saturation is blocking canary promotion").icon(SpanIcon::between(SvgIcon::AirplaneFill)),
            ));
            ui.alert(Alert::info(
                "Quota nearing threshold: worker job burst consumed 84% of reserved CPU in staging",
            ));
            ui.alert(Alert::success(
                "Rollback completed: worker deployment reverted and error rate returned below 0.2%",
            ));
        },
    );
}

fn alert_message(name: &str) -> String {
    match name {
        "Primary" => "Control-plane notice: deployment window opened for prod-a".to_owned(),
        "Secondary" => "Background check: certificate rotation plan queued".to_owned(),
        "Success" => "Deploy success: scheduler canary promoted to 100% traffic".to_owned(),
        "Danger" => "Incident: api-gateway serving errors above emergency threshold".to_owned(),
        "Warning" => "Warning: p95 latency trending upward on prod-b".to_owned(),
        "Info" => "Info: new runbook annotation added for rollback procedure".to_owned(),
        "Light" => "Maintenance note: staging node reboot scheduled at 03:00 UTC".to_owned(),
        "Dark" => "Audit event: operator initiated manual drain on node pool b".to_owned(),
        _ => "Operator message".to_owned(),
    }
}
