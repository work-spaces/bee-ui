use super::bootstrap_tokens::{self, ColorMode};

pub fn style() -> egui::Style {
    let spacing = bootstrap_tokens::spacing();
    let typography = bootstrap_tokens::typography();
    let radius = bootstrap_tokens::radius();
    let controls = bootstrap_tokens::controls();
    let tokens = bootstrap_tokens::colors(ColorMode::Dark);

    let mut style = egui::Theme::Dark.default_style();
    style.spacing.item_spacing = egui::vec2(spacing.sm, spacing.sm);
    style.spacing.button_padding = egui::vec2(controls.button_padding_x, controls.button_padding_y);
    style.spacing.interact_size.y = controls.control_height.max(
        (typography.body_font_size * typography.body_line_height)
            + (controls.button_padding_y * 2.0)
            + (controls.border_width * 2.0),
    );
    style.spacing.indent = spacing.md;

    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(typography.sm, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(typography.body_font_size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(typography.body_font_size, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(typography.lg, egui::FontFamily::Proportional),
    );

    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(tokens.body_color);
    visuals.weak_text_color = Some(tokens.secondary_color);
    visuals.hyperlink_color = tokens.link_color;

    visuals.selection.bg_fill = tokens.link_color;
    visuals.selection.stroke = egui::Stroke::new(controls.border_width, tokens.emphasis_color);

    visuals.panel_fill = tokens.body_bg;
    visuals.window_fill = tokens.body_bg;
    visuals.faint_bg_color = tokens.tertiary_bg;
    visuals.extreme_bg_color = tokens.secondary_bg;
    visuals.code_bg_color = tokens.dark.bg_subtle;

    visuals.warn_fg_color = tokens.warning.text_emphasis;
    visuals.error_fg_color = tokens.danger.text_emphasis;

    visuals.widgets.noninteractive.bg_fill = tokens.body_bg;
    visuals.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(controls.border_width, tokens.border_color);
    visuals.widgets.noninteractive.fg_stroke.color = tokens.body_color;

    visuals.widgets.inactive.bg_fill = tokens.tertiary_bg;
    visuals.widgets.inactive.bg_stroke =
        egui::Stroke::new(controls.border_width, tokens.border_color);
    visuals.widgets.inactive.fg_stroke.color = tokens.body_color;

    visuals.widgets.hovered.bg_fill = tokens.dark.bg_subtle;
    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(controls.border_width, tokens.light.border_subtle);
    visuals.widgets.hovered.fg_stroke.color = tokens.emphasis_color;

    visuals.widgets.active.bg_fill = tokens.dark.bg_subtle;
    visuals.widgets.active.bg_stroke =
        egui::Stroke::new(controls.border_width, tokens.focus_ring_color);
    visuals.widgets.active.fg_stroke.color = tokens.emphasis_color;

    visuals.widgets.open.bg_fill = tokens.dark.bg_subtle;
    visuals.widgets.open.bg_stroke =
        egui::Stroke::new(controls.border_width, tokens.focus_ring_color);
    visuals.widgets.open.fg_stroke.color = tokens.emphasis_color;

    visuals.window_corner_radius = egui::CornerRadius::same(radius.lg);
    visuals.menu_corner_radius = egui::CornerRadius::same(radius.md);

    style.visuals = visuals;
    style
}
