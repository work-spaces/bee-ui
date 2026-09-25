use crate::core::{Span, Variant};

#[derive(Clone, Debug)]
pub struct PageItem {
    pub text: Span,
    pub active: bool,
    pub disabled: bool,
    pub on_click: Option<fn(&egui::Context)>,
}

impl PageItem {
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

impl From<&str> for PageItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for PageItem {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<Span> for PageItem {
    fn from(value: Span) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Debug)]
pub struct PaginationProps {
    pub items: Vec<PageItem>,
    pub variant: Variant,
}

impl PaginationProps {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            variant: Variant::Primary,
        }
    }

    pub fn item(mut self, item: impl Into<PageItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<PageItem>) {
        self.items.push(item.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for PaginationProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Pagination {
    pub items: Vec<PageItem>,
    pub variant: Variant,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            variant: Variant::Primary,
        }
    }

    pub fn item(mut self, item: impl Into<PageItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<PageItem>) {
        self.items.push(item.into());
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn props(self) -> PaginationProps {
        self.into()
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Pagination> for PaginationProps {
    fn from(value: Pagination) -> Self {
        Self {
            items: value.items,
            variant: value.variant,
        }
    }
}

impl From<PaginationProps> for Pagination {
    fn from(value: PaginationProps) -> Self {
        Self {
            items: value.items,
            variant: value.variant,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pagination_builder_collects_items_and_active_state() {
        let pagination = Pagination::new()
            .item(PageItem::new("Previous").disabled(true))
            .item(PageItem::new("1").current())
            .item("2")
            .variant(Variant::Info);

        assert_eq!(pagination.items.len(), 3);
        assert_eq!(pagination.items[0].text, Span::from("Previous"));
        assert!(pagination.items[0].disabled);
        assert_eq!(pagination.items[1].text, Span::from("1"));
        assert!(pagination.items[1].active);
        assert_eq!(pagination.items[2].text, Span::from("2"));
        assert_eq!(pagination.variant, Variant::Info);
    }

    #[test]
    fn page_item_from_string_and_span_preserve_text() {
        let from_string: PageItem = String::from("Next").into();
        let from_span: PageItem = Span::from("Last").into();

        assert_eq!(from_string.text, Span::from("Next"));
        assert_eq!(from_span.text, Span::from("Last"));
    }
}
