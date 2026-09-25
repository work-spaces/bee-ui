use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct AlertProps {
    pub text: Span,
    pub variant: Variant,
    pub outlined: bool,
}

impl AlertProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            outlined: false,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub fn outline(self) -> Self {
        self.outlined(true)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Alert {
    pub text: Span,
    pub variant: Variant,
    pub outlined: bool,
}

impl Alert {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            outlined: false,
        }
    }

    pub fn primary(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Primary)
    }

    pub fn secondary(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Secondary)
    }

    pub fn success(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Success)
    }

    pub fn danger(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Danger)
    }

    pub fn warning(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Warning)
    }

    pub fn info(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Info)
    }

    pub fn light(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Light)
    }

    pub fn dark(text: impl Into<Span>) -> Self {
        Self::new(text).variant(Variant::Dark)
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn outlined(mut self, outlined: bool) -> Self {
        self.outlined = outlined;
        self
    }

    pub fn outline(self) -> Self {
        self.outlined(true)
    }

    pub fn props(self) -> AlertProps {
        self.into()
    }
}

impl From<Alert> for AlertProps {
    fn from(value: Alert) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            outlined: value.outlined,
        }
    }
}

impl From<AlertProps> for Alert {
    fn from(value: AlertProps) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            outlined: value.outlined,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_builders_set_variant_and_outline() {
        let alert = Alert::warning("Disk usage high").outline();
        assert_eq!(alert.text, Span::from("Disk usage high"));
        assert_eq!(alert.variant, Variant::Warning);
        assert!(alert.outlined);
    }

    #[test]
    fn alert_accepts_str_and_string_text_inputs() {
        let from_str = Alert::new("Node recovered");
        let from_string = Alert::new(String::from("Node degraded"));

        assert_eq!(from_str.text, Span::from("Node recovered"));
        assert_eq!(from_string.text, Span::from("Node degraded"));
    }
}
