use walkers::MapMemory;

use crate::{
    components::{Button, HStack, Label},
    show_in_ui,
    theme::bootstrap_tokens,
};

/// Controls zoom and centering for a map.
pub struct MapControlPanel;

impl MapControlPanel {
    /// Displays map controls and applies any actions to `memory`.
    pub fn show(ui: &mut egui::Ui, memory: &mut MapMemory) {
        let spacing = bootstrap_tokens::spacing();

        show_in_ui(ui, |root| {
            root.hstack(HStack::new().gap(spacing.md), |root| {
                if root
                    .button(Button::secondary("−").hover_text("Zoom out"))
                    .clicked()
                {
                    let _ = memory.zoom_out();
                }
                if root
                    .button(Button::secondary("+").hover_text("Zoom in"))
                    .clicked()
                {
                    let _ = memory.zoom_in();
                }
                if root
                    .button(
                        Button::secondary("Recenter").hover_text("Return to the selected location"),
                    )
                    .clicked()
                {
                    memory.follow_my_position();
                }
                root.label(
                    Label::new(format!("Zoom {:.1}", memory.zoom()))
                        .small()
                        .subtle(),
                );
            });
        });
    }
}
