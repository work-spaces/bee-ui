use crate::core::{SvgIcon, Variant};

#[derive(Clone, Debug)]
pub struct Button {
    pub text: String,
    pub icon: Option<SvgIcon>,
    pub icon_after_text: bool,
    pub variant: Variant,
    pub enabled: bool,
    pub hover_text: Option<String>,
    pub on_click: Option<fn(&egui::Context)>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            icon_after_text: false,
            variant: Variant::Primary,
            enabled: true,
            hover_text: None,
            on_click: None,
        }
    }

    pub fn primary(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Primary)
    }

    pub fn secondary(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Secondary)
    }

    pub fn success(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Success)
    }

    pub fn danger(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Danger)
    }

    pub fn warning(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Warning)
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Info)
    }

    pub fn light(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Light)
    }

    pub fn dark(text: impl Into<String>) -> Self {
        Self::new(text).variant(Variant::Dark)
    }

    pub fn icon(mut self, icon: SvgIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_after_text(mut self, icon_after_text: bool) -> Self {
        self.icon_after_text = icon_after_text;
        self
    }

    pub fn icon_after(self) -> Self {
        self.icon_after_text(true)
    }

    pub fn icon_before(self) -> Self {
        self.icon_after_text(false)
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn disabled(self) -> Self {
        self.enabled(false)
    }

    pub fn hover_text(mut self, hover_text: impl Into<String>) -> Self {
        self.hover_text = Some(hover_text.into());
        self
    }

    pub fn on_click(mut self, on_click: fn(&egui::Context)) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_variant_and_enabled_builders_work() {
        let button = Button::warning("Restart").disabled();
        assert_eq!(button.text, "Restart");
        assert!(button.icon.is_none());
        assert!(!button.icon_after_text);
        assert_eq!(button.variant, Variant::Warning);
        assert!(!button.enabled);
        assert!(button.hover_text.is_none());
        assert!(button.on_click.is_none());
    }

    #[test]
    fn button_icon_builder_sets_icon_and_position() {
        let button = Button::new("Deploy").icon(SvgIcon::Flag).icon_after();

        assert_eq!(button.icon, Some(SvgIcon::Flag));
        assert!(button.icon_after_text);
    }

    #[test]
    fn button_interaction_builders_set_hover_and_click() {
        fn noop(_: &egui::Context) {}

        let button = Button::new("Theme")
            .hover_text("Switch demo theme")
            .on_click(noop);

        assert_eq!(button.text, "Theme");
        assert_eq!(button.hover_text.as_deref(), Some("Switch demo theme"));
        assert!(button.on_click.is_some());
    }

    #[test]
    fn button_accepts_string_text_input() {
        let button = Button::new(String::from("Rebalance"));
        assert_eq!(button.text, "Rebalance");
    }
}
