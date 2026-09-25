use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct FormLabelProps {
    pub text: Span,
    pub variant: Variant,
    pub required: bool,
    pub muted: bool,
    pub hover_text: Option<String>,
}

impl FormLabelProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            required: false,
            muted: false,
            hover_text: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn is_required(self) -> Self {
        self.required(true)
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn hover_text(mut self, hover_text: impl Into<String>) -> Self {
        self.hover_text = Some(hover_text.into());
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormLabel {
    pub text: Span,
    pub variant: Variant,
    pub required: bool,
    pub muted: bool,
    pub hover_text: Option<String>,
}

impl FormLabel {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            required: false,
            muted: false,
            hover_text: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn is_required(self) -> Self {
        self.required(true)
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn hover_text(mut self, hover_text: impl Into<String>) -> Self {
        self.hover_text = Some(hover_text.into());
        self
    }

    pub fn props(self) -> FormLabelProps {
        self.into()
    }
}

impl From<FormLabel> for FormLabelProps {
    fn from(value: FormLabel) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            required: value.required,
            muted: value.muted,
            hover_text: value.hover_text,
        }
    }
}

impl From<FormLabelProps> for FormLabel {
    fn from(value: FormLabelProps) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            required: value.required,
            muted: value.muted,
            hover_text: value.hover_text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_label_builders_set_required_and_muted() {
        let label = FormLabel::new("Email")
            .variant(Variant::Secondary)
            .is_required()
            .muted(true);

        assert_eq!(label.text, Span::from("Email"));
        assert_eq!(label.variant, Variant::Secondary);
        assert!(label.required);
        assert!(label.muted);
    }

    #[test]
    fn form_label_accepts_string_input() {
        let label = FormLabel::new(String::from("Cluster name"));
        assert_eq!(label.text, Span::from("Cluster name"));
    }
}
