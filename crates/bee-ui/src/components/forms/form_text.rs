use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Debug, PartialEq)]
pub struct FormTextProps {
    pub text: Span,
    pub variant: Variant,
    pub validation: FormValidationState,
}

impl FormTextProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Secondary,
            validation: FormValidationState::None,
        }
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormText {
    pub text: Span,
    pub variant: Variant,
    pub validation: FormValidationState,
}

impl FormText {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Secondary,
            validation: FormValidationState::None,
        }
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

    pub fn props(self) -> FormTextProps {
        self.into()
    }
}

impl From<FormText> for FormTextProps {
    fn from(value: FormText) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            validation: value.validation,
        }
    }
}

impl From<FormTextProps> for FormText {
    fn from(value: FormTextProps) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            validation: value.validation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_text_builders_set_validation_state() {
        let text = FormText::new("Looks good")
            .variant(Variant::Success)
            .valid();

        assert_eq!(text.text, Span::from("Looks good"));
        assert_eq!(text.variant, Variant::Success);
        assert_eq!(text.validation, FormValidationState::Valid);
    }

    #[test]
    fn form_text_accepts_string_input() {
        let text = FormText::new(String::from("Replicas updated"));
        assert_eq!(text.text, Span::from("Replicas updated"));
    }
}
