use crate::{components::forms::FormValidationState, core::Variant};

#[derive(Clone, Debug)]
pub struct FormControlProps {
    pub placeholder: Option<String>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub password: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormControlProps {
    pub fn new() -> Self {
        Self {
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

impl Default for FormControlProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct FormControl {
    pub placeholder: Option<String>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub password: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormControl {
    pub fn new() -> Self {
        Self {
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

    pub fn props(self) -> FormControlProps {
        self.into()
    }
}

impl Default for FormControl {
    fn default() -> Self {
        Self::new()
    }
}

impl From<FormControl> for FormControlProps {
    fn from(value: FormControl) -> Self {
        Self {
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

impl From<FormControlProps> for FormControl {
    fn from(value: FormControlProps) -> Self {
        Self {
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
    fn form_control_builders_set_password_and_validation() {
        fn noop(_: &egui::Context) {}

        let control = FormControl::new()
            .placeholder("Email")
            .variant(Variant::Info)
            .invalid()
            .width(320.0)
            .as_password()
            .on_change(noop);

        assert_eq!(control.placeholder.as_deref(), Some("Email"));
        assert_eq!(control.variant, Variant::Info);
        assert_eq!(control.validation, FormValidationState::Invalid);
        assert_eq!(control.width, Some(320.0));
        assert!(control.password);
        assert!(control.on_change.is_some());
    }
}
