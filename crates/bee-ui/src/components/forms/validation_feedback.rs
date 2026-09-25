use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationFeedbackProps {
    pub text: Span,
    pub state: FormValidationState,
    pub variant: Variant,
}

impl ValidationFeedbackProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            state: FormValidationState::None,
            variant: Variant::Primary,
        }
    }

    pub fn valid(text: impl Into<Span>) -> Self {
        Self::new(text).state(FormValidationState::Valid)
    }

    pub fn invalid(text: impl Into<Span>) -> Self {
        Self::new(text).state(FormValidationState::Invalid)
    }

    pub fn state(mut self, state: FormValidationState) -> Self {
        self.state = state;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationFeedback {
    pub text: Span,
    pub state: FormValidationState,
    pub variant: Variant,
}

impl ValidationFeedback {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            state: FormValidationState::None,
            variant: Variant::Primary,
        }
    }

    pub fn valid(text: impl Into<Span>) -> Self {
        Self::new(text).state(FormValidationState::Valid)
    }

    pub fn invalid(text: impl Into<Span>) -> Self {
        Self::new(text).state(FormValidationState::Invalid)
    }

    pub fn state(mut self, state: FormValidationState) -> Self {
        self.state = state;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn props(self) -> ValidationFeedbackProps {
        self.into()
    }
}

impl From<ValidationFeedback> for ValidationFeedbackProps {
    fn from(value: ValidationFeedback) -> Self {
        Self {
            text: value.text,
            state: value.state,
            variant: value.variant,
        }
    }
}

impl From<ValidationFeedbackProps> for ValidationFeedback {
    fn from(value: ValidationFeedbackProps) -> Self {
        Self {
            text: value.text,
            state: value.state,
            variant: value.variant,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_feedback_builder_sets_state() {
        let feedback = ValidationFeedback::invalid("Name is required").variant(Variant::Danger);

        assert_eq!(feedback.text, Span::from("Name is required"));
        assert_eq!(feedback.state, FormValidationState::Invalid);
        assert_eq!(feedback.variant, Variant::Danger);
    }

    #[test]
    fn validation_feedback_accepts_string_text_input() {
        let feedback = ValidationFeedback::new(String::from("All checks passed"));
        assert_eq!(feedback.text, Span::from("All checks passed"));
    }
}
