pub mod bootstrap_tokens;
pub mod dark;
pub mod fonts;
pub mod icons;
pub mod light;

const DEFAULT_THEME_STYLES_INSTALLED_ID: &str = "runnerd_bs.theme.default_styles_installed";
const DEFAULT_THEME_FONT_PROFILE_ID: &str = "runnerd_bs.theme.default_font_profile";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeFontProfile {
    /// Bootstrap-like default: keep `egui` proportional font mapping untouched.
    #[default]
    BootstrapSystemSans,
    /// Opt-in `runnerd` identity typography (0xProto proportional regular as the primary face).
    Runnerd0xProto,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeInstallOptions {
    pub font_profile: ThemeFontProfile,
}

/// Installs runnerd-bs defaults into an `egui::Context`.
///
/// Default behavior keeps `egui`'s proportional font mapping (Bootstrap-like baseline)
/// and does not force 0xProto globally.
pub fn install_default_theme(ctx: &egui::Context) {
    install_theme_with_options(ctx, ThemeInstallOptions::default());
}

/// Installs runnerd-bs styles and applies a caller-selected default font profile.
pub fn install_theme_with_options(ctx: &egui::Context, options: ThemeInstallOptions) {
    let should_install_styles = ctx.data_mut(|data| {
        let id = egui::Id::new(DEFAULT_THEME_STYLES_INSTALLED_ID);
        if data.get_persisted::<bool>(id).unwrap_or(false) {
            false
        } else {
            data.insert_persisted(id, true);
            true
        }
    });

    if should_install_styles {
        install_default_theme_styles(ctx);
    }

    apply_default_font_profile(ctx, options.font_profile);
}

/// Installs the 0xProto proportional mapping for cases where Bootstrap parity is not desired.
pub fn install_runnerd_0xproto_font_profile(ctx: &egui::Context) {
    apply_default_font_profile(ctx, ThemeFontProfile::Runnerd0xProto);
}

fn apply_default_font_profile(ctx: &egui::Context, profile: ThemeFontProfile) {
    let should_apply = ctx.data_mut(|data| {
        let id = egui::Id::new(DEFAULT_THEME_FONT_PROFILE_ID);
        let next = profile_storage_value(profile);
        let current = data.get_persisted::<u8>(id);

        if current == Some(next) {
            false
        } else {
            data.insert_persisted(id, next);
            true
        }
    });

    if !should_apply {
        return;
    }

    match profile {
        ThemeFontProfile::BootstrapSystemSans => uninstall_default_font_mapping(ctx),
        ThemeFontProfile::Runnerd0xProto => install_default_font_mapping(ctx),
    }
}

fn profile_storage_value(profile: ThemeFontProfile) -> u8 {
    match profile {
        ThemeFontProfile::BootstrapSystemSans => 0,
        ThemeFontProfile::Runnerd0xProto => 1,
    }
}

/// Installs the default runnerd-bs light/dark styles into an egui context.
pub fn install_default_theme_styles(ctx: &egui::Context) {
    ctx.set_style_of(egui::Theme::Light, light::style());
    ctx.set_style_of(egui::Theme::Dark, dark::style());
}

fn install_default_font_mapping(ctx: &egui::Context) {
    // Maximize laziness: only preload Proportional regular face.
    let face = fonts::FontFace::PropoRegular;
    let font_name = face.manifest_key().to_owned();

    let mut definitions = fonts::current_or_staged_font_definitions(ctx);
    definitions
        .font_data
        .entry(font_name.clone())
        .or_insert_with(|| fonts::load_face(face));

    let proportional = definitions
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    proportional.retain(|name| name != &font_name);
    proportional.insert(0, font_name);

    fonts::apply_staged_font_definitions(ctx, definitions);
}

fn uninstall_default_font_mapping(ctx: &egui::Context) {
    let face = fonts::FontFace::PropoRegular;
    let font_name = face.manifest_key();

    let mut definitions = fonts::current_or_staged_font_definitions(ctx);
    if let Some(proportional) = definitions
        .families
        .get_mut(&egui::FontFamily::Proportional)
    {
        proportional.retain(|name| name != font_name);
    }

    fonts::apply_staged_font_definitions(ctx, definitions);
}

/// Toggles the active egui theme between light and dark and returns the new theme.
pub fn toggle_theme(ctx: &egui::Context) -> egui::Theme {
    let next = match ctx.theme() {
        egui::Theme::Dark => egui::Theme::Light,
        egui::Theme::Light => egui::Theme::Dark,
    };
    ctx.set_theme(next);
    next
}
