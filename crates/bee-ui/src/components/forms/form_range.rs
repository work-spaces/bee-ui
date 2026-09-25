use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Debug)]
pub struct FormRangeProps {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub show_value: bool,
    pub label: Option<Span>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormRangeProps {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            step: 1.0,
            show_value: true,
            label: None,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            on_change: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = Some(label.into());
        self
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn on_change(mut self, on_change: fn(&egui::Context)) -> Self {
        self.on_change = Some(on_change);
        self
    }

    pub fn normalized_bounds(&self) -> (f32, f32) {
        if self.min <= self.max {
            (self.min, self.max)
        } else {
            (self.max, self.min)
        }
    }

    pub fn normalized_step(&self) -> f32 {
        if self.step.is_finite() && self.step > 0.0 {
            self.step
        } else {
            1.0
        }
    }
}

#[derive(Clone, Debug)]
pub struct FormRange {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub show_value: bool,
    pub label: Option<Span>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormRange {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            step: 1.0,
            show_value: true,
            label: None,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            on_change: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn show_value(mut self, show_value: bool) -> Self {
        self.show_value = show_value;
        self
    }

    pub fn label(mut self, label: impl Into<Span>) -> Self {
        self.label = Some(label.into());
        self
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn on_change(mut self, on_change: fn(&egui::Context)) -> Self {
        self.on_change = Some(on_change);
        self
    }

    pub fn props(self) -> FormRangeProps {
        self.into()
    }
}

impl From<FormRange> for FormRangeProps {
    fn from(value: FormRange) -> Self {
        Self {
            min: value.min,
            max: value.max,
            step: value.step,
            show_value: value.show_value,
            label: value.label,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            on_change: value.on_change,
        }
    }
}

impl From<FormRangeProps> for FormRange {
    fn from(value: FormRangeProps) -> Self {
        Self {
            min: value.min,
            max: value.max,
            step: value.step,
            show_value: value.show_value,
            label: value.label,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            on_change: value.on_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_range_normalizes_bounds_and_step() {
        let props = FormRangeProps::new(100.0, 0.0).step(0.0);

        assert_eq!(props.normalized_bounds(), (0.0, 100.0));
        assert_eq!(props.normalized_step(), 1.0);
    }

    #[test]
    fn form_range_builder_sets_validation_and_label() {
        let range = FormRange::new(0.0, 1.0)
            .step(0.05)
            .label("Threshold")
            .valid()
            .variant(Variant::Success);

        assert_eq!(range.step, 0.05);
        assert_eq!(range.label, Some(Span::from("Threshold")));
        assert_eq!(range.validation, FormValidationState::Valid);
        assert_eq!(range.variant, Variant::Success);
    }

    #[test]
    fn form_range_accepts_string_label_input() {
        let range = FormRange::new(0.0, 10.0).label(String::from("Retries"));
        assert_eq!(range.label, Some(Span::from("Retries")));
    }
}
