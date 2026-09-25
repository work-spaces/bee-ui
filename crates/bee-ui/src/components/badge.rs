use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct BadgeProps {
    pub text: Span,
    pub variant: Variant,
    pub pill: bool,
}

impl BadgeProps {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            pill: false,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }

    pub fn as_pill(self) -> Self {
        self.pill(true)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Badge {
    pub text: Span,
    pub variant: Variant,
    pub pill: bool,
}

impl Badge {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            pill: false,
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

    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }

    pub fn as_pill(self) -> Self {
        self.pill(true)
    }

    pub fn props(self) -> BadgeProps {
        self.into()
    }
}

impl From<Badge> for BadgeProps {
    fn from(value: Badge) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            pill: value.pill,
        }
    }
}

impl From<BadgeProps> for Badge {
    fn from(value: BadgeProps) -> Self {
        Self {
            text: value.text,
            variant: value.variant,
            pill: value.pill,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_variant_constructor_sets_variant() {
        let danger = Badge::danger("Down");
        let healthy = Badge::success("Healthy").as_pill();

        assert_eq!(danger.text, Span::from("Down"));
        assert_eq!(danger.variant, Variant::Danger);
        assert_eq!(healthy.text, Span::from("Healthy"));
        assert!(healthy.pill);
    }

    #[test]
    fn badge_accepts_str_and_string_text_inputs() {
        let from_str = Badge::new("Queued");
        let from_string = Badge::new(String::from("Running"));

        assert_eq!(from_str.text, Span::from("Queued"));
        assert_eq!(from_string.text, Span::from("Running"));
    }
}
