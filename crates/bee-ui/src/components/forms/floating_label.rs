use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Debug)]
pub struct FloatingLabelProps {
    pub label: Span,
    pub placeholder: Option<String>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub password: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FloatingLabelProps {
    pub fn new(label: impl Into<Span>) -> Self {
        Self {
            label: label.into(),
            placeholder: None,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            password: false,
            on_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn validation(mut self, validation: FormValidationState) -> Self {
        self.validation = validation;
        self
    }

    pub fn valid(self) -> Self {
        self.validation(FormValidationState::Valid)
    }

    pub fn invalid(self) -> Self {
        self.validation(FormValidationState::Invalid)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    pub fn as_password(self) -> Self {
        self.password(true)
    }

    pub fn on_change(mut self, on_change: fn(&egui::Context)) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

#[derive(Clone, Debug)]
pub struct FloatingLabel {
    pub label: Span,
    pub placeholder: Option<String>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub password: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FloatingLabel {
    pub fn new(label: impl Into<Span>) -> Self {
        Self {
            label: label.into(),
            placeholder: None,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            password: false,
            on_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn validation(mut self, validation: FormValidationState) -> Self {
        self.validation = validation;
        self
    }

    pub fn valid(self) -> Self {
        self.validation(FormValidationState::Valid)
    }

    pub fn invalid(self) -> Self {
        self.validation(FormValidationState::Invalid)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    pub fn as_password(self) -> Self {
        self.password(true)
    }

    pub fn on_change(mut self, on_change: fn(&egui::Context)) -> Self {
        self.on_change = Some(on_change);
        self
    }

    pub fn props(self) -> FloatingLabelProps {
        self.into()
    }
}

impl From<FloatingLabel> for FloatingLabelProps {
    fn from(value: FloatingLabel) -> Self {
        Self {
            label: value.label,
            placeholder: value.placeholder,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            password: value.password,
            on_change: value.on_change,
        }
    }
}

impl From<FloatingLabelProps> for FloatingLabel {
    fn from(value: FloatingLabelProps) -> Self {
        Self {
            label: value.label,
            placeholder: value.placeholder,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            password: value.password,
            on_change: value.on_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floating_label_builder_sets_placeholder_and_validation() {
        let floating = FloatingLabel::new("Email")
            .placeholder("name@example.com")
            .invalid()
            .variant(Variant::Danger)
            .width(300.0);

        assert_eq!(floating.label, Span::from("Email"));
        assert_eq!(floating.placeholder.as_deref(), Some("name@example.com"));
        assert_eq!(floating.validation, FormValidationState::Invalid);
        assert_eq!(floating.variant, Variant::Danger);
        assert_eq!(floating.width, Some(300.0));
    }

    #[test]
    fn floating_label_accepts_string_label_input() {
        let floating = FloatingLabel::new(String::from("Password"));
        assert_eq!(floating.label, Span::from("Password"));
    }
}
