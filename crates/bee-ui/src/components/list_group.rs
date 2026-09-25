use crate::core::{Span, Variant};

#[derive(Clone, Debug)]
pub struct ListGroupItem {
    pub text: Span,
    pub variant: Variant,
    pub active: bool,
    pub disabled: bool,
    pub on_click: Option<fn(&egui::Context)>,
}

impl ListGroupItem {
    pub fn new(text: impl Into<Span>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::Primary,
            active: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
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

impl From<&str> for ListGroupItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ListGroupItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Span> for ListGroupItem {
    fn from(value: Span) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct ListGroupProps {
    pub items: Vec<ListGroupItem>,
    pub flush: bool,
}

impl ListGroupProps {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            flush: false,
        }
    }

    pub fn item(mut self, item: impl Into<ListGroupItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<ListGroupItem>) {
        self.items.push(item.into());
    }

    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for ListGroupProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct ListGroup {
    pub items: Vec<ListGroupItem>,
    pub flush: bool,
}

impl ListGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            flush: false,
        }
    }

    pub fn item(mut self, item: impl Into<ListGroupItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<ListGroupItem>) {
        self.items.push(item.into());
    }

    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }

    pub fn props(self) -> ListGroupProps {
        self.into()
    }
}

impl Default for ListGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ListGroup> for ListGroupProps {
    fn from(value: ListGroup) -> Self {
        Self {
            items: value.items,
            flush: value.flush,
        }
    }
}

impl From<ListGroupProps> for ListGroup {
    fn from(value: ListGroupProps) -> Self {
        Self {
            items: value.items,
            flush: value.flush,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_group_builder_collects_items() {
        let group = ListGroup::new()
            .item(ListGroupItem::new("Overview").current())
            .item(ListGroupItem::new("Jobs").variant(Variant::Info))
            .flush(true);

        assert_eq!(group.items.len(), 2);
        assert_eq!(group.items[0].text, Span::from("Overview"));
        assert!(group.items[0].active);
        assert_eq!(group.items[1].text, Span::from("Jobs"));
        assert_eq!(group.items[1].variant, Variant::Info);
        assert!(group.flush);
    }

    #[test]
    fn list_group_item_from_string_and_span_preserve_text() {
        let from_string: ListGroupItem = String::from("Alerts").into();
        let from_span: ListGroupItem = Span::from("Metrics").into();

        assert_eq!(from_string.text, Span::from("Alerts"));
        assert_eq!(from_span.text, Span::from("Metrics"));
    }
}
