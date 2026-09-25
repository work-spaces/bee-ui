use crate::core::UiRoot;

impl<'a> UiRoot<'a> {
    /// Appends a raw egui block to the semantic tree.
    pub fn raw(&mut self, f: impl FnOnce(&mut egui::Ui) + 'a) {
        self.interop_raw(f);
    }

    /// Applies a temporary egui `Style` override while rendering a semantic subtree.
    pub fn with_style_override(
        &mut self,
        style_override: impl FnOnce(&mut egui::Style) + 'a,
        f: impl FnOnce(&mut UiRoot<'a>),
    ) {
        self.interop_with_style_override(style_override, f);
    }

    /// Applies a temporary egui theme variant (`Dark` or `Light`) while rendering a subtree.
    pub fn with_theme_variant(&mut self, theme: egui::Theme, f: impl FnOnce(&mut UiRoot<'a>)) {
        self.interop_with_theme_variant(theme, f);
    }
}

pub fn with_raw_egui(ui_root: &mut UiRoot<'_>, f: impl FnOnce(&egui::Context)) {
    f(ui_root.context());
}
