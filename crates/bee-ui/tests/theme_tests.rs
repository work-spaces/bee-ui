use bee_ui::core::theme::{FormValidationState, NavItemState};
use bee_ui::core::{UiTheme, Variant};

fn all_variants() -> [Variant; 8] {
    [
        Variant::Primary,
        Variant::Secondary,
        Variant::Success,
        Variant::Danger,
        Variant::Warning,
        Variant::Info,
        Variant::Light,
        Variant::Dark,
    ]
}

#[test]
fn default_theme_uses_bootstrap_breakpoints() {
    let theme = UiTheme::default();
    assert_eq!(theme.breakpoints.sm, 576);
    assert_eq!(theme.breakpoints.md, 768);
    assert_eq!(theme.breakpoints.lg, 992);
    assert_eq!(theme.breakpoints.xl, 1200);
    assert_eq!(theme.breakpoints.xxl, 1400);
}

#[test]
fn variant_resolver_maps_all_variants_to_palette_tones() {
    let theme = UiTheme::default();

    for variant in all_variants() {
        assert_eq!(
            theme.resolve_variant(variant),
            theme.palette.resolve_variant(variant)
        );
    }
}

#[test]
fn variant_resolver_maps_to_distinct_variant_tones() {
    let theme = UiTheme::default();

    let primary = theme.resolve_variant(Variant::Primary);
    let success = theme.resolve_variant(Variant::Success);
    let danger = theme.resolve_variant(Variant::Danger);

    assert_ne!(primary.fill, success.fill);
    assert_ne!(success.fill, danger.fill);
    assert_ne!(primary.border, danger.border);
}

#[test]
fn component_style_resolvers_reflect_props() {
    let theme = UiTheme::default();

    let card_default = theme.resolve_card_style(Variant::Primary, false);
    let card_outlined = theme.resolve_card_style(Variant::Primary, true);
    assert_ne!(card_default.fill, card_outlined.fill);

    let badge_square = theme.resolve_badge_style(Variant::Info, false);
    let badge_pill = theme.resolve_badge_style(Variant::Info, true);
    assert!(badge_pill.corner_radius > badge_square.corner_radius);

    let label_regular = theme.resolve_label_style(
        Variant::Secondary,
        bee_ui::core::theme::TextSize::Medium,
        false,
    );
    let label_muted = theme.resolve_label_style(
        Variant::Secondary,
        bee_ui::core::theme::TextSize::Medium,
        true,
    );
    assert_ne!(label_regular.text, label_muted.text);

    let button_primary = theme.resolve_button_style(Variant::Primary);
    let button_warning = theme.resolve_button_style(Variant::Warning);
    assert_ne!(button_primary.fill, button_warning.fill);
}

#[test]
fn badge_style_defaults_to_bold_weight() {
    let theme = UiTheme::default();
    let badge_style = theme.resolve_badge_style(Variant::Primary, false);

    assert!(badge_style.text_weight >= 600.0);
}

#[test]
fn badge_and_button_styles_are_consistent_with_variant_tones() {
    let theme = UiTheme::default();

    for variant in all_variants() {
        let tone = theme.resolve_variant(variant);
        let badge_style = theme.resolve_badge_style(variant, false);
        let button_style = theme.resolve_button_style(variant);

        assert_eq!(badge_style.fill, tone.fill);
        assert_eq!(badge_style.text, tone.on_fill);
        assert_eq!(badge_style.border, tone.border);

        assert_eq!(button_style.fill, tone.fill);
        assert_eq!(button_style.text, tone.on_fill);
        assert_eq!(button_style.border, tone.border);
    }
}

#[test]
fn outlined_cards_use_surface_fill_and_variant_border() {
    let theme = UiTheme::default();

    for variant in all_variants() {
        let tone = theme.resolve_variant(variant);
        let outlined = theme.resolve_card_style(variant, true);

        assert_eq!(outlined.fill, theme.palette.surface);
        assert_eq!(outlined.border, tone.border);
    }
}

#[test]
fn label_style_uses_variant_text_when_not_muted() {
    let theme = UiTheme::default();

    for variant in all_variants() {
        let tone = theme.resolve_variant(variant);
        let label =
            theme.resolve_label_style(variant, bee_ui::core::theme::TextSize::Medium, false);

        assert_eq!(label.text, tone.text);
    }
}

#[test]
fn phase_zero_theme_resolvers_cover_new_component_families() {
    let theme = UiTheme::default();

    let alert_filled = theme.resolve_alert_style(Variant::Danger, false);
    let alert_outlined = theme.resolve_alert_style(Variant::Danger, true);
    assert_ne!(alert_filled.fill, alert_outlined.fill);

    let nav_default = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Default);
    let nav_active = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Active);
    assert_ne!(nav_default.fill, nav_active.fill);

    let overlay = theme.resolve_overlay_style(Variant::Info);
    assert_eq!(overlay.panel_fill, theme.palette.surface);

    let form_valid =
        theme.resolve_form_control_style(Variant::Primary, FormValidationState::Valid, false);
    let form_invalid =
        theme.resolve_form_control_style(Variant::Primary, FormValidationState::Invalid, false);
    assert_ne!(form_valid.validation_border, form_invalid.validation_border);

    let table_plain = theme.resolve_table_style(Variant::Secondary, false);
    let table_striped = theme.resolve_table_style(Variant::Secondary, true);
    assert_ne!(table_plain.row_alt_fill, table_striped.row_alt_fill);
}

#[test]
fn phase_one_theme_resolvers_cover_component_states() {
    let theme = UiTheme::default();

    let progress_primary = theme.resolve_progress_style(Variant::Primary);
    let progress_success = theme.resolve_progress_style(Variant::Success);
    assert_ne!(progress_primary.fill, progress_success.fill);

    let pagination_default =
        theme.resolve_pagination_item_style(Variant::Primary, NavItemState::Default);
    let pagination_active =
        theme.resolve_pagination_item_style(Variant::Primary, NavItemState::Active);
    let pagination_disabled =
        theme.resolve_pagination_item_style(Variant::Primary, NavItemState::Disabled);
    assert_ne!(pagination_default.fill, pagination_active.fill);
    assert_ne!(pagination_active.text, pagination_disabled.text);

    let spinner = theme.resolve_spinner_style(Variant::Warning);
    assert!(spinner.text_size > 0.0);

    let placeholder_pulse = theme.resolve_placeholder_style(Variant::Secondary, true);
    let placeholder_static = theme.resolve_placeholder_style(Variant::Secondary, false);
    assert_ne!(placeholder_pulse.fill, placeholder_static.fill);

    let button_group = theme.resolve_button_group_style();
    assert!(button_group.gap >= 0.0);

    let list_flush =
        theme.resolve_list_group_item_style(Variant::Info, NavItemState::Default, true);
    let list_rounded =
        theme.resolve_list_group_item_style(Variant::Info, NavItemState::Default, false);
    assert_ne!(list_flush.corner_radius, list_rounded.corner_radius);
}

#[test]
fn phase_two_theme_resolvers_cover_navigation_family() {
    let theme = UiTheme::default();

    let nav_default = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Default);
    let nav_active = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Active);
    assert_ne!(nav_default.fill, nav_active.fill);

    let navbar_dark = theme.resolve_navbar_style(Variant::Dark);
    let navbar_light = theme.resolve_navbar_style(Variant::Light);
    assert_ne!(navbar_dark.fill, navbar_light.fill);
    assert_ne!(navbar_dark.brand_text, navbar_dark.border);

    let dropdown_closed = theme.resolve_dropdown_style(Variant::Info, false);
    let dropdown_open = theme.resolve_dropdown_style(Variant::Info, true);
    assert_ne!(dropdown_closed.toggle_fill, dropdown_open.toggle_fill);
    assert_ne!(dropdown_open.item_text, dropdown_open.item_disabled_text);
}

#[test]
fn phase_three_theme_resolvers_cover_forms_family() {
    let theme = UiTheme::default();

    let label_default = theme.resolve_form_label_style(Variant::Primary, false);
    let label_muted = theme.resolve_form_label_style(Variant::Primary, true);
    assert_ne!(label_default.text, label_muted.text);

    let form_text_default =
        theme.resolve_form_text_style(Variant::Secondary, FormValidationState::None);
    let form_text_invalid =
        theme.resolve_form_text_style(Variant::Secondary, FormValidationState::Invalid);
    assert_ne!(form_text_default.text, form_text_invalid.text);

    let form_control_valid =
        theme.resolve_form_control_style(Variant::Info, FormValidationState::Valid, false);
    let form_control_invalid =
        theme.resolve_form_control_style(Variant::Info, FormValidationState::Invalid, false);
    assert_ne!(
        form_control_valid.validation_border,
        form_control_invalid.validation_border
    );

    let check_enabled =
        theme.resolve_form_check_style(Variant::Success, FormValidationState::None, false);
    let check_disabled =
        theme.resolve_form_check_style(Variant::Success, FormValidationState::None, true);
    assert_ne!(check_enabled.text, check_disabled.disabled_text);

    let range_valid =
        theme.resolve_form_range_style(Variant::Primary, FormValidationState::Valid, false);
    let range_invalid =
        theme.resolve_form_range_style(Variant::Primary, FormValidationState::Invalid, false);
    assert_ne!(range_valid.text, range_invalid.text);

    let feedback_valid =
        theme.resolve_validation_feedback_style(Variant::Info, FormValidationState::Valid);
    let feedback_invalid =
        theme.resolve_validation_feedback_style(Variant::Info, FormValidationState::Invalid);
    assert_ne!(feedback_valid.text, feedback_invalid.text);
}
