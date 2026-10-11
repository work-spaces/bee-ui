use walkers::MapMemory;

use crate::{
    components::{Button, HStack},
    core::SvgIcon,
    show_in_ui,
    theme::bootstrap_tokens,
};

/// Controls zoom and centering for a map.
pub struct MapControlPanel;

impl MapControlPanel {
    /// Displays map controls and applies any actions to `memory`.
    pub fn show(ui: &mut egui::Ui, memory: &mut MapMemory) {
        let spacing = bootstrap_tokens::spacing();
        let row_size = egui::vec2(ui.available_width(), ui.spacing().interact_size.y);

        ui.allocate_ui_with_layout(
            row_size,
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                let mut clicked = None;
                show_in_ui(ui, |root| {
                    root.hstack(HStack::new().gap(spacing.md), |root| {
                        // Right-to-left layout places the first child at the far right, so add the
                        // buttons in reverse of their left-to-right display order.
                        for action in MapControlAction::DISPLAY_ORDER.into_iter().rev() {
                            if root.button(action.button()).clicked() {
                                clicked = Some(action);
                            }
                        }
                    });
                });
                if let Some(action) = clicked {
                    action.apply(memory);
                }
            },
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MapControlAction {
    ZoomOut,
    Recenter,
    ZoomIn,
}

impl MapControlAction {
    /// Left-to-right order in which the buttons appear.
    const DISPLAY_ORDER: [Self; 3] = [Self::ZoomOut, Self::Recenter, Self::ZoomIn];

    fn button(self) -> Button {
        match self {
            Self::ZoomOut => Button::secondary("")
                .icon(SvgIcon::ZoomOut)
                .hover_text("Zoom out"),
            Self::Recenter => Button::primary("")
                .icon(SvgIcon::Crosshair)
                .hover_text("Return to the selected location"),
            Self::ZoomIn => Button::secondary("")
                .icon(SvgIcon::ZoomIn)
                .hover_text("Zoom in"),
        }
    }

    fn apply(self, memory: &mut MapMemory) {
        match self {
            Self::ZoomOut => {
                let _ = memory.zoom_out();
            }
            Self::Recenter => memory.follow_my_position(),
            Self::ZoomIn => {
                let _ = memory.zoom_in();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Variant;

    #[test]
    fn buttons_are_ordered_zoom_out_recenter_zoom_in() {
        assert_eq!(
            MapControlAction::DISPLAY_ORDER,
            [
                MapControlAction::ZoomOut,
                MapControlAction::Recenter,
                MapControlAction::ZoomIn,
            ]
        );
    }

    #[test]
    fn buttons_are_icon_only_with_tooltips() {
        let expected = [
            (MapControlAction::ZoomOut, SvgIcon::ZoomOut, "Zoom out"),
            (
                MapControlAction::Recenter,
                SvgIcon::Crosshair,
                "Return to the selected location",
            ),
            (MapControlAction::ZoomIn, SvgIcon::ZoomIn, "Zoom in"),
        ];

        for (action, icon, hover_text) in expected {
            let button = action.button();
            assert!(button.text.is_empty(), "{action:?} should not show text");
            assert_eq!(button.icon, Some(icon));
            assert_eq!(button.hover_text.as_deref(), Some(hover_text));
        }
    }

    #[test]
    fn zoom_buttons_are_secondary_and_recenter_is_primary() {
        assert_eq!(
            MapControlAction::ZoomOut.button().variant,
            Variant::Secondary
        );
        assert_eq!(
            MapControlAction::Recenter.button().variant,
            Variant::Primary
        );
        assert_eq!(
            MapControlAction::ZoomIn.button().variant,
            Variant::Secondary
        );
    }

    #[test]
    fn zoom_actions_change_map_zoom() {
        let mut memory = MapMemory::default();
        let initial = memory.zoom();

        MapControlAction::ZoomIn.apply(&mut memory);
        assert!(memory.zoom() > initial);

        MapControlAction::ZoomOut.apply(&mut memory);
        assert_eq!(memory.zoom(), initial);
    }
}
