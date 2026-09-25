#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorMode {
    #[default]
    Light,
    Dark,
}

impl ColorMode {
    pub fn from_dark_mode(dark_mode: bool) -> Self {
        if dark_mode { Self::Dark } else { Self::Light }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VariantTokens {
    pub fill: egui::Color32,
    pub on_fill: egui::Color32,
    pub text_emphasis: egui::Color32,
    pub bg_subtle: egui::Color32,
    pub border_subtle: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorTokens {
    pub body_color: egui::Color32,
    pub body_color_rgb: [u8; 3],
    pub body_bg: egui::Color32,
    pub body_bg_rgb: [u8; 3],
    pub emphasis_color: egui::Color32,
    pub emphasis_color_rgb: [u8; 3],
    pub secondary_color: egui::Color32,
    pub secondary_color_rgb: [u8; 3],
    pub tertiary_color: egui::Color32,
    pub tertiary_color_rgb: [u8; 3],
    pub secondary_bg: egui::Color32,
    pub secondary_bg_rgb: [u8; 3],
    pub tertiary_bg: egui::Color32,
    pub tertiary_bg_rgb: [u8; 3],
    pub border_color: egui::Color32,
    pub border_color_translucent: egui::Color32,
    pub link_color: egui::Color32,
    pub link_color_rgb: [u8; 3],
    pub link_hover_color: egui::Color32,
    pub link_hover_color_rgb: [u8; 3],
    pub focus_ring_color: egui::Color32,
    pub primary: VariantTokens,
    pub secondary: VariantTokens,
    pub success: VariantTokens,
    pub danger: VariantTokens,
    pub warning: VariantTokens,
    pub info: VariantTokens,
    pub light: VariantTokens,
    pub dark: VariantTokens,
    pub form_valid_color: egui::Color32,
    pub form_invalid_color: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpacingTokens {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypographyTokens {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub body_font_size: f32,
    pub body_font_weight: f32,
    pub body_line_height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadiusTokens {
    pub sm: u8,
    pub md: u8,
    pub lg: u8,
    pub xl: u8,
    pub xxl: u8,
    pub two_xl: u8,
    pub pill: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ControlTokens {
    pub button_padding_x: f32,
    pub button_padding_y: f32,
    pub control_height: f32,
    pub border_width: f32,
    pub focus_ring_width: f32,
    pub focus_ring_opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontTokens {
    pub sans_serif_stack: &'static str,
    pub monospace_stack: &'static str,
    pub body_font_family: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowToken {
    pub offset_x: i8,
    pub offset_y: i8,
    pub blur: u8,
    pub spread: u8,
    pub color: egui::Color32,
    pub inset: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShadowTokens {
    pub sm: ShadowToken,
    pub md: ShadowToken,
    pub lg: ShadowToken,
    pub inset: ShadowToken,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButtonComponentTokens {
    pub padding_x: f32,
    pub padding_y: f32,
    pub font_size: f32,
    pub font_weight: f32,
    pub border_radius: u8,
    pub disabled_opacity: f32,
    pub focus_ring_width: f32,
    pub focus_ring_opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlertComponentTokens {
    pub padding_x: f32,
    pub padding_y: f32,
    pub margin_bottom: f32,
    pub border_width: f32,
    pub border_radius: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BadgeComponentTokens {
    pub font_size: f32,
    pub font_weight: f32,
    pub padding_x: f32,
    pub padding_y: f32,
    pub border_radius: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CardComponentTokens {
    pub cap_bg: egui::Color32,
    pub spacer_x: u8,
    pub spacer_y: u8,
    pub cap_padding_x: u8,
    pub cap_padding_y: u8,
    pub border_color: egui::Color32,
    pub border_width: u8,
    pub border_radius: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropdownComponentTokens {
    pub menu_fill: egui::Color32,
    pub menu_border_color: egui::Color32,
    pub menu_border_width: u8,
    pub menu_border_radius: u8,
    pub menu_padding_x: u8,
    pub menu_padding_y: u8,
    pub divider_color: egui::Color32,
    pub item_color: egui::Color32,
    pub item_hover_color: egui::Color32,
    pub item_hover_bg: egui::Color32,
    pub item_active_color: egui::Color32,
    pub item_active_bg: egui::Color32,
    pub item_disabled_color: egui::Color32,
    pub item_padding_x: u8,
    pub item_padding_y: u8,
    pub shadow: ShadowToken,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavComponentTokens {
    pub link_padding_x: u8,
    pub link_padding_y: u8,
    pub link_color: egui::Color32,
    pub link_hover_color: egui::Color32,
    pub link_disabled_color: egui::Color32,
    pub link_active_color: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavbarComponentTokens {
    pub padding_x: u8,
    pub padding_y: u8,
    pub nav_link_padding_x: u8,
    pub active_color: egui::Color32,
    pub disabled_color: egui::Color32,
    pub brand_color: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PaginationComponentTokens {
    pub padding_x: f32,
    pub padding_y: f32,
    pub font_size: f32,
    pub border_width: f32,
    pub border_radius: u8,
    pub color: egui::Color32,
    pub bg: egui::Color32,
    pub border_color: egui::Color32,
    pub hover_color: egui::Color32,
    pub hover_bg: egui::Color32,
    pub hover_border_color: egui::Color32,
    pub focus_color: egui::Color32,
    pub focus_bg: egui::Color32,
    pub focus_ring_color: egui::Color32,
    pub focus_ring_width: f32,
    pub active_color: egui::Color32,
    pub active_bg: egui::Color32,
    pub active_border_color: egui::Color32,
    pub disabled_color: egui::Color32,
    pub disabled_bg: egui::Color32,
    pub disabled_border_color: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListGroupComponentTokens {
    pub item_padding_x: f32,
    pub item_padding_y: f32,
    pub border_width: f32,
    pub border_radius: u8,
    pub item_color: egui::Color32,
    pub item_bg: egui::Color32,
    pub item_border_color: egui::Color32,
    pub action_color: egui::Color32,
    pub action_hover_color: egui::Color32,
    pub action_hover_bg: egui::Color32,
    pub action_active_color: egui::Color32,
    pub action_active_bg: egui::Color32,
    pub disabled_color: egui::Color32,
    pub disabled_bg: egui::Color32,
    pub active_color: egui::Color32,
    pub active_bg: egui::Color32,
    pub active_border_color: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FormComponentTokens {
    pub input_padding_x: f32,
    pub input_padding_y: f32,
    pub input_font_size: f32,
    pub input_font_weight: f32,
    pub input_border_width: f32,
    pub input_border_radius: u8,
    pub input_color: egui::Color32,
    pub input_bg: egui::Color32,
    pub input_disabled_color: egui::Color32,
    pub input_disabled_bg: egui::Color32,
    pub input_border_color: egui::Color32,
    pub input_focus_border_color: egui::Color32,
    pub input_focus_width: f32,
    pub input_placeholder_color: egui::Color32,
    pub form_text_margin_top: f32,
    pub form_text_font_size: f32,
    pub form_text_color: egui::Color32,
    pub form_check_disabled_opacity: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableComponentTokens {
    pub color: egui::Color32,
    pub bg: egui::Color32,
    pub border_color: egui::Color32,
    pub striped_bg: egui::Color32,
    pub hover_bg: egui::Color32,
    pub active_bg: egui::Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressComponentTokens {
    pub height: f32,
    pub font_size: f32,
    pub bg: egui::Color32,
    pub border_radius: u8,
    pub bar_color: egui::Color32,
    pub bar_bg: egui::Color32,
    pub bar_transition_seconds: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpinnerComponentTokens {
    pub width: f32,
    pub height: f32,
    pub border_width: f32,
    pub animation_speed_seconds: f32,
}

pub fn colors(mode: ColorMode) -> ColorTokens {
    match mode {
        ColorMode::Light => light_colors(),
        ColorMode::Dark => dark_colors(),
    }
}

pub fn spacing() -> SpacingTokens {
    SpacingTokens {
        xs: 4.0,
        sm: 8.0,
        md: 16.0,
        lg: 24.0,
        xl: 48.0,
    }
}

pub fn typography() -> TypographyTokens {
    TypographyTokens {
        xs: 12.0,
        sm: 14.0,
        md: 16.0,
        lg: 20.0,
        body_font_size: 16.0,
        body_font_weight: 400.0,
        body_line_height: 1.5,
    }
}

pub fn radius() -> RadiusTokens {
    RadiusTokens {
        sm: 4,
        md: 6,
        lg: 8,
        xl: 16,
        xxl: 32,
        two_xl: 32,
        pill: 255,
    }
}

pub fn controls() -> ControlTokens {
    ControlTokens {
        button_padding_x: 12.0,
        button_padding_y: 6.0,
        control_height: 38.0,
        border_width: 1.0,
        focus_ring_width: 4.0,
        focus_ring_opacity: 0.25,
    }
}

pub fn fonts() -> FontTokens {
    FontTokens {
        sans_serif_stack: "system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", \"Noto Sans\", \"Liberation Sans\", Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\", \"Noto Color Emoji\"",
        monospace_stack: "SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"Courier New\", monospace",
        body_font_family: "var(--bs-font-sans-serif)",
    }
}

pub fn shadows() -> ShadowTokens {
    ShadowTokens {
        sm: ShadowToken {
            offset_x: 0,
            offset_y: 2,
            blur: 4,
            spread: 0,
            color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 19),
            inset: false,
        },
        md: ShadowToken {
            offset_x: 0,
            offset_y: 8,
            blur: 16,
            spread: 0,
            color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 38),
            inset: false,
        },
        lg: ShadowToken {
            offset_x: 0,
            offset_y: 16,
            blur: 48,
            spread: 0,
            color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 45),
            inset: false,
        },
        inset: ShadowToken {
            offset_x: 0,
            offset_y: 1,
            blur: 2,
            spread: 0,
            color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 19),
            inset: true,
        },
    }
}

pub fn button_component() -> ButtonComponentTokens {
    ButtonComponentTokens {
        padding_x: 12.0,
        padding_y: 6.0,
        font_size: 16.0,
        font_weight: 400.0,
        border_radius: radius().md,
        disabled_opacity: 0.65,
        focus_ring_width: 4.0,
        focus_ring_opacity: 0.5,
    }
}

pub fn alert_component() -> AlertComponentTokens {
    AlertComponentTokens {
        padding_x: 16.0,
        padding_y: 16.0,
        margin_bottom: 16.0,
        border_width: 1.0,
        border_radius: radius().md,
    }
}

pub fn badge_component() -> BadgeComponentTokens {
    BadgeComponentTokens {
        font_size: 12.0,
        font_weight: 700.0,
        padding_x: 7.8,
        padding_y: 4.2,
        border_radius: radius().md,
    }
}

pub fn card_component(mode: ColorMode) -> CardComponentTokens {
    let colors = colors(mode);
    CardComponentTokens {
        cap_bg: rgba(colors.body_color_rgb, 0.03),
        spacer_x: 16,
        spacer_y: 16,
        cap_padding_x: 16,
        cap_padding_y: 8,
        border_color: colors.border_color_translucent,
        border_width: 1,
        border_radius: radius().md,
    }
}

pub fn dropdown_component(mode: ColorMode) -> DropdownComponentTokens {
    let colors = colors(mode);
    let shadow = match mode {
        ColorMode::Light => shadows().md,
        ColorMode::Dark => ShadowToken {
            offset_x: 0,
            offset_y: 0,
            blur: 0,
            spread: 0,
            color: egui::Color32::TRANSPARENT,
            inset: false,
        },
    };

    DropdownComponentTokens {
        menu_fill: colors.body_bg,
        menu_border_color: colors.border_color_translucent,
        menu_border_width: 1,
        menu_border_radius: radius().md,
        menu_padding_x: 0,
        menu_padding_y: 8,
        divider_color: colors.border_color_translucent,
        item_color: colors.body_color,
        item_hover_color: colors.body_color,
        item_hover_bg: colors.tertiary_bg,
        item_active_color: egui::Color32::WHITE,
        item_active_bg: colors.primary.fill,
        item_disabled_color: colors.tertiary_color,
        item_padding_x: 16,
        item_padding_y: 4,
        shadow,
    }
}

pub fn nav_component(mode: ColorMode) -> NavComponentTokens {
    let colors = colors(mode);
    NavComponentTokens {
        link_padding_x: 16,
        link_padding_y: 8,
        link_color: colors.link_color,
        link_hover_color: colors.link_hover_color,
        link_disabled_color: colors.secondary_color,
        link_active_color: colors.emphasis_color,
    }
}

pub fn navbar_component(mode: ColorMode) -> NavbarComponentTokens {
    let colors = colors(mode);
    NavbarComponentTokens {
        padding_x: 0,
        padding_y: 8,
        nav_link_padding_x: 8,
        active_color: rgba(colors.emphasis_color_rgb, 1.0),
        disabled_color: rgba(colors.emphasis_color_rgb, 0.3),
        brand_color: rgba(colors.emphasis_color_rgb, 1.0),
    }
}

pub fn pagination_component(mode: ColorMode) -> PaginationComponentTokens {
    let colors = colors(mode);
    PaginationComponentTokens {
        padding_x: 12.0,
        padding_y: 6.0,
        font_size: 16.0,
        border_width: 1.0,
        border_radius: radius().md,
        color: colors.link_color,
        bg: colors.body_bg,
        border_color: colors.border_color,
        hover_color: colors.link_hover_color,
        hover_bg: colors.tertiary_bg,
        hover_border_color: colors.border_color,
        focus_color: colors.link_hover_color,
        focus_bg: colors.secondary_bg,
        focus_ring_color: colors.focus_ring_color,
        focus_ring_width: 4.0,
        active_color: egui::Color32::WHITE,
        active_bg: colors.primary.fill,
        active_border_color: colors.primary.fill,
        disabled_color: colors.secondary_color,
        disabled_bg: colors.secondary_bg,
        disabled_border_color: colors.border_color,
    }
}

pub fn list_group_component(mode: ColorMode) -> ListGroupComponentTokens {
    let colors = colors(mode);
    ListGroupComponentTokens {
        item_padding_x: 16.0,
        item_padding_y: 8.0,
        border_width: 1.0,
        border_radius: radius().md,
        item_color: colors.body_color,
        item_bg: colors.body_bg,
        item_border_color: colors.border_color,
        action_color: colors.secondary_color,
        action_hover_color: colors.emphasis_color,
        action_hover_bg: colors.tertiary_bg,
        action_active_color: colors.body_color,
        action_active_bg: colors.secondary_bg,
        disabled_color: colors.secondary_color,
        disabled_bg: colors.body_bg,
        active_color: egui::Color32::WHITE,
        active_bg: colors.primary.fill,
        active_border_color: colors.primary.fill,
    }
}

pub fn form_component(mode: ColorMode) -> FormComponentTokens {
    let colors = colors(mode);
    FormComponentTokens {
        input_padding_x: 12.0,
        input_padding_y: 6.0,
        input_font_size: 16.0,
        input_font_weight: 400.0,
        input_border_width: 1.0,
        input_border_radius: radius().md,
        input_color: colors.body_color,
        input_bg: colors.body_bg,
        input_disabled_color: colors.secondary_color,
        input_disabled_bg: colors.secondary_bg,
        input_border_color: colors.border_color,
        input_focus_border_color: egui::Color32::from_rgb(134, 183, 254),
        input_focus_width: 4.0,
        input_placeholder_color: colors.secondary_color,
        form_text_margin_top: 4.0,
        form_text_font_size: 14.0,
        form_text_color: colors.secondary_color,
        form_check_disabled_opacity: 0.5,
    }
}

pub fn table_component(mode: ColorMode) -> TableComponentTokens {
    let colors = colors(mode);
    TableComponentTokens {
        color: colors.emphasis_color,
        bg: colors.body_bg,
        border_color: colors.border_color,
        striped_bg: rgba(colors.emphasis_color_rgb, 0.05),
        hover_bg: rgba(colors.emphasis_color_rgb, 0.075),
        active_bg: rgba(colors.emphasis_color_rgb, 0.10),
    }
}

pub fn progress_component(mode: ColorMode) -> ProgressComponentTokens {
    let colors = colors(mode);
    ProgressComponentTokens {
        height: 16.0,
        font_size: 12.0,
        bg: colors.secondary_bg,
        border_radius: radius().md,
        bar_color: egui::Color32::WHITE,
        bar_bg: colors.primary.fill,
        bar_transition_seconds: 0.6,
    }
}

pub fn spinner_component() -> SpinnerComponentTokens {
    SpinnerComponentTokens {
        width: 32.0,
        height: 32.0,
        border_width: 4.0,
        animation_speed_seconds: 0.75,
    }
}

fn light_colors() -> ColorTokens {
    ColorTokens {
        body_color: egui::Color32::from_rgb(33, 37, 41),
        body_color_rgb: [33, 37, 41],
        body_bg: egui::Color32::WHITE,
        body_bg_rgb: [255, 255, 255],
        emphasis_color: egui::Color32::BLACK,
        emphasis_color_rgb: [0, 0, 0],
        secondary_color: egui::Color32::from_rgba_unmultiplied(33, 37, 41, 191),
        secondary_color_rgb: [33, 37, 41],
        tertiary_color: egui::Color32::from_rgba_unmultiplied(33, 37, 41, 128),
        tertiary_color_rgb: [33, 37, 41],
        secondary_bg: egui::Color32::from_rgb(233, 236, 239),
        secondary_bg_rgb: [233, 236, 239],
        tertiary_bg: egui::Color32::from_rgb(248, 249, 250),
        tertiary_bg_rgb: [248, 249, 250],
        border_color: egui::Color32::from_rgb(222, 226, 230),
        border_color_translucent: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 45),
        link_color: egui::Color32::from_rgb(13, 110, 253),
        link_color_rgb: [13, 110, 253],
        link_hover_color: egui::Color32::from_rgb(10, 88, 202),
        link_hover_color_rgb: [10, 88, 202],
        focus_ring_color: egui::Color32::from_rgba_unmultiplied(13, 110, 253, 64),
        primary: VariantTokens {
            fill: egui::Color32::from_rgb(13, 110, 253),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(5, 44, 101),
            bg_subtle: egui::Color32::from_rgb(207, 226, 255),
            border_subtle: egui::Color32::from_rgb(158, 197, 254),
        },
        secondary: VariantTokens {
            fill: egui::Color32::from_rgb(108, 117, 125),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(43, 47, 50),
            bg_subtle: egui::Color32::from_rgb(226, 227, 229),
            border_subtle: egui::Color32::from_rgb(196, 200, 203),
        },
        success: VariantTokens {
            fill: egui::Color32::from_rgb(25, 135, 84),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(10, 54, 34),
            bg_subtle: egui::Color32::from_rgb(209, 231, 221),
            border_subtle: egui::Color32::from_rgb(163, 207, 187),
        },
        danger: VariantTokens {
            fill: egui::Color32::from_rgb(220, 53, 69),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(88, 21, 28),
            bg_subtle: egui::Color32::from_rgb(248, 215, 218),
            border_subtle: egui::Color32::from_rgb(241, 174, 181),
        },
        warning: VariantTokens {
            fill: egui::Color32::from_rgb(255, 193, 7),
            on_fill: egui::Color32::BLACK,
            text_emphasis: egui::Color32::from_rgb(102, 77, 3),
            bg_subtle: egui::Color32::from_rgb(255, 243, 205),
            border_subtle: egui::Color32::from_rgb(255, 230, 156),
        },
        info: VariantTokens {
            fill: egui::Color32::from_rgb(13, 202, 240),
            on_fill: egui::Color32::BLACK,
            text_emphasis: egui::Color32::from_rgb(5, 81, 96),
            bg_subtle: egui::Color32::from_rgb(207, 244, 252),
            border_subtle: egui::Color32::from_rgb(158, 234, 249),
        },
        light: VariantTokens {
            fill: egui::Color32::from_rgb(248, 249, 250),
            on_fill: egui::Color32::BLACK,
            text_emphasis: egui::Color32::from_rgb(73, 80, 87),
            bg_subtle: egui::Color32::from_rgb(252, 252, 253),
            border_subtle: egui::Color32::from_rgb(233, 236, 239),
        },
        dark: VariantTokens {
            fill: egui::Color32::from_rgb(33, 37, 41),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(73, 80, 87),
            bg_subtle: egui::Color32::from_rgb(206, 212, 218),
            border_subtle: egui::Color32::from_rgb(173, 181, 189),
        },
        form_valid_color: egui::Color32::from_rgb(25, 135, 84),
        form_invalid_color: egui::Color32::from_rgb(220, 53, 69),
    }
}

fn dark_colors() -> ColorTokens {
    ColorTokens {
        body_color: egui::Color32::from_rgb(222, 226, 230),
        body_color_rgb: [222, 226, 230],
        body_bg: egui::Color32::from_rgb(33, 37, 41),
        body_bg_rgb: [33, 37, 41],
        emphasis_color: egui::Color32::WHITE,
        emphasis_color_rgb: [255, 255, 255],
        secondary_color: egui::Color32::from_rgba_unmultiplied(222, 226, 230, 191),
        secondary_color_rgb: [222, 226, 230],
        tertiary_color: egui::Color32::from_rgba_unmultiplied(222, 226, 230, 128),
        tertiary_color_rgb: [222, 226, 230],
        secondary_bg: egui::Color32::from_rgb(52, 58, 64),
        secondary_bg_rgb: [52, 58, 64],
        tertiary_bg: egui::Color32::from_rgb(43, 48, 53),
        tertiary_bg_rgb: [43, 48, 53],
        border_color: egui::Color32::from_rgb(73, 80, 87),
        border_color_translucent: egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38),
        link_color: egui::Color32::from_rgb(110, 168, 254),
        link_color_rgb: [110, 168, 254],
        link_hover_color: egui::Color32::from_rgb(139, 185, 254),
        link_hover_color_rgb: [139, 185, 254],
        focus_ring_color: egui::Color32::from_rgba_unmultiplied(13, 110, 253, 64),
        primary: VariantTokens {
            fill: egui::Color32::from_rgb(13, 110, 253),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(110, 168, 254),
            bg_subtle: egui::Color32::from_rgb(3, 22, 51),
            border_subtle: egui::Color32::from_rgb(8, 66, 152),
        },
        secondary: VariantTokens {
            fill: egui::Color32::from_rgb(108, 117, 125),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(167, 172, 177),
            bg_subtle: egui::Color32::from_rgb(22, 23, 25),
            border_subtle: egui::Color32::from_rgb(65, 70, 75),
        },
        success: VariantTokens {
            fill: egui::Color32::from_rgb(25, 135, 84),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(117, 183, 152),
            bg_subtle: egui::Color32::from_rgb(5, 27, 17),
            border_subtle: egui::Color32::from_rgb(15, 81, 50),
        },
        danger: VariantTokens {
            fill: egui::Color32::from_rgb(220, 53, 69),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(234, 134, 143),
            bg_subtle: egui::Color32::from_rgb(44, 11, 14),
            border_subtle: egui::Color32::from_rgb(132, 32, 41),
        },
        warning: VariantTokens {
            fill: egui::Color32::from_rgb(255, 193, 7),
            on_fill: egui::Color32::BLACK,
            text_emphasis: egui::Color32::from_rgb(255, 218, 106),
            bg_subtle: egui::Color32::from_rgb(51, 39, 1),
            border_subtle: egui::Color32::from_rgb(153, 116, 4),
        },
        info: VariantTokens {
            fill: egui::Color32::from_rgb(13, 202, 240),
            on_fill: egui::Color32::BLACK,
            text_emphasis: egui::Color32::from_rgb(110, 223, 246),
            bg_subtle: egui::Color32::from_rgb(3, 40, 48),
            border_subtle: egui::Color32::from_rgb(8, 121, 144),
        },
        light: VariantTokens {
            fill: egui::Color32::from_rgb(248, 249, 250),
            on_fill: egui::Color32::BLACK,
            text_emphasis: egui::Color32::from_rgb(248, 249, 250),
            bg_subtle: egui::Color32::from_rgb(52, 58, 64),
            border_subtle: egui::Color32::from_rgb(73, 80, 87),
        },
        dark: VariantTokens {
            fill: egui::Color32::from_rgb(33, 37, 41),
            on_fill: egui::Color32::WHITE,
            text_emphasis: egui::Color32::from_rgb(222, 226, 230),
            bg_subtle: egui::Color32::from_rgb(26, 29, 32),
            border_subtle: egui::Color32::from_rgb(52, 58, 64),
        },
        form_valid_color: egui::Color32::from_rgb(117, 183, 152),
        form_invalid_color: egui::Color32::from_rgb(234, 134, 143),
    }
}

fn rgba(rgb: [u8; 3], opacity: f32) -> egui::Color32 {
    let alpha = (opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
    egui::Color32::from_rgba_unmultiplied(rgb[0], rgb[1], rgb[2], alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_tokens_match_bootstrap_resolved_css_variables() {
        let light = colors(ColorMode::Light);

        assert_eq!(light.body_color, egui::Color32::from_rgb(33, 37, 41));
        assert_eq!(light.body_bg, egui::Color32::WHITE);
        assert_eq!(light.border_color, egui::Color32::from_rgb(222, 226, 230));
        assert_eq!(
            light.border_color_translucent,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 45)
        );
        assert_eq!(light.body_color_rgb, [33, 37, 41]);
        assert_eq!(light.body_bg_rgb, [255, 255, 255]);
        assert_eq!(
            light.secondary_color,
            egui::Color32::from_rgba_unmultiplied(33, 37, 41, 191)
        );
        assert_eq!(
            light.warning.text_emphasis,
            egui::Color32::from_rgb(102, 77, 3)
        );
        assert_eq!(
            light.danger.text_emphasis,
            egui::Color32::from_rgb(88, 21, 28)
        );
        assert_eq!(light.link_color, egui::Color32::from_rgb(13, 110, 253));
    }

    #[test]
    fn dark_tokens_match_bootstrap_resolved_css_variables() {
        let dark = colors(ColorMode::Dark);

        assert_eq!(dark.body_color, egui::Color32::from_rgb(222, 226, 230));
        assert_eq!(dark.body_bg, egui::Color32::from_rgb(33, 37, 41));
        assert_eq!(dark.border_color, egui::Color32::from_rgb(73, 80, 87));
        assert_eq!(
            dark.border_color_translucent,
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38)
        );
        assert_eq!(dark.body_color_rgb, [222, 226, 230]);
        assert_eq!(dark.body_bg_rgb, [33, 37, 41]);
        assert_eq!(
            dark.secondary_color,
            egui::Color32::from_rgba_unmultiplied(222, 226, 230, 191)
        );
        assert_eq!(dark.link_color, egui::Color32::from_rgb(110, 168, 254));
        assert_eq!(
            dark.danger.text_emphasis,
            egui::Color32::from_rgb(234, 134, 143)
        );
        assert_eq!(
            dark.warning.text_emphasis,
            egui::Color32::from_rgb(255, 218, 106)
        );
    }

    #[test]
    fn bootstrap_spacing_and_control_geometry_match_baseline() {
        let spacing = spacing();
        let controls = controls();
        let radius = radius();

        assert_eq!(spacing.xs, 4.0);
        assert_eq!(spacing.sm, 8.0);
        assert_eq!(spacing.md, 16.0);
        assert_eq!(spacing.lg, 24.0);
        assert_eq!(spacing.xl, 48.0);

        assert_eq!(controls.button_padding_x, 12.0);
        assert_eq!(controls.button_padding_y, 6.0);
        assert_eq!(controls.control_height, 38.0);
        assert_eq!(controls.focus_ring_width, 4.0);
        assert_eq!(controls.focus_ring_opacity, 0.25);

        assert_eq!(radius.sm, 4);
        assert_eq!(radius.md, 6);
        assert_eq!(radius.lg, 8);
        assert_eq!(radius.xl, 16);
        assert_eq!(radius.xxl, 32);
        assert_eq!(radius.two_xl, 32);
    }

    #[test]
    fn bootstrap_fonts_and_shadows_match_bootstrap_defaults() {
        let fonts = fonts();
        let shadows = shadows();

        assert!(fonts.sans_serif_stack.contains("system-ui"));
        assert!(fonts.sans_serif_stack.contains("Segoe UI"));
        assert!(fonts.monospace_stack.contains("SFMono-Regular"));
        assert_eq!(fonts.body_font_family, "var(--bs-font-sans-serif)");

        assert_eq!(shadows.md.offset_x, 0);
        assert_eq!(shadows.md.offset_y, 8);
        assert_eq!(shadows.md.blur, 16);
        assert_eq!(
            shadows.md.color,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 38)
        );
        assert!(shadows.inset.inset);
    }

    #[test]
    fn dropdown_shadow_matches_bootstrap_theme_defaults() {
        let light = dropdown_component(ColorMode::Light);
        let dark = dropdown_component(ColorMode::Dark);

        assert!(light.shadow.color != egui::Color32::TRANSPARENT);
        assert_eq!(dark.shadow.color, egui::Color32::TRANSPARENT);
    }

    #[test]
    fn phase3_component_token_defaults_match_bootstrap_values() {
        let button = button_component();
        let alert = alert_component();
        let badge = badge_component();
        let dropdown = dropdown_component(ColorMode::Light);
        let forms = form_component(ColorMode::Light);
        let pagination = pagination_component(ColorMode::Light);
        let spinner = spinner_component();

        assert_eq!(button.padding_x, 12.0);
        assert_eq!(button.padding_y, 6.0);
        assert_eq!(button.disabled_opacity, 0.65);

        assert_eq!(alert.padding_x, 16.0);
        assert_eq!(alert.margin_bottom, 16.0);

        assert_eq!(badge.font_size, 12.0);
        assert_eq!(badge.font_weight, 700.0);

        assert_eq!(dropdown.menu_padding_y, 8);
        assert_eq!(dropdown.item_padding_x, 16);

        assert_eq!(forms.input_padding_x, 12.0);
        assert_eq!(forms.input_focus_width, 4.0);
        assert_eq!(forms.form_text_font_size, 14.0);

        assert_eq!(pagination.padding_x, 12.0);
        assert_eq!(pagination.padding_y, 6.0);

        assert_eq!(spinner.width, 32.0);
        assert_eq!(spinner.border_width, 4.0);
        assert_eq!(spinner.animation_speed_seconds, 0.75);
    }
}
