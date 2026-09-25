use crate::core::{Span, Variant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LabelProps {
    pub text: Span,
    pub variant: Variant,
    pub size: LabelSize,
    pub muted: bool,
}

impl LabelProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            size: LabelSize::default(),
            muted: false,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(self) -> Self {
        self.size(LabelSize::Small)
    }

    pub fn large(self) -> Self {
        self.size(LabelSize::Large)
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn subtle(self) -> Self {
        self.muted(true)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    pub text: Span,
    pub variant: Variant,
    pub size: LabelSize,
    pub muted: bool,
}

impl Label {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            size: LabelSize::default(),
            muted: false,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(self) -> Self {
        self.size(LabelSize::Small)
    }

    pub fn large(self) -> Self {
        self.size(LabelSize::Large)
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn subtle(self) -> Self {
        self.muted(true)
    }

    pub fn props(self) -> LabelProps {
        self.into()
    }
}

impl From<Label> for LabelProps {
    fn from(value: Label) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            size: value.size,
            muted: value.muted,
        }
    }
}

impl From<LabelProps> for Label {
    fn from(value: LabelProps) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            size: value.size,
            muted: value.muted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_convenience_builders_set_size_and_muting() {
        let label = Label::new("Latency").small().subtle();
        assert_eq!(label.text, Span::from("Latency"));
        assert_eq!(label.size, LabelSize::Small);
        assert!(label.muted);
    }

    #[test]
    fn label_accepts_string_text_input() {
        let label = Label::new(String::from("Errors"));
        assert_eq!(label.text, Span::from("Errors"));
    }
}
