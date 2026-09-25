#[path = "../../assets/icons.rs"]
mod icon_assets;

pub use icon_assets::Icon as SvgIcon;

use super::{style::Variant, theme::UiTheme};
use crate::theme::fonts::{self, FontFace};

pub const DEFAULT_ICON_SIZE: f32 = 16.0;
pub const DEFAULT_INLINE_ICON_GAP: f32 = 4.0;
pub const ICON_RENDER_SIZE_MULTIPLIER: f32 = 1.2;

#[derive(Clone, Debug, PartialEq)]
pub struct IconProps {
    pub icon: SvgIcon,
    pub size: f32,
    pub gap_before: f32,
    pub gap_after: f32,
}

impl IconProps {
    pub const fn new(icon: SvgIcon) -> Self {
        Self {
            icon,
            size: DEFAULT_ICON_SIZE,
            gap_before: 0.0,
            gap_after: 0.0,
        }
    }

    /// Convenience for icons that should appear before text.
    pub const fn leading(icon: SvgIcon) -> Self {
        Self::new(icon).gap_after(DEFAULT_INLINE_ICON_GAP)
    }

    /// Convenience for icons that should appear after text.
    pub const fn trailing(icon: SvgIcon) -> Self {
        Self::new(icon).gap_before(DEFAULT_INLINE_ICON_GAP)
    }

    /// Convenience for icons that should be visually separated on both sides.
    pub const fn between(icon: SvgIcon) -> Self {
        Self::new(icon).gap(DEFAULT_INLINE_ICON_GAP)
    }

    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub const fn gap_before(mut self, gap_before: f32) -> Self {
        self.gap_before = gap_before;
        self
    }

    pub const fn gap_after(mut self, gap_after: f32) -> Self {
        self.gap_after = gap_after;
        self
    }

    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap_before = gap;
        self.gap_after = gap;
        self
    }

    pub fn svg(&self) -> &'static str {
        self.icon.svg()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Icon {
    pub icon: SvgIcon,
    pub size: f32,
    pub gap_before: f32,
    pub gap_after: f32,
}

impl Icon {
    pub const fn new(icon: SvgIcon) -> Self {
        Self {
            icon,
            size: DEFAULT_ICON_SIZE,
            gap_before: 0.0,
            gap_after: 0.0,
        }
    }

    /// Convenience for icons that should appear before text.
    pub const fn leading(icon: SvgIcon) -> Self {
        Self::new(icon).gap_after(DEFAULT_INLINE_ICON_GAP)
    }

    /// Convenience for icons that should appear after text.
    pub const fn trailing(icon: SvgIcon) -> Self {
        Self::new(icon).gap_before(DEFAULT_INLINE_ICON_GAP)
    }

    /// Convenience for icons that should be visually separated on both sides.
    pub const fn between(icon: SvgIcon) -> Self {
        Self::new(icon).gap(DEFAULT_INLINE_ICON_GAP)
    }

    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub const fn gap_before(mut self, gap_before: f32) -> Self {
        self.gap_before = gap_before;
        self
    }

    pub const fn gap_after(mut self, gap_after: f32) -> Self {
        self.gap_after = gap_after;
        self
    }

    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap_before = gap;
        self.gap_after = gap;
        self
    }

    pub const fn props(self) -> IconProps {
        IconProps {
            icon: self.icon,
            size: self.size,
            gap_before: self.gap_before,
            gap_after: self.gap_after,
        }
    }

    pub fn svg(&self) -> &'static str {
        self.icon.svg()
    }
}

impl From<Icon> for IconProps {
    fn from(value: Icon) -> Self {
        Self {
            icon: value.icon,
            size: value.size,
            gap_before: value.gap_before,
            gap_after: value.gap_after,
        }
    }
}

impl From<IconProps> for Icon {
    fn from(value: IconProps) -> Self {
        Self {
            icon: value.icon,
            size: value.size,
            gap_before: value.gap_before,
            gap_after: value.gap_after,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpanFont {
    Proportional,
    Monospace,
    Embedded(FontFace),
}

impl SpanFont {
    fn resolve_family(&self, ctx: &egui::Context) -> Option<egui::FontFamily> {
        match self {
            Self::Proportional => Some(egui::FontFamily::Proportional),
            Self::Monospace => Some(egui::FontFamily::Monospace),
            Self::Embedded(face) => {
                if fonts::ensure_face_registered(ctx, *face) {
                    Some(fonts::font_family_for(*face))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpanTextStyle {
    pub color: Option<egui::Color32>,
    pub background: Option<egui::Color32>,
    pub size: Option<f32>,
    pub font: Option<SpanFont>,
    pub strong: bool,
    pub weak: bool,
    pub italics: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

impl SpanTextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn background(mut self, background: egui::Color32) -> Self {
        self.background = Some(background);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size.max(0.0));
        self
    }

    pub fn font(mut self, font: SpanFont) -> Self {
        self.font = Some(font);
        self
    }

    pub fn strong(mut self, strong: bool) -> Self {
        self.strong = strong;
        self
    }

    pub fn weak(mut self, weak: bool) -> Self {
        self.weak = weak;
        self
    }

    pub fn italics(mut self, italics: bool) -> Self {
        self.italics = italics;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    fn apply_to(&self, ctx: &egui::Context, mut text: egui::RichText) -> egui::RichText {
        if let Some(color) = self.color {
            text = text.color(color);
        }

        if let Some(background) = self.background {
            text = text.background_color(background);
        }

        if let Some(size) = self.size {
            text = text.size(size);
        }

        if let Some(font) = &self.font {
            if let Some(family) = font.resolve_family(ctx) {
                text = text.family(family);
            }
        }

        if self.strong {
            text = text.strong();
        }

        if self.weak {
            text = text.weak();
        }

        if self.italics {
            text = text.italics();
        }

        if self.underline {
            text = text.underline();
        }

        if self.strikethrough {
            text = text.strikethrough();
        }

        text
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpanText {
    pub text: String,
    pub style: SpanTextStyle,
}

impl SpanText {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: SpanTextStyle::default(),
        }
    }

    pub fn style(mut self, style: SpanTextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.style = self.style.color(color);
        self
    }

    pub fn background(mut self, background: egui::Color32) -> Self {
        self.style = self.style.background(background);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.style = self.style.size(size);
        self
    }

    pub fn font(mut self, font: SpanFont) -> Self {
        self.style = self.style.font(font);
        self
    }

    pub fn strong(mut self, strong: bool) -> Self {
        self.style = self.style.strong(strong);
        self
    }

    pub fn weak(mut self, weak: bool) -> Self {
        self.style = self.style.weak(weak);
        self
    }

    pub fn italics(mut self, italics: bool) -> Self {
        self.style = self.style.italics(italics);
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.style = self.style.underline(underline);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.style = self.style.strikethrough(strikethrough);
        self
    }

    pub fn rich_text(&self, ctx: &egui::Context) -> egui::RichText {
        self.style.apply_to(ctx, egui::RichText::new(&self.text))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpanIcon {
    pub icon: SvgIcon,
    pub size: f32,
    pub tint: Option<egui::Color32>,
    pub variant: Option<Variant>,
    pub gap_before: f32,
    pub gap_after: f32,
}

impl SpanIcon {
    pub const fn new(icon: SvgIcon) -> Self {
        Self {
            icon,
            size: DEFAULT_ICON_SIZE,
            tint: None,
            variant: None,
            gap_before: 0.0,
            gap_after: 0.0,
        }
    }

    pub const fn leading(icon: SvgIcon) -> Self {
        Self::new(icon).gap_after(DEFAULT_INLINE_ICON_GAP)
    }

    pub const fn trailing(icon: SvgIcon) -> Self {
        Self::new(icon).gap_before(DEFAULT_INLINE_ICON_GAP)
    }

    pub const fn between(icon: SvgIcon) -> Self {
        Self::new(icon).gap(DEFAULT_INLINE_ICON_GAP)
    }

    pub const fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub const fn tint(mut self, tint: egui::Color32) -> Self {
        self.tint = Some(tint);
        self
    }

    /// Tint this icon using a semantic [`Variant`] color from the active theme.
    ///
    /// Explicit [`.tint`](Self::tint) colors take precedence when both are set.
    pub const fn variant(mut self, variant: Variant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub const fn gap_before(mut self, gap_before: f32) -> Self {
        self.gap_before = gap_before;
        self
    }

    pub const fn gap_after(mut self, gap_after: f32) -> Self {
        self.gap_after = gap_after;
        self
    }

    pub const fn gap(mut self, gap: f32) -> Self {
        self.gap_before = gap;
        self.gap_after = gap;
        self
    }

    pub fn svg(&self) -> &'static str {
        self.icon.svg()
    }

    pub const fn rendered_size(&self) -> f32 {
        self.size * ICON_RENDER_SIZE_MULTIPLIER
    }

    pub fn uses_current_color(&self) -> bool {
        self.svg().contains("currentColor")
    }

    fn svg_markup_for_tint(&self) -> std::borrow::Cow<'static, str> {
        if self.uses_current_color() {
            std::borrow::Cow::Owned(self.svg().replace("currentColor", "white"))
        } else {
            std::borrow::Cow::Borrowed(self.svg())
        }
    }

    /// Resolve the effective icon tint.
    ///
    /// Precedence: explicit color tint, then variant tone text color, then `fallback`.
    pub fn resolve_tint(
        &self,
        theme: &UiTheme,
        fallback: Option<egui::Color32>,
    ) -> Option<egui::Color32> {
        self.tint
            .or_else(|| {
                self.variant
                    .map(|variant| theme.resolve_variant(variant).text)
            })
            .or(fallback)
    }

    /// Build an egui [`Image`] for this icon.
    ///
    /// Icons that use `currentColor` are rewritten to white so callers can tint them
    /// (including via button `image_tint_follows_text_color`).
    pub fn egui_image(&self, tint: Option<egui::Color32>) -> egui::Image<'static> {
        let rendered_size = self.rendered_size();
        let image_uri = if self.uses_current_color() {
            format!("bytes://runnerd-bs/icons/{:?}-current-color.svg", self.icon)
        } else {
            format!("bytes://runnerd-bs/icons/{:?}.svg", self.icon)
        };

        let mut image = match self.svg_markup_for_tint() {
            std::borrow::Cow::Borrowed(svg) => egui::Image::from_bytes(image_uri, svg.as_bytes()),
            std::borrow::Cow::Owned(svg) => egui::Image::from_bytes(image_uri, svg.into_bytes()),
        }
        .fit_to_exact_size(egui::vec2(rendered_size, rendered_size));

        if let Some(tint) = tint {
            image = image.tint(tint);
        }

        image
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        self.render_with_tint(ui, self.tint)
    }

    pub fn render_with_theme(&self, ui: &mut egui::Ui, theme: &UiTheme) {
        self.render_with_tint(ui, self.resolve_tint(theme, None))
    }

    pub fn render_with_tint(&self, ui: &mut egui::Ui, tint: Option<egui::Color32>) {
        let effective_tint =
            tint.or_else(|| self.uses_current_color().then(|| ui.visuals().text_color()));
        ui.add(self.egui_image(effective_tint));
    }
}

impl From<IconProps> for SpanIcon {
    fn from(value: IconProps) -> Self {
        Self {
            icon: value.icon,
            size: value.size,
            tint: None,
            variant: None,
            gap_before: value.gap_before,
            gap_after: value.gap_after,
        }
    }
}

impl From<Icon> for SpanIcon {
    fn from(value: Icon) -> Self {
        Self {
            icon: value.icon,
            size: value.size,
            tint: None,
            variant: None,
            gap_before: value.gap_before,
            gap_after: value.gap_after,
        }
    }
}

impl From<SvgIcon> for SpanIcon {
    fn from(value: SvgIcon) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpanRun {
    Text(SpanText),
    Icon(SpanIcon),
}

impl From<SpanText> for SpanRun {
    fn from(value: SpanText) -> Self {
        Self::Text(value)
    }
}

impl From<SpanIcon> for SpanRun {
    fn from(value: SpanIcon) -> Self {
        Self::Icon(value)
    }
}

impl From<IconProps> for SpanRun {
    fn from(value: IconProps) -> Self {
        Self::Icon(value.into())
    }
}

impl From<Icon> for SpanRun {
    fn from(value: Icon) -> Self {
        Self::Icon(value.into())
    }
}

impl From<&str> for SpanRun {
    fn from(value: &str) -> Self {
        Self::Text(SpanText::new(value))
    }
}

impl From<String> for SpanRun {
    fn from(value: String) -> Self {
        Self::Text(SpanText::new(value))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RichText {
    pub runs: Vec<SpanRun>,
    pub wrap: bool,
}

pub type Span = RichText;

impl RichText {
    pub fn new() -> Self {
        Self {
            runs: Vec::new(),
            wrap: true,
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Self::new().text(text)
    }

    pub fn run(mut self, run: impl Into<SpanRun>) -> Self {
        self.runs.push(run.into());
        self
    }

    pub fn text(self, text: impl Into<String>) -> Self {
        self.run(SpanText::new(text))
    }

    pub fn styled_text(self, text: impl Into<String>, style: SpanTextStyle) -> Self {
        self.run(SpanText::new(text).style(style))
    }

    pub fn icon(self, icon: impl Into<SpanIcon>) -> Self {
        self.run(icon.into())
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn no_wrap(self) -> Self {
        self.wrap(false)
    }

    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    pub fn plain_text(&self) -> String {
        let mut text = String::new();

        for run in &self.runs {
            if let SpanRun::Text(span_text) = run {
                text.push_str(&span_text.text);
            }
        }

        text
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        self.render_with_icons(ui, |icon_ui, icon| {
            icon.render(icon_ui);
        });
    }

    pub fn render_with_icons(
        &self,
        ui: &mut egui::Ui,
        mut render_icon: impl FnMut(&mut egui::Ui, &SpanIcon),
    ) {
        if self.wrap {
            ui.horizontal_wrapped(|row_ui| self.render_runs(row_ui, &mut render_icon));
        } else {
            ui.horizontal(|row_ui| self.render_runs(row_ui, &mut render_icon));
        }
    }

    fn render_runs(
        &self,
        ui: &mut egui::Ui,
        render_icon: &mut dyn FnMut(&mut egui::Ui, &SpanIcon),
    ) {
        ui.scope(|scope_ui| {
            scope_ui.spacing_mut().item_spacing.x = 0.0;

            for run in &self.runs {
                match run {
                    SpanRun::Text(text) => {
                        if text.text.is_empty() {
                            continue;
                        }

                        scope_ui.label(text.rich_text(scope_ui.ctx()));
                    }
                    SpanRun::Icon(icon) => {
                        if icon.gap_before > 0.0 {
                            scope_ui.add_space(icon.gap_before);
                        }

                        render_icon(scope_ui, icon);

                        if icon.gap_after > 0.0 {
                            scope_ui.add_space(icon.gap_after);
                        }
                    }
                }
            }
        });
    }
}

impl std::fmt::Display for RichText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.plain_text())
    }
}

impl From<RichText> for String {
    fn from(value: RichText) -> Self {
        value.plain_text()
    }
}

impl From<&RichText> for String {
    fn from(value: &RichText) -> Self {
        value.plain_text()
    }
}

impl Into<egui::WidgetText> for RichText {
    fn into(self) -> egui::WidgetText {
        self.plain_text().into()
    }
}

impl Into<egui::WidgetText> for &RichText {
    fn into(self) -> egui::WidgetText {
        self.plain_text().into()
    }
}

impl From<SpanText> for RichText {
    fn from(value: SpanText) -> Self {
        Self::new().run(value)
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::plain(value)
    }
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::plain(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_spacing_convenience_builders_set_expected_gaps() {
        let leading = Icon::leading(SvgIcon::ArrowRight);
        let trailing = Icon::trailing(SvgIcon::ArrowRight);
        let between = Icon::between(SvgIcon::ArrowRight);

        assert_eq!(leading.gap_before, 0.0);
        assert_eq!(leading.gap_after, DEFAULT_INLINE_ICON_GAP);

        assert_eq!(trailing.gap_before, DEFAULT_INLINE_ICON_GAP);
        assert_eq!(trailing.gap_after, 0.0);

        assert_eq!(between.gap_before, DEFAULT_INLINE_ICON_GAP);
        assert_eq!(between.gap_after, DEFAULT_INLINE_ICON_GAP);
    }

    #[test]
    fn icon_exposes_embedded_svg_markup() {
        let icon = Icon::new(SvgIcon::Check);
        let svg = icon.svg();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn rich_span_builder_preserves_text_and_icon_order() {
        let span = RichText::new()
            .text("Deploy ")
            .icon(SpanIcon::leading(SvgIcon::Check))
            .styled_text(
                "ready",
                SpanTextStyle::new()
                    .color(egui::Color32::LIGHT_GREEN)
                    .strong(true)
                    .font(SpanFont::Monospace),
            );

        assert_eq!(span.runs.len(), 3);
        assert!(matches!(span.runs.first(), Some(SpanRun::Text(_))));
        assert!(matches!(span.runs.get(1), Some(SpanRun::Icon(_))));
        assert!(matches!(span.runs.get(2), Some(SpanRun::Text(_))));
    }

    #[test]
    fn rich_text_from_str_and_string_preserve_plain_text() {
        let from_str: Span = "Cluster healthy".into();
        let from_string: Span = String::from("Node draining").into();

        assert_eq!(from_str.plain_text(), "Cluster healthy");
        assert_eq!(from_string.plain_text(), "Node draining");
    }

    #[test]
    fn span_run_from_str_and_string_create_text_runs() {
        let from_str: SpanRun = "status".into();
        let from_string: SpanRun = String::from("warning").into();

        assert!(matches!(from_str, SpanRun::Text(_)));
        assert!(matches!(from_string, SpanRun::Text(_)));
    }

    #[test]
    fn span_icon_conversion_from_icon_props_preserves_dimensions_and_gaps() {
        let icon = IconProps::between(SvgIcon::ArrowRight).size(20.0);
        let span_icon: SpanIcon = icon.into();

        assert_eq!(span_icon.size, 20.0);
        assert_eq!(span_icon.gap_before, DEFAULT_INLINE_ICON_GAP);
        assert_eq!(span_icon.gap_after, DEFAULT_INLINE_ICON_GAP);
    }

    #[test]
    fn span_icon_rendered_size_scales_proportionally() {
        let icon = SpanIcon::new(SvgIcon::Check).size(20.0);

        assert_eq!(icon.rendered_size(), 24.0);
    }

    #[test]
    fn span_icon_variant_resolves_theme_text_tone() {
        let theme = UiTheme::default();
        let icon = SpanIcon::new(SvgIcon::InfoCircle).variant(Variant::Warning);

        assert_eq!(icon.variant, Some(Variant::Warning));
        assert_eq!(
            icon.resolve_tint(&theme, None),
            Some(theme.resolve_variant(Variant::Warning).text)
        );
    }

    #[test]
    fn span_icon_explicit_tint_takes_precedence_over_variant() {
        let theme = UiTheme::default();
        let tint = egui::Color32::from_rgb(1, 2, 3);
        let icon = SpanIcon::new(SvgIcon::InfoCircle)
            .variant(Variant::Info)
            .tint(tint);

        assert_eq!(icon.resolve_tint(&theme, None), Some(tint));
    }

    #[test]
    fn span_icon_current_color_svg_is_normalized_for_tinting() {
        let icon = SpanIcon::new(SvgIcon::Check);
        let svg_markup = icon.svg_markup_for_tint();

        assert!(!svg_markup.contains("currentColor"));
        assert!(svg_markup.contains("fill=\"white\""));
    }

    #[test]
    fn rich_span_custom_icon_renderer_is_called_for_each_icon_run() {
        let ctx = egui::Context::default();
        let mut rendered_icons = 0;

        let rich = RichText::new()
            .text("Open")
            .icon(SpanIcon::trailing(SvgIcon::ArrowRight))
            .icon(SpanIcon::leading(SvgIcon::Check));

        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                rich.render_with_icons(ui, |_ui, _icon| {
                    rendered_icons += 1;
                });
            });
        });

        assert_eq!(rendered_icons, 2);
    }
}
