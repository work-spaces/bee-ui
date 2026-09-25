use crate::core::{Span, Variant};

#[derive(Clone, Debug)]
pub struct NavItem {
    pub text: Span,
    pub active: bool,
    pub disabled: bool,
    pub on_click: Option<fn(&egui::Context)>,
}

impl NavItem {
    pub fn new(text: impl Into<Span>) -> Self {
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

impl From<&str> for NavItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NavItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Span> for NavItem {
    fn from(value: Span) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct NavProps {
    pub items: Vec<NavItem>,
    pub variant: Variant,
    pub vertical: bool,
    pub fill: bool,
    pub justified: bool,
}

impl NavProps {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            variant: Variant::Primary,
            vertical: false,
            fill: false,
            justified: false,
        }
    }

    pub fn item(mut self, item: impl Into<NavItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<NavItem>) {
        self.items.push(item.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn stack(self) -> Self {
        self.vertical(true)
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn justified(mut self, justified: bool) -> Self {
        self.justified = justified;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for NavProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Nav {
    pub items: Vec<NavItem>,
    pub variant: Variant,
    pub vertical: bool,
    pub fill: bool,
    pub justified: bool,
}

impl Nav {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            variant: Variant::Primary,
            vertical: false,
            fill: false,
            justified: false,
        }
    }

    pub fn item(mut self, item: impl Into<NavItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<NavItem>) {
        self.items.push(item.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn stack(self) -> Self {
        self.vertical(true)
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn justified(mut self, justified: bool) -> Self {
        self.justified = justified;
        self
    }

    pub fn props(self) -> NavProps {
        self.into()
    }
}

impl Default for Nav {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Nav> for NavProps {
    fn from(value: Nav) -> Self {
        Self {
            items: value.items,
            variant: value.variant,
            vertical: value.vertical,
            fill: value.fill,
            justified: value.justified,
        }
    }
}

impl From<NavProps> for Nav {
    fn from(value: NavProps) -> Self {
        Self {
            items: value.items,
            variant: value.variant,
            vertical: value.vertical,
            fill: value.fill,
            justified: value.justified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_builder_collects_items_and_layout_hints() {
        let nav = Nav::new()
            .item(NavItem::new("Overview").current())
            .item(NavItem::new("Logs").disabled(true))
            .variant(Variant::Info)
            .fill(true)
            .justified(true);

        assert_eq!(nav.items.len(), 2);
        assert_eq!(nav.items[0].text, Span::from("Overview"));
        assert!(nav.items[0].active);
        assert_eq!(nav.items[1].text, Span::from("Logs"));
        assert!(nav.items[1].disabled);
        assert_eq!(nav.variant, Variant::Info);
        assert!(nav.fill);
        assert!(nav.justified);
    }

    #[test]
    fn nav_item_from_string_and_span_preserve_text() {
        let from_string: NavItem = String::from("Deployments").into();
        let from_span: NavItem = Span::from("Settings").into();

        assert_eq!(from_string.text, Span::from("Deployments"));
        assert_eq!(from_span.text, Span::from("Settings"));
    }
}
