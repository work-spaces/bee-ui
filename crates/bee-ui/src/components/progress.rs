use crate::core::{Span, Variant};

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressProps {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub variant: Variant,
    pub label: Option<Span>,
    pub show_percentage: bool,
}

impl ProgressProps {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            min: 0.0,
            max: 100.0,
            variant: Variant::Primary,
            label: None,
            show_percentage: false,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn show_percentage(mut self, show_percentage: bool) -> Self {
        self.show_percentage = show_percentage;
        self
    }

    pub fn with_percentage(self) -> Self {
        self.show_percentage(true)
    }

    pub fn fraction(&self) -> f32 {
        let range = self.max - self.min;
        if !range.is_finite() || range <= 0.0 {
            return 0.0;
        }

        let fraction = (self.value - self.min) / range;
        if fraction.is_finite() {
            fraction.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn percent_value(&self) -> f32 {
        self.fraction() * 100.0
    }
}

impl Default for ProgressProps {
    fn default() -> Self {
        Self::new(0.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Progress {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub variant: Variant,
    pub label: Option<Span>,
    pub show_percentage: bool,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            min: 0.0,
            max: 100.0,
            variant: Variant::Primary,
            label: None,
            show_percentage: false,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn show_percentage(mut self, show_percentage: bool) -> Self {
        self.show_percentage = show_percentage;
        self
    }

    pub fn with_percentage(self) -> Self {
        self.show_percentage(true)
    }

    pub fn props(self) -> ProgressProps {
        self.into()
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl From<Progress> for ProgressProps {
    fn from(value: Progress) -> Self {
        Self {
            value: value.value,
            min: value.min,
            max: value.max,
            variant: value.variant,
            label: value.label,
            show_percentage: value.show_percentage,
        }
    }
}

impl From<ProgressProps> for Progress {
    fn from(value: ProgressProps) -> Self {
        Self {
            value: value.value,
            min: value.min,
            max: value.max,
            variant: value.variant,
            label: value.label,
            show_percentage: value.show_percentage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_fraction_clamps_to_zero_and_one() {
        let low = ProgressProps::new(-10.0);
        let high = ProgressProps::new(150.0);

        assert_eq!(low.fraction(), 0.0);
        assert_eq!(high.fraction(), 1.0);
    }

    #[test]
    fn progress_fraction_handles_invalid_ranges() {
        let props = ProgressProps::new(10.0).range(5.0, 5.0);
        assert_eq!(props.fraction(), 0.0);
    }

    #[test]
    fn progress_builders_set_label_and_percentage() {
        let progress = Progress::new(42.0)
            .variant(Variant::Info)
            .label("42 MB")
            .with_percentage();

        assert_eq!(progress.variant, Variant::Info);
        assert_eq!(progress.label, Some(Span::from("42 MB")));
        assert!(progress.show_percentage);
    }

    #[test]
    fn progress_accepts_string_label_input() {
        let progress = Progress::new(72.0).label(String::from("72%"));
        assert_eq!(progress.label, Some(Span::from("72%")));
    }
}
