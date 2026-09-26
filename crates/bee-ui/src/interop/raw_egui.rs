use crate::core::UiRoot;

impl UiRoot<'_> {
    /// Runs `f` with direct access to the current `egui::Ui` and returns its value.
    pub fn raw<R>(&mut self, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
        self.interop_raw(f)
    }

    /// Applies a temporary egui `Style` override while rendering a semantic subtree.
    pub fn with_style_override<R>(
        &mut self,
        style_override: impl FnOnce(&mut egui::Style),
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        self.interop_with_style_override(style_override, f)
    }

    /// Applies a temporary egui theme variant (`Dark` or `Light`) while rendering a subtree.
    pub fn with_theme_variant<R>(
        &mut self,
        theme: egui::Theme,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        self.interop_with_theme_variant(theme, f)
    }
}

pub fn with_raw_egui(ui_root: &mut UiRoot<'_>, f: impl FnOnce(&egui::Context)) {
    f(ui_root.context());
}
