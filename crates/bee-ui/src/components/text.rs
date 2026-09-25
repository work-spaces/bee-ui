use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct TextProps {
    pub text: Span,
    pub variant: Variant,
    pub muted: bool,
}

impl TextProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            muted: false,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
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
pub struct Text {
    pub text: Span,
    pub variant: Variant,
    pub muted: bool,
}

impl Text {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            muted: false,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn subtle(self) -> Self {
        self.muted(true)
    }

    pub fn props(self) -> TextProps {
        self.into()
    }
}

impl From<Text> for TextProps {
    fn from(value: Text) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            muted: value.muted,
        }
    }
}

impl From<TextProps> for Text {
    fn from(value: TextProps) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            muted: value.muted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_subtle_builder_sets_muted() {
        let text = Text::new("Lorem ipsum").subtle();
        assert_eq!(text.text, Span::from("Lorem ipsum"));
        assert!(text.muted);
    }

    #[test]
    fn text_accepts_string_input() {
        let text = Text::new(String::from("Dolor sit amet"));
        assert_eq!(text.text, Span::from("Dolor sit amet"));
    }
}
