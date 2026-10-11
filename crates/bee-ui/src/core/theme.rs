use crate::{
    core::Variant,
    layout::Breakpoints,
    theme::bootstrap_tokens::{self, ColorMode, VariantTokens},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    pub fn from_dark_mode(dark_mode: bool) -> Self {
        if dark_mode { Self::Dark } else { Self::Light }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariantTone {
    pub fill: egui::Color32,
    pub on_fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub bg_subtle: egui::Color32,
    pub border_subtle: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SemanticPalette {
    pub primary: VariantTone,
    pub secondary: VariantTone,
    pub success: VariantTone,
    pub danger: VariantTone,
    pub warning: VariantTone,
    pub info: VariantTone,
    pub light: VariantTone,
    pub dark: VariantTone,
    pub surface: egui::Color32,
    pub surface_secondary: egui::Color32,
    pub surface_tertiary: egui::Color32,
    pub border_subtle: egui::Color32,
    pub text_default: egui::Color32,
    pub text_muted: egui::Color32,
    pub text_tertiary: egui::Color32,
    pub link: egui::Color32,
    pub focus_ring: egui::Color32,
    pub form_valid: egui::Color32,
    pub form_invalid: egui::Color32,
}

impl SemanticPalette {
    pub fn resolve_variant(&self, variant: Variant) -> VariantTone {
        match variant {
            Variant::Primary => self.primary,
            Variant::Secondary => self.secondary,
            Variant::Success => self.success,
            Variant::Danger => self.danger,
            Variant::Warning => self.warning,
            Variant::Info => self.info,
            Variant::Light => self.light,
            Variant::Dark => self.dark,
        }
    }

    pub fn bootstrap(mode: ThemeMode) -> Self {
        let tokens = bootstrap_tokens::colors(match mode {
            ThemeMode::Light => ColorMode::Light,
            ThemeMode::Dark => ColorMode::Dark,
        });

        Self {
            primary: map_variant(tokens.primary),
            secondary: map_variant(tokens.secondary),
            success: map_variant(tokens.success),
            danger: map_variant(tokens.danger),
            warning: map_variant(tokens.warning),
            info: map_variant(tokens.info),
            light: map_variant(tokens.light),
            dark: map_variant(tokens.dark),
            surface: tokens.body_bg,
            surface_secondary: tokens.secondary_bg,
            surface_tertiary: tokens.tertiary_bg,
            border_subtle: tokens.border_color,
            text_default: tokens.body_color,
            text_muted: tokens.secondary_color,
            text_tertiary: tokens.tertiary_color,
            link: tokens.link_color,
            focus_ring: tokens.focus_ring_color,
            form_valid: tokens.form_valid_color,
            form_invalid: tokens.form_invalid_color,
        }
    }
}

impl Default for SemanticPalette {
    fn default() -> Self {
        Self::bootstrap(ThemeMode::Light)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpacingScale {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        let tokens = bootstrap_tokens::spacing();
        Self {
            xs: tokens.xs,
            sm: tokens.sm,
            md: tokens.md,
            lg: tokens.lg,
            xl: tokens.xl,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadiusScale {
    pub sm: u8,
    pub md: u8,
    pub lg: u8,
    pub pill: u8,
}

impl Default for RadiusScale {
    fn default() -> Self {
        let tokens = bootstrap_tokens::radius();
        Self {
            sm: tokens.sm,
            md: tokens.md,
            lg: tokens.lg,
            pill: tokens.pill,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypographyScale {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl Default for TypographyScale {
    fn default() -> Self {
        let tokens = bootstrap_tokens::typography();
        Self {
            xs: tokens.xs,
            sm: tokens.sm,
            md: tokens.md,
            lg: tokens.lg,
        }
    }
}

fn map_variant(tokens: VariantTokens) -> VariantTone {
    VariantTone {
        fill: tokens.fill,
        on_fill: tokens.on_fill,
        text: tokens.text_emphasis,
        border: tokens.fill,
        bg_subtle: tokens.bg_subtle,
        border_subtle: tokens.border_subtle,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardStyle {
    pub fill: egui::Color32,
    pub cap_fill: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub header_text: egui::Color32,
    pub body_text: egui::Color32,
    pub footer_text: egui::Color32,
    pub body_padding_x: f32,
    pub body_padding_y: f32,
    pub header_gap: f32,
    pub footer_gap: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BadgeStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub text_size: f32,
    pub text_weight: f32,
    pub padding_x: f32,
    pub padding_y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LabelStyle {
    pub text: egui::Color32,
    pub text_size: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub hover_fill: egui::Color32,
    pub hover_text: egui::Color32,
    pub hover_border: egui::Color32,
    pub active_fill: egui::Color32,
    pub active_text: egui::Color32,
    pub active_border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub text_size: f32,
    pub text_weight: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub disabled_opacity: f32,
    pub focus_ring_color: egui::Color32,
    pub focus_ring_width: f32,
    pub focus_ring_opacity: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavItemState {
    #[default]
    Default,
    Active,
    Disabled,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormValidationState {
    #[default]
    None,
    Valid,
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlertStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub padding_x: f32,
    pub padding_y: f32,
    pub margin_bottom: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavItemStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub hover_fill: egui::Color32,
    pub hover_text: egui::Color32,
    pub hover_border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub padding_x: f32,
    pub padding_y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverlayStyle {
    pub backdrop: egui::Color32,
    pub panel_fill: egui::Color32,
    pub panel_text: egui::Color32,
    pub panel_border: egui::Color32,
    pub panel_border_width: f32,
    pub panel_corner_radius: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormControlStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub placeholder: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub focus_border: egui::Color32,
    pub focus_ring_color: egui::Color32,
    pub focus_ring_width: f32,
    pub validation_border: egui::Color32,
    pub corner_radius: u8,
    pub text_size: f32,
    pub text_weight: f32,
    pub padding_x: f32,
    pub padding_y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormLabelStyle {
    pub text: egui::Color32,
    pub required_text: egui::Color32,
    pub text_size: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormTextStyle {
    pub text: egui::Color32,
    pub text_size: f32,
    pub margin_top: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormCheckStyle {
    pub text: egui::Color32,
    pub disabled_text: egui::Color32,
    pub text_size: f32,
    pub disabled_opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormRangeStyle {
    pub text: egui::Color32,
    pub disabled_text: egui::Color32,
    pub text_size: f32,
    pub disabled_opacity: f32,
    pub label_gap: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ValidationFeedbackStyle {
    pub text: egui::Color32,
    pub text_size: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TableStyle {
    pub header_fill: egui::Color32,
    pub header_text: egui::Color32,
    pub row_fill: egui::Color32,
    pub row_alt_fill: egui::Color32,
    pub row_hover_fill: egui::Color32,
    pub row_active_fill: egui::Color32,
    pub row_text: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub cell_padding: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressStyle {
    pub fill: egui::Color32,
    pub track: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub height: f32,
    pub text_size: f32,
    pub transition_seconds: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CloseButtonStyle {
    pub text: egui::Color32,
    pub disabled_text: egui::Color32,
    pub text_size: f32,
    pub corner_radius: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaginationItemStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub hover_fill: egui::Color32,
    pub hover_text: egui::Color32,
    pub hover_border: egui::Color32,
    pub active_fill: egui::Color32,
    pub active_text: egui::Color32,
    pub active_border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub text_size: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub focus_ring_color: egui::Color32,
    pub focus_ring_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerStyle {
    pub color: egui::Color32,
    pub text: egui::Color32,
    pub text_size: f32,
    pub size: f32,
    pub border_width: f32,
    pub animation_speed_seconds: f32,
    pub label_gap: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaceholderStyle {
    pub fill: egui::Color32,
    pub pulse_fill: egui::Color32,
    pub corner_radius: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonGroupStyle {
    pub gap: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListGroupItemStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub text_size: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub hover_fill: egui::Color32,
    pub hover_text: egui::Color32,
    pub action_active_fill: egui::Color32,
    pub action_active_text: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavbarStyle {
    pub fill: egui::Color32,
    pub text: egui::Color32,
    pub border: egui::Color32,
    pub border_width: f32,
    pub corner_radius: u8,
    pub brand_text: egui::Color32,
    pub active_text: egui::Color32,
    pub disabled_text: egui::Color32,
    pub padding_x: i8,
    pub padding_y: i8,
    pub item_gap: f32,
    pub nav_link_padding_x: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropdownStyle {
    pub toggle_fill: egui::Color32,
    pub toggle_text: egui::Color32,
    pub toggle_border: egui::Color32,
    pub toggle_border_width: f32,
    pub toggle_corner_radius: u8,
    pub toggle_padding_x: f32,
    pub toggle_padding_y: f32,
    pub menu_fill: egui::Color32,
    pub menu_border: egui::Color32,
    pub menu_border_width: f32,
    pub menu_corner_radius: u8,
    pub menu_padding_x: i8,
    pub menu_padding_y: i8,
    pub menu_divider: egui::Color32,
    pub item_text: egui::Color32,
    pub item_hover_text: egui::Color32,
    pub item_disabled_text: egui::Color32,
    pub item_active_text: egui::Color32,
    pub item_active_fill: egui::Color32,
    pub item_hover_fill: egui::Color32,
    pub item_corner_radius: u8,
    pub item_gap: f32,
    pub item_padding_x: f32,
    pub item_padding_y: f32,
    pub menu_shadow: bootstrap_tokens::ShadowToken,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiTheme {
    pub breakpoints: Breakpoints,
    pub spacing: SpacingScale,
    pub radius: RadiusScale,
    pub typography: TypographyScale,
    pub mode: ThemeMode,
    pub palette: SemanticPalette,
    pub palette_light: SemanticPalette,
    pub palette_dark: SemanticPalette,
}

impl UiTheme {
    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
        self.palette = match mode {
            ThemeMode::Light => self.palette_light,
            ThemeMode::Dark => self.palette_dark,
        };
    }

    pub fn sync_mode_from_dark_mode(&mut self, dark_mode: bool) {
        self.set_mode(ThemeMode::from_dark_mode(dark_mode));
    }

    pub fn resolve_variant(&self, variant: Variant) -> VariantTone {
        self.palette.resolve_variant(variant)
    }

    fn color_mode(&self) -> ColorMode {
        match self.mode {
            ThemeMode::Light => ColorMode::Light,
            ThemeMode::Dark => ColorMode::Dark,
        }
    }

    pub fn resolve_card_style(&self, variant: Variant, outlined: bool) -> CardStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::card_component(self.color_mode());

        CardStyle {
            fill: self.palette.surface,
            cap_fill: if outlined {
                egui::Color32::TRANSPARENT
            } else {
                tone.fill
            },
            border: tone.border,
            border_width: f32::from(tokens.border_width),
            corner_radius: tokens.border_radius,
            header_text: if outlined {
                self.palette.text_default
            } else {
                tone.on_fill
            },
            body_text: self.palette.text_default,
            footer_text: self.palette.text_muted,
            body_padding_x: f32::from(tokens.spacer_x),
            body_padding_y: f32::from(tokens.spacer_y),
            header_gap: f32::from(tokens.cap_padding_y),
            footer_gap: f32::from(tokens.cap_padding_y),
        }
    }

    pub fn resolve_badge_style(&self, variant: Variant, pill: bool) -> BadgeStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::badge_component();

        BadgeStyle {
            fill: tone.fill,
            text: tone.on_fill,
            border: tone.border,
            border_width: 1.0,
            corner_radius: if pill {
                self.radius.pill
            } else {
                tokens.border_radius
            },
            text_size: tokens.font_size,
            text_weight: tokens.font_weight,
            padding_x: tokens.padding_x,
            padding_y: tokens.padding_y,
        }
    }

    pub fn resolve_label_style(&self, variant: Variant, size: TextSize, muted: bool) -> LabelStyle {
        let tone = self.resolve_variant(variant);

        let text = if muted {
            self.palette.text_muted
        } else {
            tone.text
        };

        let text_size = match size {
            TextSize::Small => self.typography.xs,
            TextSize::Medium => self.typography.md,
            TextSize::Large => self.typography.lg,
        };

        LabelStyle { text, text_size }
    }

    pub fn resolve_button_style(&self, variant: Variant) -> ButtonStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::button_component();

        let hover_fill = darken(tone.fill, 0.10);
        let active_fill = darken(tone.fill, 0.16);

        ButtonStyle {
            fill: tone.fill,
            text: tone.on_fill,
            border: tone.border,
            hover_fill,
            hover_text: tone.on_fill,
            hover_border: hover_fill,
            active_fill,
            active_text: tone.on_fill,
            active_border: active_fill,
            border_width: 1.0,
            corner_radius: tokens.border_radius,
            text_size: tokens.font_size,
            text_weight: tokens.font_weight,
            padding_x: tokens.padding_x,
            padding_y: tokens.padding_y,
            disabled_opacity: tokens.disabled_opacity,
            focus_ring_color: rgba_with_opacity(tone.fill, tokens.focus_ring_opacity),
            focus_ring_width: tokens.focus_ring_width,
            focus_ring_opacity: tokens.focus_ring_opacity,
        }
    }

    pub fn resolve_progress_style(&self, variant: Variant) -> ProgressStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::progress_component(self.color_mode());

        ProgressStyle {
            fill: tone.fill,
            track: tokens.bg,
            text: tokens.bar_color,
            border: tone.border_subtle,
            border_width: 1.0,
            corner_radius: tokens.border_radius,
            height: tokens.height,
            text_size: tokens.font_size,
            transition_seconds: tokens.bar_transition_seconds,
        }
    }

    pub fn resolve_close_button_style(&self) -> CloseButtonStyle {
        CloseButtonStyle {
            text: self.palette.text_muted,
            disabled_text: self.palette.text_tertiary,
            text_size: self.typography.md,
            corner_radius: self.radius.md,
        }
    }

    pub fn resolve_pagination_item_style(
        &self,
        variant: Variant,
        state: NavItemState,
    ) -> PaginationItemStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::pagination_component(self.color_mode());

        match state {
            NavItemState::Default => PaginationItemStyle {
                fill: tokens.bg,
                text: if matches!(variant, Variant::Primary) {
                    tokens.color
                } else {
                    tone.text
                },
                border: tokens.border_color,
                hover_fill: tokens.hover_bg,
                hover_text: tokens.hover_color,
                hover_border: tokens.hover_border_color,
                active_fill: tokens.active_bg,
                active_text: tokens.active_color,
                active_border: tokens.active_border_color,
                border_width: tokens.border_width,
                corner_radius: tokens.border_radius,
                text_size: tokens.font_size,
                padding_x: tokens.padding_x,
                padding_y: tokens.padding_y,
                focus_ring_color: tokens.focus_ring_color,
                focus_ring_width: tokens.focus_ring_width,
            },
            NavItemState::Active => PaginationItemStyle {
                fill: tone.fill,
                text: tone.on_fill,
                border: tone.border,
                hover_fill: tone.fill,
                hover_text: tone.on_fill,
                hover_border: tone.border,
                active_fill: tone.fill,
                active_text: tone.on_fill,
                active_border: tone.border,
                border_width: tokens.border_width,
                corner_radius: tokens.border_radius,
                text_size: tokens.font_size,
                padding_x: tokens.padding_x,
                padding_y: tokens.padding_y,
                focus_ring_color: tokens.focus_ring_color,
                focus_ring_width: tokens.focus_ring_width,
            },
            NavItemState::Disabled => PaginationItemStyle {
                fill: tokens.disabled_bg,
                text: tokens.disabled_color,
                border: tokens.disabled_border_color,
                hover_fill: tokens.disabled_bg,
                hover_text: tokens.disabled_color,
                hover_border: tokens.disabled_border_color,
                active_fill: tokens.disabled_bg,
                active_text: tokens.disabled_color,
                active_border: tokens.disabled_border_color,
                border_width: tokens.border_width,
                corner_radius: tokens.border_radius,
                text_size: tokens.font_size,
                padding_x: tokens.padding_x,
                padding_y: tokens.padding_y,
                focus_ring_color: tokens.focus_ring_color,
                focus_ring_width: tokens.focus_ring_width,
            },
        }
    }

    pub fn resolve_spinner_style(&self, variant: Variant) -> SpinnerStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::spinner_component();

        SpinnerStyle {
            color: tone.fill,
            text: tone.text,
            text_size: self.typography.sm,
            size: tokens.width,
            border_width: tokens.border_width,
            animation_speed_seconds: tokens.animation_speed_seconds,
            label_gap: self.spacing.sm,
        }
    }

    pub fn resolve_placeholder_style(&self, variant: Variant, pulse: bool) -> PlaceholderStyle {
        let tone = self.resolve_variant(variant);
        let fill_ratio = if pulse { 0.14 } else { 0.10 };
        let fill = mix(self.palette.surface, tone.fill, fill_ratio);

        PlaceholderStyle {
            fill,
            pulse_fill: mix(fill, tone.fill, 0.34),
            corner_radius: self.radius.md,
        }
    }

    pub fn resolve_button_group_style(&self) -> ButtonGroupStyle {
        ButtonGroupStyle {
            gap: self.spacing.sm,
        }
    }

    pub fn resolve_list_group_item_style(
        &self,
        variant: Variant,
        state: NavItemState,
        flush: bool,
    ) -> ListGroupItemStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::list_group_component(self.color_mode());
        let corner_radius = if flush { 0 } else { tokens.border_radius };

        match state {
            NavItemState::Default => ListGroupItemStyle {
                fill: tokens.item_bg,
                text: tokens.item_color,
                border: tokens.item_border_color,
                border_width: tokens.border_width,
                corner_radius,
                text_size: self.typography.md,
                padding_x: tokens.item_padding_x,
                padding_y: tokens.item_padding_y,
                hover_fill: tokens.action_hover_bg,
                hover_text: tokens.action_hover_color,
                action_active_fill: tokens.action_active_bg,
                action_active_text: tokens.action_active_color,
            },
            NavItemState::Active => ListGroupItemStyle {
                fill: tone.fill,
                text: tone.on_fill,
                border: tone.border,
                border_width: tokens.border_width,
                corner_radius,
                text_size: self.typography.md,
                padding_x: tokens.item_padding_x,
                padding_y: tokens.item_padding_y,
                hover_fill: tokens.active_bg,
                hover_text: tokens.active_color,
                action_active_fill: tokens.active_bg,
                action_active_text: tokens.active_color,
            },
            NavItemState::Disabled => ListGroupItemStyle {
                fill: tokens.disabled_bg,
                text: tokens.disabled_color,
                border: tokens.item_border_color,
                border_width: tokens.border_width,
                corner_radius,
                text_size: self.typography.md,
                padding_x: tokens.item_padding_x,
                padding_y: tokens.item_padding_y,
                hover_fill: tokens.disabled_bg,
                hover_text: tokens.disabled_color,
                action_active_fill: tokens.disabled_bg,
                action_active_text: tokens.disabled_color,
            },
        }
    }

    pub fn resolve_alert_style(&self, variant: Variant, outlined: bool) -> AlertStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::alert_component();

        AlertStyle {
            fill: if outlined {
                self.palette.surface
            } else {
                tone.bg_subtle
            },
            text: tone.fill,
            border: tone.border,
            border_width: tokens.border_width,
            corner_radius: tokens.border_radius,
            padding_x: tokens.padding_x,
            padding_y: tokens.padding_y,
            margin_bottom: tokens.margin_bottom,
        }
    }

    pub fn resolve_nav_item_style(&self, variant: Variant, state: NavItemState) -> NavItemStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::nav_component(self.color_mode());

        match state {
            NavItemState::Default => NavItemStyle {
                fill: egui::Color32::TRANSPARENT,
                text: if matches!(variant, Variant::Primary) {
                    tokens.link_color
                } else {
                    tone.text
                },
                border: egui::Color32::TRANSPARENT,
                hover_fill: egui::Color32::TRANSPARENT,
                hover_text: if matches!(variant, Variant::Primary) {
                    tokens.link_hover_color
                } else {
                    tone.text
                },
                hover_border: egui::Color32::TRANSPARENT,
                border_width: 0.0,
                corner_radius: self.radius.md,
                padding_x: f32::from(tokens.link_padding_x),
                padding_y: f32::from(tokens.link_padding_y),
            },
            NavItemState::Active => NavItemStyle {
                fill: tone.bg_subtle,
                text: tokens.link_active_color,
                border: tone.border_subtle,
                hover_fill: tone.bg_subtle,
                hover_text: tokens.link_active_color,
                hover_border: tone.border_subtle,
                border_width: 1.0,
                corner_radius: self.radius.md,
                padding_x: f32::from(tokens.link_padding_x),
                padding_y: f32::from(tokens.link_padding_y),
            },
            NavItemState::Disabled => NavItemStyle {
                fill: egui::Color32::TRANSPARENT,
                text: tokens.link_disabled_color,
                border: egui::Color32::TRANSPARENT,
                hover_fill: egui::Color32::TRANSPARENT,
                hover_text: tokens.link_disabled_color,
                hover_border: egui::Color32::TRANSPARENT,
                border_width: 0.0,
                corner_radius: self.radius.md,
                padding_x: f32::from(tokens.link_padding_x),
                padding_y: f32::from(tokens.link_padding_y),
            },
        }
    }

    pub fn resolve_navbar_style(&self, variant: Variant) -> NavbarStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::navbar_component(self.color_mode());

        NavbarStyle {
            fill: tone.fill,
            text: tone.on_fill,
            border: tone.border,
            border_width: 1.0,
            corner_radius: self.radius.md,
            brand_text: tokens.brand_color,
            active_text: tokens.active_color,
            disabled_text: tokens.disabled_color,
            padding_x: as_i8(f32::from(tokens.padding_x)),
            padding_y: as_i8(f32::from(tokens.padding_y)),
            item_gap: self.spacing.sm,
            nav_link_padding_x: f32::from(tokens.nav_link_padding_x),
        }
    }

    pub fn resolve_dropdown_style(&self, variant: Variant, open: bool) -> DropdownStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::dropdown_component(self.color_mode());
        let button_tokens = bootstrap_tokens::button_component();

        DropdownStyle {
            toggle_fill: if open { tone.border_subtle } else { tone.fill },
            toggle_text: tone.on_fill,
            toggle_border: tone.border,
            toggle_border_width: 1.0,
            toggle_corner_radius: self.radius.md,
            toggle_padding_x: button_tokens.padding_x,
            toggle_padding_y: button_tokens.padding_y,
            menu_fill: tokens.menu_fill,
            menu_border: tokens.menu_border_color,
            menu_border_width: f32::from(tokens.menu_border_width),
            menu_corner_radius: tokens.menu_border_radius,
            menu_padding_x: as_i8(f32::from(tokens.menu_padding_x)),
            menu_padding_y: as_i8(f32::from(tokens.menu_padding_y)),
            menu_divider: tokens.divider_color,
            item_text: tokens.item_color,
            item_hover_text: tokens.item_hover_color,
            item_disabled_text: tokens.item_disabled_color,
            item_active_text: tokens.item_active_color,
            item_active_fill: tokens.item_active_bg,
            item_hover_fill: tokens.item_hover_bg,
            item_corner_radius: self.radius.sm,
            item_gap: self.spacing.xs,
            item_padding_x: f32::from(tokens.item_padding_x),
            item_padding_y: f32::from(tokens.item_padding_y),
            menu_shadow: tokens.shadow,
        }
    }

    pub fn resolve_overlay_style(&self, variant: Variant) -> OverlayStyle {
        let tone = self.resolve_variant(variant);

        OverlayStyle {
            backdrop: egui::Color32::from_black_alpha(160),
            panel_fill: self.palette.surface,
            panel_text: self.palette.text_default,
            panel_border: tone.border_subtle,
            panel_border_width: 1.0,
            panel_corner_radius: self.radius.lg,
        }
    }

    pub fn resolve_form_control_style(
        &self,
        _variant: Variant,
        validation: FormValidationState,
        disabled: bool,
    ) -> FormControlStyle {
        let tokens = bootstrap_tokens::form_component(self.color_mode());
        let fill = if disabled {
            tokens.input_disabled_bg
        } else {
            tokens.input_bg
        };

        let text = if disabled {
            tokens.input_disabled_color
        } else {
            tokens.input_color
        };

        let validation_border = match validation {
            FormValidationState::None => tokens.input_border_color,
            FormValidationState::Valid => self.palette.form_valid,
            FormValidationState::Invalid => self.palette.form_invalid,
        };

        FormControlStyle {
            fill,
            text,
            placeholder: tokens.input_placeholder_color,
            border: tokens.input_border_color,
            border_width: tokens.input_border_width,
            focus_border: tokens.input_focus_border_color,
            focus_ring_color: self.palette.focus_ring,
            focus_ring_width: tokens.input_focus_width,
            validation_border,
            corner_radius: tokens.input_border_radius,
            text_size: tokens.input_font_size,
            text_weight: tokens.input_font_weight,
            padding_x: tokens.input_padding_x,
            padding_y: tokens.input_padding_y,
        }
    }

    pub fn resolve_form_label_style(&self, _variant: Variant, muted: bool) -> FormLabelStyle {
        FormLabelStyle {
            text: if muted {
                self.palette.text_muted
            } else {
                self.palette.text_default
            },
            required_text: self.palette.form_invalid,
            text_size: self.typography.md,
        }
    }

    pub fn resolve_form_text_style(
        &self,
        _variant: Variant,
        validation: FormValidationState,
    ) -> FormTextStyle {
        let tokens = bootstrap_tokens::form_component(self.color_mode());
        let text = match validation {
            FormValidationState::None => tokens.form_text_color,
            FormValidationState::Valid => self.palette.form_valid,
            FormValidationState::Invalid => self.palette.form_invalid,
        };

        FormTextStyle {
            text,
            text_size: tokens.form_text_font_size,
            margin_top: tokens.form_text_margin_top,
        }
    }

    pub fn resolve_form_check_style(
        &self,
        _variant: Variant,
        validation: FormValidationState,
        disabled: bool,
    ) -> FormCheckStyle {
        let tokens = bootstrap_tokens::form_component(self.color_mode());
        let text = match validation {
            FormValidationState::None => self.palette.text_default,
            FormValidationState::Valid => self.palette.form_valid,
            FormValidationState::Invalid => self.palette.form_invalid,
        };

        FormCheckStyle {
            text,
            disabled_text: if disabled {
                with_opacity(text, tokens.form_check_disabled_opacity)
            } else {
                text
            },
            text_size: tokens.input_font_size,
            disabled_opacity: tokens.form_check_disabled_opacity,
        }
    }

    pub fn resolve_form_range_style(
        &self,
        variant: Variant,
        validation: FormValidationState,
        disabled: bool,
    ) -> FormRangeStyle {
        let tokens = bootstrap_tokens::form_component(self.color_mode());
        let text = match validation {
            FormValidationState::None => self.resolve_variant(variant).text,
            FormValidationState::Valid => self.palette.form_valid,
            FormValidationState::Invalid => self.palette.form_invalid,
        };

        FormRangeStyle {
            text,
            disabled_text: if disabled {
                with_opacity(text, tokens.form_check_disabled_opacity)
            } else {
                text
            },
            text_size: tokens.input_font_size,
            disabled_opacity: tokens.form_check_disabled_opacity,
            label_gap: self.spacing.sm,
        }
    }

    pub fn resolve_validation_feedback_style(
        &self,
        _variant: Variant,
        state: FormValidationState,
    ) -> ValidationFeedbackStyle {
        let text = match state {
            FormValidationState::None => self.palette.text_muted,
            FormValidationState::Valid => self.palette.form_valid,
            FormValidationState::Invalid => self.palette.form_invalid,
        };

        ValidationFeedbackStyle {
            text,
            text_size: self.typography.sm,
        }
    }

    pub fn resolve_table_style(&self, variant: Variant, striped: bool) -> TableStyle {
        let tone = self.resolve_variant(variant);
        let tokens = bootstrap_tokens::table_component(self.color_mode());

        TableStyle {
            header_fill: tone.bg_subtle,
            header_text: tokens.color,
            row_fill: tokens.bg,
            row_alt_fill: if striped {
                tokens.striped_bg
            } else {
                tokens.bg
            },
            row_hover_fill: tokens.hover_bg,
            row_active_fill: tokens.active_bg,
            row_text: tokens.color,
            border: tokens.border_color,
            border_width: 1.0,
            cell_padding: self.spacing.sm,
        }
    }
}

impl Default for UiTheme {
    fn default() -> Self {
        let palette_light = SemanticPalette::bootstrap(ThemeMode::Light);
        let palette_dark = SemanticPalette::bootstrap(ThemeMode::Dark);

        Self {
            breakpoints: Breakpoints::bootstrap(),
            spacing: SpacingScale::default(),
            radius: RadiusScale::default(),
            typography: TypographyScale::default(),
            mode: ThemeMode::Light,
            palette: palette_light,
            palette_light,
            palette_dark,
        }
    }
}

fn as_i8(value: f32) -> i8 {
    value.round().clamp(f32::from(i8::MIN), f32::from(i8::MAX)) as i8
}

fn rgba_with_opacity(color: egui::Color32, opacity: f32) -> egui::Color32 {
    let [r, g, b, _] = color.to_array();
    let a = (opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn with_opacity(color: egui::Color32, opacity: f32) -> egui::Color32 {
    let [r, g, b, a] = color.to_array();
    let alpha = (f32::from(a) * opacity.clamp(0.0, 1.0)).round() as u8;
    egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
}

fn mix(a: egui::Color32, b: egui::Color32, ratio: f32) -> egui::Color32 {
    let ratio = ratio.clamp(0.0, 1.0);
    let inv = 1.0 - ratio;

    let [ar, ag, ab, aa] = a.to_array();
    let [br, bg, bb, ba] = b.to_array();

    egui::Color32::from_rgba_unmultiplied(
        (f32::from(ar) * inv + f32::from(br) * ratio).round() as u8,
        (f32::from(ag) * inv + f32::from(bg) * ratio).round() as u8,
        (f32::from(ab) * inv + f32::from(bb) * ratio).round() as u8,
        (f32::from(aa) * inv + f32::from(ba) * ratio).round() as u8,
    )
}

fn darken(color: egui::Color32, ratio: f32) -> egui::Color32 {
    mix(color, egui::Color32::BLACK, ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_uses_bootstrap_breakpoints() {
        let theme = UiTheme::default();
        assert_eq!(theme.breakpoints, Breakpoints::bootstrap());
    }

    #[test]
    fn pill_badge_has_larger_corner_radius_than_standard_badge() {
        let theme = UiTheme::default();

        let rounded = theme.resolve_badge_style(Variant::Info, false);
        let pill = theme.resolve_badge_style(Variant::Info, true);

        assert!(pill.corner_radius > rounded.corner_radius);
    }

    #[test]
    fn label_size_maps_to_typography_scale() {
        let theme = UiTheme::default();

        let small = theme.resolve_label_style(Variant::Primary, TextSize::Small, false);
        let medium = theme.resolve_label_style(Variant::Primary, TextSize::Medium, false);
        let large = theme.resolve_label_style(Variant::Primary, TextSize::Large, false);

        assert!(small.text_size < medium.text_size);
        assert!(medium.text_size < large.text_size);
    }

    #[test]
    fn alert_and_nav_styles_reflect_component_state() {
        let theme = UiTheme::default();

        let alert_filled = theme.resolve_alert_style(Variant::Warning, false);
        let alert_outlined = theme.resolve_alert_style(Variant::Warning, true);
        assert_ne!(alert_filled.fill, alert_outlined.fill);
        assert_eq!(alert_outlined.fill, theme.palette.surface);

        let warning = theme.resolve_variant(Variant::Warning);
        assert_eq!(alert_filled.fill, warning.bg_subtle);
        assert_eq!(alert_filled.text, warning.fill);
        assert_eq!(alert_filled.border, warning.fill);
        assert_eq!(alert_outlined.text, warning.fill);
        assert_eq!(alert_outlined.border, warning.fill);

        let nav_default = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Default);
        let nav_active = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Active);
        let nav_disabled = theme.resolve_nav_item_style(Variant::Primary, NavItemState::Disabled);

        assert_ne!(nav_default.fill, nav_active.fill);
        assert_ne!(nav_active.text, nav_disabled.text);
    }

    #[test]
    fn overlay_form_and_table_styles_are_exposed() {
        let theme = UiTheme::default();

        let overlay = theme.resolve_overlay_style(Variant::Dark);
        assert_eq!(overlay.panel_fill, theme.palette.surface);

        let form_valid =
            theme.resolve_form_control_style(Variant::Primary, FormValidationState::Valid, false);
        let form_invalid =
            theme.resolve_form_control_style(Variant::Primary, FormValidationState::Invalid, false);
        assert_ne!(form_valid.validation_border, form_invalid.validation_border);

        let striped = theme.resolve_table_style(Variant::Info, true);
        let plain = theme.resolve_table_style(Variant::Info, false);
        assert_ne!(striped.row_alt_fill, plain.row_alt_fill);
    }

    #[test]
    fn mode_switch_updates_palette_and_component_styles() {
        let mut theme = UiTheme::default();
        let light_alert = theme.resolve_alert_style(Variant::Danger, false);

        theme.set_mode(ThemeMode::Dark);

        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(
            theme.palette.text_default,
            egui::Color32::from_rgb(222, 226, 230)
        );

        let dark_alert = theme.resolve_alert_style(Variant::Danger, false);
        assert_ne!(light_alert.fill, dark_alert.fill);
    }

    #[test]
    fn spinner_style_uses_bootstrap_spacing_for_label_gap() {
        let theme = UiTheme::default();
        let spinner = theme.resolve_spinner_style(Variant::Primary);

        assert_eq!(spinner.label_gap, theme.spacing.sm);
    }

    #[test]
    fn form_range_style_uses_bootstrap_spacing_for_label_gap() {
        let theme = UiTheme::default();
        let range =
            theme.resolve_form_range_style(Variant::Primary, FormValidationState::None, false);

        assert_eq!(range.label_gap, theme.spacing.sm);
    }

    #[test]
    fn spacing_and_typography_defaults_match_bootstrap_baseline() {
        let theme = UiTheme::default();

        assert_eq!(theme.spacing.xs, 4.0);
        assert_eq!(theme.spacing.sm, 8.0);
        assert_eq!(theme.spacing.md, 16.0);
        assert_eq!(theme.spacing.lg, 24.0);
        assert_eq!(theme.spacing.xl, 48.0);

        assert_eq!(theme.typography.sm, 14.0);
        assert_eq!(theme.typography.md, 16.0);
        assert_eq!(theme.typography.lg, 20.0);
    }
}
