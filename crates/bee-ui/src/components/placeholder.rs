use crate::core::Variant;

#[derive(Clone, Debug, PartialEq)]
pub struct PlaceholderProps {
    pub variant: Variant,
    pub width: Option<f32>,
    pub height: f32,
    pub pulse: bool,
}

impl PlaceholderProps {
    pub fn new() -> Self {
        Self {
            variant: Variant::Secondary,
            width: None,
            height: 16.0,
            pulse: true,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn pulse(mut self, pulse: bool) -> Self {
        self.pulse = pulse;
        self
    }

    pub fn static_fill(self) -> Self {
        self.pulse(false)
    }
}

impl Default for PlaceholderProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Placeholder {
    pub variant: Variant,
    pub width: Option<f32>,
    pub height: f32,
    pub pulse: bool,
}

impl Placeholder {
    pub fn new() -> Self {
        Self {
            variant: Variant::Secondary,
            width: None,
            height: 16.0,
            pulse: true,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn pulse(mut self, pulse: bool) -> Self {
        self.pulse = pulse;
        self
    }

    pub fn static_fill(self) -> Self {
        self.pulse(false)
    }

    pub fn props(self) -> PlaceholderProps {
        self.into()
    }
}

impl Default for Placeholder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Placeholder> for PlaceholderProps {
    fn from(value: Placeholder) -> Self {
        Self {
            variant: value.variant,
            width: value.width,
            height: value.height,
            pulse: value.pulse,
        }
    }
}

impl From<PlaceholderProps> for Placeholder {
    fn from(value: PlaceholderProps) -> Self {
        Self {
            variant: value.variant,
            width: value.width,
            height: value.height,
            pulse: value.pulse,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_builder_sets_dimensions_and_pulse() {
        let placeholder = Placeholder::new()
            .variant(Variant::Light)
            .width(120.0)
            .height(18.0)
            .static_fill();

        assert_eq!(placeholder.variant, Variant::Light);
        assert_eq!(placeholder.width, Some(120.0));
        assert_eq!(placeholder.height, 18.0);
        assert!(!placeholder.pulse);
    }
}
