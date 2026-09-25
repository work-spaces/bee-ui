use crate::{
    components::nav::NavItem,
    core::{Span, Variant},
};

#[derive(Clone, Debug)]
pub struct NavbarProps {
    pub brand: Option<Span>,
    pub variant: Variant,
    pub items: Vec<NavItem>,
    pub on_brand_click: Option<fn(&egui::Context)>,
}

impl NavbarProps {
    pub fn new() -> Self {
        Self {
            brand: None,
            variant: Variant::Dark,
            items: Vec::new(),
            on_brand_click: None,
        }
    }

    pub fn brand(mut self, brand: impl Into<Span>) -> Self {
        self.brand = Some(brand.into());
        self
    }

    pub fn clear_brand(mut self) -> Self {
        self.brand = None;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn item(mut self, item: impl Into<NavItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<NavItem>) {
        self.items.push(item.into());
    }

    pub fn on_brand_click(mut self, on_brand_click: fn(&egui::Context)) -> Self {
        self.on_brand_click = Some(on_brand_click);
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.brand.is_none()
    }
}

impl Default for NavbarProps {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Navbar {
    pub brand: Option<Span>,
    pub variant: Variant,
    pub items: Vec<NavItem>,
    pub on_brand_click: Option<fn(&egui::Context)>,
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            brand: None,
            variant: Variant::Dark,
            items: Vec::new(),
            on_brand_click: None,
        }
    }

    pub fn brand(mut self, brand: impl Into<Span>) -> Self {
        self.brand = Some(brand.into());
        self
    }

    pub fn clear_brand(mut self) -> Self {
        self.brand = None;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn item(mut self, item: impl Into<NavItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn push_item(&mut self, item: impl Into<NavItem>) {
        self.items.push(item.into());
    }

    pub fn on_brand_click(mut self, on_brand_click: fn(&egui::Context)) -> Self {
        self.on_brand_click = Some(on_brand_click);
        self
    }

    pub fn props(self) -> NavbarProps {
        self.into()
    }
}

impl Default for Navbar {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Navbar> for NavbarProps {
    fn from(value: Navbar) -> Self {
        Self {
            brand: value.brand,
            variant: value.variant,
            items: value.items,
            on_brand_click: value.on_brand_click,
        }
    }
}

impl From<NavbarProps> for Navbar {
    fn from(value: NavbarProps) -> Self {
        Self {
            brand: value.brand,
            variant: value.variant,
            items: value.items,
            on_brand_click: value.on_brand_click,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navbar_builder_collects_brand_and_items() {
        fn noop(_: &egui::Context) {}

        let navbar = Navbar::new()
            .brand("runnerd")
            .item(NavItem::new("Dashboard").current())
            .item(NavItem::new("Settings"))
            .variant(Variant::Primary)
            .on_brand_click(noop);

        assert_eq!(navbar.brand, Some(Span::from("runnerd")));
        assert_eq!(navbar.items.len(), 2);
        assert_eq!(navbar.items[0].text, Span::from("Dashboard"));
        assert!(navbar.items[0].active);
        assert_eq!(navbar.items[1].text, Span::from("Settings"));
        assert_eq!(navbar.variant, Variant::Primary);
        assert!(navbar.on_brand_click.is_some());
    }

    #[test]
    fn navbar_accepts_string_brand_input() {
        let navbar = Navbar::new().brand(String::from("ops"));
        assert_eq!(navbar.brand, Some(Span::from("ops")));
    }
}
