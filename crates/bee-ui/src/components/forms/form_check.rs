use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormCheckKind {
    #[default]
    Checkbox,
    Radio,
}

#[derive(Clone, Debug)]
pub struct FormCheckProps {
    pub label: Span,
    pub kind: FormCheckKind,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormCheckProps {
    pub fn new(label: impl Into<Span>) -> Self {
        Self {
            label: label.into(),
            kind: FormCheckKind::Checkbox,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            on_change: None,
        }
    }

    pub fn checkbox(label: impl Into<Span>) -> Self {
        Self::new(label).kind(FormCheckKind::Checkbox)
    }

    pub fn radio(label: impl Into<Span>) -> Self {
        Self::new(label).kind(FormCheckKind::Radio)
    }

    pub fn kind(mut self, kind: FormCheckKind) -> Self {
        self.kind = kind;
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

    pub fn on_change(mut self, on_change: fn(&egui::Context)) -> Self {
        self.on_change = Some(on_change);
        self
    }
}

#[derive(Clone, Debug)]
pub struct FormCheck {
    pub label: Span,
    pub kind: FormCheckKind,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormCheck {
    pub fn new(label: impl Into<Span>) -> Self {
        Self {
            label: label.into(),
            kind: FormCheckKind::Checkbox,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            on_change: None,
        }
    }

    pub fn checkbox(label: impl Into<Span>) -> Self {
        Self::new(label).kind(FormCheckKind::Checkbox)
    }

    pub fn radio(label: impl Into<Span>) -> Self {
        Self::new(label).kind(FormCheckKind::Radio)
    }

    pub fn kind(mut self, kind: FormCheckKind) -> Self {
        self.kind = kind;
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

    pub fn on_change(mut self, on_change: fn(&egui::Context)) -> Self {
        self.on_change = Some(on_change);
        self
    }

    pub fn props(self) -> FormCheckProps {
        self.into()
    }
}

impl From<FormCheck> for FormCheckProps {
    fn from(value: FormCheck) -> Self {
        Self {
            label: value.label,
            kind: value.kind,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            on_change: value.on_change,
        }
    }
}

impl From<FormCheckProps> for FormCheck {
    fn from(value: FormCheckProps) -> Self {
        Self {
            label: value.label,
            kind: value.kind,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            on_change: value.on_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_check_builder_switches_to_radio_and_validation() {
        let check = FormCheck::radio("Primary node")
            .variant(Variant::Info)
            .invalid()
            .disabled(true);

        assert_eq!(check.label, Span::from("Primary node"));
        assert_eq!(check.kind, FormCheckKind::Radio);
        assert_eq!(check.variant, Variant::Info);
        assert_eq!(check.validation, FormValidationState::Invalid);
        assert!(check.disabled);
    }

    #[test]
    fn form_check_accepts_string_label_input() {
        let check = FormCheck::new(String::from("Enable cache"));
        assert_eq!(check.label, Span::from("Enable cache"));
    }
}
