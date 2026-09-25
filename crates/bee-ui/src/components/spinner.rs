use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct SpinnerProps {
    pub variant: Variant,
    pub size: f32,
    pub label: Option<Span>,
}

impl SpinnerProps {
    pub fn new() -> Self {
        Self {
            variant: Variant::Primary,
            size: 32.0,
            label: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Default for SpinnerProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Spinner {
    pub variant: Variant,
    pub size: f32,
    pub label: Option<Span>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            variant: Variant::Primary,
            size: 32.0,
            label: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn props(self) -> SpinnerProps {
        self.into()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Spinner> for SpinnerProps {
    fn from(value: Spinner) -> Self {
        Self {
            variant: value.variant,
            size: value.size,
            label: value.label,
        }
    }
}

impl From<SpinnerProps> for Spinner {
    fn from(value: SpinnerProps) -> Self {
        Self {
            variant: value.variant,
            size: value.size,
            label: value.label,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_builders_set_size_and_label() {
        let spinner = Spinner::new()
            .variant(Variant::Info)
            .size(24.0)
            .label("Loading");

        assert_eq!(spinner.variant, Variant::Info);
        assert_eq!(spinner.size, 24.0);
        assert_eq!(spinner.label, Some(Span::from("Loading")));
    }

    #[test]
    fn spinner_accepts_string_label_input() {
        let spinner = Spinner::new().label(String::from("Waiting"));
        assert_eq!(spinner.label, Some(Span::from("Waiting")));
    }
}
