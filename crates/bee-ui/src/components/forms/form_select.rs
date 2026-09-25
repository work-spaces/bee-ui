use crate::{
    components::forms::FormValidationState,
    core::{Span, Variant},
};

#[derive(Clone, Debug, PartialEq)]
pub struct FormSelectOption {
    pub text: Span,
    pub disabled: bool,
}

impl FormSelectOption {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl From<&str> for FormSelectOption {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for FormSelectOption {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Span> for FormSelectOption {
    fn from(value: Span) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct FormSelectProps {
    pub options: Vec<FormSelectOption>,
    pub placeholder: Option<Span>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormSelectProps {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            placeholder: Some("Select...".into()),
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            on_change: None,
        }
    }

    pub fn option(mut self, option: impl Into<FormSelectOption>) -> Self {
        self.options.push(option.into());
        self
    }

    pub fn push_option(&mut self, option: impl Into<FormSelectOption>) {
        self.options.push(option.into());
    }

    pub fn placeholder(mut self, placeholder: impl Into<Span>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn no_placeholder(mut self) -> Self {
        self.placeholder = None;
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

    pub fn len(&self) -> usize {
        self.options.len()
    }

    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    pub fn selected_text(&self, selected_index: usize) -> Option<&Span> {
        self.options.get(selected_index).map(|option| &option.text)
    }
}

impl Default for FormSelectProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct FormSelect {
    pub options: Vec<FormSelectOption>,
    pub placeholder: Option<Span>,
    pub variant: Variant,
    pub validation: FormValidationState,
    pub disabled: bool,
    pub width: Option<f32>,
    pub on_change: Option<fn(&egui::Context)>,
}

impl FormSelect {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            placeholder: Some("Select...".into()),
            variant: Variant::Primary,
            validation: FormValidationState::None,
            disabled: false,
            width: None,
            on_change: None,
        }
    }

    pub fn option(mut self, option: impl Into<FormSelectOption>) -> Self {
        self.options.push(option.into());
        self
    }

    pub fn push_option(&mut self, option: impl Into<FormSelectOption>) {
        self.options.push(option.into());
    }

    pub fn placeholder(mut self, placeholder: impl Into<Span>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn no_placeholder(mut self) -> Self {
        self.placeholder = None;
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

    pub fn props(self) -> FormSelectProps {
        self.into()
    }
}

impl Default for FormSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl From<FormSelect> for FormSelectProps {
    fn from(value: FormSelect) -> Self {
        Self {
            options: value.options,
            placeholder: value.placeholder,
            variant: value.variant,
            validation: value.validation,
            disabled: value.disabled,
            width: value.width,
            on_change: value.on_change,
        }
    }
}

impl From<FormSelectProps> for FormSelect {
    fn from(value: FormSelectProps) -> Self {
        Self {
            options: value.options,
            placeholder: value.placeholder,
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
    fn form_select_builder_collects_options_and_placeholder() {
        fn noop(_: &egui::Context) {}

        let select = FormSelect::new()
            .option("Primary")
            .option(FormSelectOption::new("Disabled").disabled(true))
            .placeholder("Choose one")
            .variant(Variant::Info)
            .valid()
            .on_change(noop);

        assert_eq!(select.options.len(), 2);
        assert_eq!(select.options[0].text, Span::from("Primary"));
        assert_eq!(select.options[1].text, Span::from("Disabled"));
        assert!(select.options[1].disabled);
        assert_eq!(select.placeholder, Some(Span::from("Choose one")));
        assert_eq!(select.variant, Variant::Info);
        assert_eq!(select.validation, FormValidationState::Valid);
        assert!(select.on_change.is_some());
    }

    #[test]
    fn form_select_option_from_string_and_span_preserve_text() {
        let from_string: FormSelectOption = String::from("Canary").into();
        let from_span: FormSelectOption = Span::from("Blue/Green").into();

        assert_eq!(from_string.text, Span::from("Canary"));
        assert_eq!(from_span.text, Span::from("Blue/Green"));
    }
}
