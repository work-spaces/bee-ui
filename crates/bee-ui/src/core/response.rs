//! Responses returned by interactive `UiRoot` methods.
//!
//! Single widgets (`button`, `close_button`) return the backend's `egui::Response` directly.
//! Collections and form inputs return the small `Copy` types defined here.

/// Result of drawing a collection of clickable items (`button_group`, `list_group`, `nav`,
/// `pagination`, `table`, `dropdown`).
///
/// At most one item can be clicked per pass; if the backend reports several, the first wins.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ItemClickResponse {
    clicked_index: Option<usize>,
}

impl ItemClickResponse {
    pub(crate) const fn new(clicked_index: Option<usize>) -> Self {
        Self { clicked_index }
    }

    pub(crate) fn from_clicked(clicked: &[usize]) -> Self {
        Self::new(clicked.first().copied())
    }

    /// Whether any item was clicked this pass.
    pub const fn clicked(&self) -> bool {
        self.clicked_index.is_some()
    }

    /// Index of the clicked item, if any.
    pub const fn clicked_index(&self) -> Option<usize> {
        self.clicked_index
    }

    /// Whether the item at `index` was clicked this pass.
    pub fn item_clicked(&self, index: usize) -> bool {
        self.clicked_index == Some(index)
    }
}

/// Result of drawing a `navbar`: item clicks plus brand clicks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NavbarResponse {
    items: ItemClickResponse,
    brand_clicked: bool,
}

impl NavbarResponse {
    pub(crate) const fn new(items: ItemClickResponse, brand_clicked: bool) -> Self {
        Self {
            items,
            brand_clicked,
        }
    }

    /// Whether any nav item was clicked this pass. Brand clicks are reported separately by
    /// [`Self::brand_clicked`].
    pub const fn clicked(&self) -> bool {
        self.items.clicked()
    }

    /// Index of the clicked nav item, if any.
    pub const fn clicked_index(&self) -> Option<usize> {
        self.items.clicked_index()
    }

    /// Whether the nav item at `index` was clicked this pass.
    pub fn item_clicked(&self, index: usize) -> bool {
        self.items.item_clicked(index)
    }

    /// Whether the brand was clicked this pass.
    pub const fn brand_clicked(&self) -> bool {
        self.brand_clicked
    }

    /// The nav item clicks on their own.
    pub const fn items(&self) -> ItemClickResponse {
        self.items
    }
}

/// Result of drawing a form input (`form_control`, `form_select`, `form_check`, `form_range`,
/// `floating_label`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChangeResponse {
    changed: bool,
}

impl ChangeResponse {
    pub(crate) const fn new(changed: bool) -> Self {
        Self { changed }
    }

    /// Whether the user changed the bound value this pass.
    pub const fn changed(&self) -> bool {
        self.changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_click_response_uses_the_first_clicked_index() {
        let response = ItemClickResponse::from_clicked(&[2, 0]);
        assert!(response.clicked());
        assert_eq!(response.clicked_index(), Some(2));
        assert!(response.item_clicked(2));
        assert!(!response.item_clicked(0));
    }

    #[test]
    fn empty_item_click_response_reports_nothing() {
        let response = ItemClickResponse::from_clicked(&[]);
        assert!(!response.clicked());
        assert_eq!(response.clicked_index(), None);
        assert_eq!(response, ItemClickResponse::default());
    }

    #[test]
    fn navbar_response_reports_brand_separately() {
        let response = NavbarResponse::new(ItemClickResponse::default(), true);
        assert!(response.brand_clicked());
        assert!(!response.clicked());

        let response = NavbarResponse::new(ItemClickResponse::new(Some(1)), false);
        assert!(!response.brand_clicked());
        assert!(response.item_clicked(1));
    }
}
