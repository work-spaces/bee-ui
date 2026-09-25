use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Debug)]
pub struct FormRangeProps {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    /// Mirrors HTML `step="any"`: the value is continuous and never snapped to `step`.
    pub step_any: bool,
    pub show_value: bool,
    pub label: Option<Span>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    /// Total outer width (track + value display + label).
    pub width: Option<f32>,
    /// Stretch to the available width of the parent (takes precedence over `width`).
    pub fill_width: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormRangeProps {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            step: 1.0,
            step_any: false,
            show_value: true,
            label: None,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            fill_width: false,
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
        self.step_any = false;
        self
    }

    pub fn step_any(mut self) -> Self {
        self.step_any = true;
        self
    }

    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.fill_width = fill_width;
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

    /// Step to snap values to, or `None` when the range is continuous (`step_any`).
    pub fn resolved_step(&self) -> Option<f32> {
        (!self.step_any).then(|| self.normalized_step())
    }
}

#[derive(Clone, Debug)]
pub struct FormRange {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub step_any: bool,
    pub show_value: bool,
    pub label: Option<Span>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub fill_width: bool,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormRange {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            step: 1.0,
            step_any: false,
            show_value: true,
            label: None,
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            fill_width: false,
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
        self.step_any = false;
        self
    }

    /// Continuous range (HTML `step="any"`); values are never snapped.
    pub fn step_any(mut self) -> Self {
        self.step_any = true;
        self
    }

    /// Stretch the range to fill the available width of its parent.
    pub fn fill_width(mut self, fill_width: bool) -> Self {
        self.fill_width = fill_width;
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
            step_any: value.step_any,
            show_value: value.show_value,
            label: value.label,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            fill_width: value.fill_width,
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
            step_any: value.step_any,
            show_value: value.show_value,
            label: value.label,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            fill_width: value.fill_width,
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
    fn form_range_step_any_and_fill_width_builders() {
        let props = FormRange::new(0.0, 1.0).step_any().fill_width(true).props();
        assert!(props.step_any);
        assert!(props.fill_width);
        assert_eq!(props.resolved_step(), None);

        let stepped = FormRange::new(0.0, 1.0).step_any().step(0.25).props();
        assert_eq!(stepped.resolved_step(), Some(0.25));
    }

    #[test]
    fn form_range_accepts_string_label_input() {
        let range = FormRange::new(0.0, 10.0).label(String::from("Retries"));
        assert_eq!(range.label, Some(Span::from("Retries")));
    }
}
