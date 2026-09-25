use crate::core::Variant;

#[derive(Clone, Debug)]
pub struct DropdownItem {
    pub text: String,
    pub active: bool,
    pub disabled: bool,
    pub on_click: Option<fn(&egui::Context)>,
}

impl DropdownItem {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            active: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn current(self) -> Self {
        self.active(true)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, on_click: fn(&egui::Context)) -> Self {
        self.on_click = Some(on_click);
        self
    }
}

impl From<&str> for DropdownItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DropdownItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct DropdownProps {
    pub label: String,
    pub variant: Variant,
    pub items: Vec<DropdownItem>,
    pub align_end: bool,
    pub disabled: bool,
}

impl DropdownProps {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Secondary,
            items: Vec::new(),
            align_end: false,
            disabled: false,
        }
    }

    pub fn item(mut self, item: impl Into<DropdownItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<DropdownItem>) {
        self.items.push(item.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn align_end(mut self, align_end: bool) -> Self {
        self.align_end = align_end;
        self
    }

    pub fn align_right(self) -> Self {
        self.align_end(true)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Dropdown {
    pub label: String,
    pub variant: Variant,
    pub items: Vec<DropdownItem>,
    pub align_end: bool,
    pub disabled: bool,
}

impl Dropdown {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Secondary,
            items: Vec::new(),
            align_end: false,
            disabled: false,
        }
    }

    pub fn item(mut self, item: impl Into<DropdownItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<DropdownItem>) {
        self.items.push(item.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn align_end(mut self, align_end: bool) -> Self {
        self.align_end = align_end;
        self
    }

    pub fn align_right(self) -> Self {
        self.align_end(true)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn props(self) -> DropdownProps {
        self.into()
    }
}

impl From<Dropdown> for DropdownProps {
    fn from(value: Dropdown) -> Self {
        Self {
            label: value.label,
            variant: value.variant,
            items: value.items,
            align_end: value.align_end,
            disabled: value.disabled,
        }
    }
}

impl From<DropdownProps> for Dropdown {
    fn from(value: DropdownProps) -> Self {
        Self {
            label: value.label,
            variant: value.variant,
            items: value.items,
            align_end: value.align_end,
            disabled: value.disabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_builder_collects_items_and_alignment() {
        let dropdown = Dropdown::new("Actions")
            .item(DropdownItem::new("View details"))
            .item(DropdownItem::new("Delete").disabled(true))
            .variant(Variant::Dark)
            .align_right();

        assert_eq!(dropdown.label, "Actions");
        assert_eq!(dropdown.items.len(), 2);
        assert_eq!(dropdown.items[0].text, "View details");
        assert_eq!(dropdown.items[1].text, "Delete");
        assert!(dropdown.items[1].disabled);
        assert_eq!(dropdown.variant, Variant::Dark);
        assert!(dropdown.align_end);
    }

    #[test]
    fn dropdown_item_from_string_preserves_text() {
        let from_string: DropdownItem = String::from("Edit").into();

        assert_eq!(from_string.text, "Edit");
    }
}
