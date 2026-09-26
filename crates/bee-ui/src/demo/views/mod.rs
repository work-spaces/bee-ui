use crate::prelude::*;

use super::{DemoView, state, theme_badge_text};

mod alerts;
mod badges;
mod buttons;
mod cards;
mod dropdowns;
mod form;
mod grid_row_col;
mod groups;
mod labels;
mod overview;
mod plots;
mod progress_spinners;
mod shared;
mod tables;
mod typography_fonts_icons;

pub(super) fn render_demo_view(
    ui: &mut UiRoot<'_>,
    active_view: DemoView,
    demo_state: &mut state::DemoState,
) {
    ui.card(
        Card::new()
            .title(active_view.title())
            .footer(active_view.subtitle()),
        |ui| {
            ui.row(Row::new().gutter_x(12.0), |ui| {
                ui.col(Col::new(), |ui| {
                    ui.badge(Badge::secondary(active_view.category()).as_pill());
                });
                ui.col(Col::new(), |ui| {
                    ui.badge(Badge::info(theme_badge_text(ui.context().theme())).as_pill());
                });
                ui.col(Col::new(), |ui| {
                    ui.badge(
                        Badge::secondary(format!("Zoom: {:.1}×", ui.context().zoom_factor()))
                            .as_pill(),
                    );
                });
            });
        },
    );

    match active_view {
        DemoView::Overview => overview::render_view_overview(ui),
        DemoView::GridRowCol => grid_row_col::render_view_grid_row_col(ui),
        DemoView::TypographyFontsIcons => {
            typography_fonts_icons::render_view_typography_fonts_icons(ui)
        }
        DemoView::Alerts => alerts::render_view_alerts(ui),
        DemoView::Badges => badges::render_view_badges(ui),
        DemoView::Labels => labels::render_view_labels(ui),
        DemoView::Buttons => buttons::render_view_buttons(ui),
        DemoView::ButtonGroupsListGroups => groups::render_view_groups(ui),
        DemoView::Dropdowns => dropdowns::render_view_dropdowns(ui, demo_state),
        DemoView::Form => form::render_view_form(ui, demo_state),
        DemoView::Cards => cards::render_view_cards(ui),
        DemoView::ProgressSpinners => progress_spinners::render_view_progress_spinners(ui),
        DemoView::Tables => tables::render_view_tables(ui),
        DemoView::Plots => plots::render_view_plots(ui),
    }
}
