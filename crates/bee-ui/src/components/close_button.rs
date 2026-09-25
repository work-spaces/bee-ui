use crate::core::Span;

#[derive(Clone, Debug)]
pub struct CloseButtonProps {
    pub label: Span,
    pub enabled: bool,
    pub hover_text: Option<String>,
    pub on_click: Option<fn(&egui::Context)>,
}

impl CloseButtonProps {
    pub fn new() -> Self {
        Self {
            label: "×".into(),
            enabled: true,
            hover_text: None,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = label.into();
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

impl Default for CloseButtonProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct CloseButton {
    pub label: Span,
    pub enabled: bool,
    pub hover_text: Option<String>,
    pub on_click: Option<fn(&egui::Context)>,
}

impl CloseButton {
    pub fn new() -> Self {
        Self {
            label: "×".into(),
            enabled: true,
            hover_text: None,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = label.into();
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

    pub fn props(self) -> CloseButtonProps {
        self.into()
    }
}

impl Default for CloseButton {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CloseButton> for CloseButtonProps {
    fn from(value: CloseButton) -> Self {
        Self {
            label: value.label,
            enabled: value.enabled,
            hover_text: value.hover_text,
            on_click: value.on_click,
        }
    }
}

impl From<CloseButtonProps> for CloseButton {
    fn from(value: CloseButtonProps) -> Self {
        Self {
            label: value.label,
            enabled: value.enabled,
            hover_text: value.hover_text,
            on_click: value.on_click,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_button_defaults_to_multiplication_sign_label() {
        let button = CloseButton::new();
        assert_eq!(button.label, Span::from("×"));
        assert!(button.enabled);
    }

    #[test]
    fn close_button_builder_sets_interaction_props() {
        fn noop(_: &egui::Context) {}

        let button = CloseButton::new()
            .hover_text("Dismiss")
            .on_click(noop)
            .disabled();

        assert_eq!(button.label, Span::from("×"));
        assert_eq!(button.hover_text.as_deref(), Some("Dismiss"));
        assert!(button.on_click.is_some());
        assert!(!button.enabled);
    }

    #[test]
    fn close_button_accepts_string_label_input() {
        let button = CloseButton::new().label(String::from("Close"));
        assert_eq!(button.label, Span::from("Close"));
    }
}
