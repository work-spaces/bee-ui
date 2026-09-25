use crate::components::Button;

#[derive(Clone, Debug)]
pub struct ButtonGroupProps {
    pub buttons: Vec<Button>,
    pub vertical: bool,
}

impl ButtonGroupProps {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            vertical: false,
        }
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn push_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn stack(self) -> Self {
        self.vertical(true)
    }

    pub fn len(&self) -> usize {
        self.buttons.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty()
    }
}

impl Default for ButtonGroupProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct ButtonGroup {
    pub buttons: Vec<Button>,
    pub vertical: bool,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            vertical: false,
        }
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn push_button(&mut self, button: Button) {
        self.buttons.push(button);
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn stack(self) -> Self {
        self.vertical(true)
    }

    pub fn props(self) -> ButtonGroupProps {
        self.into()
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ButtonGroup> for ButtonGroupProps {
    fn from(value: ButtonGroup) -> Self {
        Self {
            buttons: value.buttons,
            vertical: value.vertical,
        }
    }
}

impl From<ButtonGroupProps> for ButtonGroup {
    fn from(value: ButtonGroupProps) -> Self {
        Self {
            buttons: value.buttons,
            vertical: value.vertical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_group_builder_collects_buttons() {
        let group = ButtonGroup::new()
            .button(Button::primary("One"))
            .button(Button::secondary("Two"))
            .stack();

        assert_eq!(group.buttons.len(), 2);
        assert!(group.vertical);
    }
}
