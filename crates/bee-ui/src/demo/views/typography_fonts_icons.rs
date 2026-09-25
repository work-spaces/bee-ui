use crate::{
    prelude::*,
    theme::fonts::{FontFace, ensure_face_registered},
};

use super::shared;

pub(super) fn render_view_typography_fonts_icons(ui: &mut UiRoot<'_>) {
    let _ = ensure_face_registered(ui.context(), FontFace::PropoRegular);
    let _ = ensure_face_registered(ui.context(), FontFace::PropoSemiBold);
    let _ = ensure_face_registered(ui.context(), FontFace::MonoSemiBold);

    ui.card(
        Card::new()
            .title("Intro — typography, fonts, and iconography")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Use semantic text primitives for operator-facing language: terse status lines, clear labels, and actionable icon cues.",
            ));
            ui.badge(Badge::info("Text + Label + Span styling + Icon runs").as_pill());
        },
    );

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new().title("Variant gallery — Text").outline(),
                |ui| {
                    shared::for_each_variant(|name, variant| {
                        ui.text(
                            Text::new(format!(
                                "{name}: {} incident summary updated in the timeline",
                                name.to_lowercase()
                            ))
                            .variant(variant),
                        );
                    });
                    ui.text(Text::new("Muted helper text for secondary context").subtle());
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Variant gallery — Label sizes and states")
                    .outline(),
                |ui| {
                    shared::for_each_variant(|name, variant| {
                        ui.label(Label::new(format!("{name} label")).variant(variant));
                    });
                    ui.label(
                        Label::new("Small section meta label")
                            .small()
                            .variant(Variant::Secondary),
                    );
                    ui.label(Label::new("Medium label (default)").variant(Variant::Primary));
                    ui.label(
                        Label::new("Large emphasis label")
                            .large()
                            .variant(Variant::Warning),
                    );
                    ui.label(Label::new("Muted metadata label").small().subtle());
                },
            );
        });
    });

    ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — rich span styling")
                    .outline(),
                |ui| {
                    ui.text(Text::new(
                        RichText::new()
                            .text("Deploy policy: ")
                            .styled_text(
                                "canary",
                                SpanTextStyle::new()
                                    .strong(true)
                                    .font(SpanFont::Monospace)
                                    .size(15.0),
                            )
                            .text(" to prod-b with ")
                            .styled_text(
                                "25% traffic",
                                SpanTextStyle::new()
                                    .underline(true)
                                    .size(15.0)
                                    .font(SpanFont::Embedded(FontFace::PropoSemiBold)),
                            ),
                    ));

                    ui.text(Text::new(
                        RichText::new()
                            .text("SLO note: ")
                            .styled_text(
                                "latency budget currently exhausted",
                                SpanTextStyle::new()
                                    .italics(true)
                                    .weak(true)
                                    .font(SpanFont::Proportional),
                            )
                            .text(" • ")
                            .styled_text(
                                "error spike",
                                SpanTextStyle::new()
                                    .strikethrough(true)
                                    .font(SpanFont::Embedded(FontFace::PropoRegular)),
                            ),
                    ));
                },
            );
        });

        ui.col(Col::new().xs(12).lg(6), |ui| {
            ui.card(
                Card::new()
                    .title("Composed example — font selection")
                    .outline(),
                |ui| {
                    ui.text(Text::new(RichText::new().styled_text(
                        "SpanFont::Proportional — Fleet health summary",
                        SpanTextStyle::new().font(SpanFont::Proportional),
                    )));
                    ui.text(Text::new(RichText::new().styled_text(
                        "SpanFont::Monospace — rollout_id=canary-2042",
                        SpanTextStyle::new().font(SpanFont::Monospace),
                    )));
                    ui.text(Text::new(RichText::new().styled_text(
                        "SpanFont::Embedded(FontFace::MonoSemiBold)",
                        SpanTextStyle::new().font(SpanFont::Embedded(FontFace::MonoSemiBold)),
                    )));
                    ui.text(Text::new(RichText::new().styled_text(
                        "SpanFont::Embedded(FontFace::PropoRegular)",
                        SpanTextStyle::new().font(SpanFont::Embedded(FontFace::PropoRegular)),
                    )));
                },
            );
        });
    });

    ui.card(
        Card::new()
            .title("Composed example — inline icons in text spans")
            .outline()
            .footer("Icon variant tints adapt by theme for contrast in light and dark modes"),
        |ui| {
            ui.text(Text::new(
                RichText::new()
                    .icon(Icon::leading(SvgIcon::CheckCircleFill).size(14.0))
                    .text("Deployment succeeded on prod-a"),
            ));

            ui.text(Text::new(
                RichText::new()
                    .text("Open incident timeline")
                    .icon(SpanIcon::trailing(SvgIcon::ArrowRight).size(14.0)),
            ));

            ui.text(Text::new(
                RichText::new()
                    .text("Alerts")
                    .icon(
                        SpanIcon::between(SvgIcon::ExclamationTriangleFill)
                            .size(16.0)
                            .variant(Variant::Warning),
                    )
                    .text("Escalate"),
            ));

            ui.text(Text::new(
                RichText::new()
                    .icon(
                        SpanIcon::leading(SvgIcon::InfoCircle)
                            .size(12.0)
                            .variant(Variant::Info),
                    )
                    .text("Operator note: staging rollback is in progress"),
            ));
        },
    );
}
