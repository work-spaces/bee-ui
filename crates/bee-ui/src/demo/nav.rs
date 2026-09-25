use crate::prelude::*;

use super::{DemoView, render_quick_actions, set_active_view};

pub(super) fn render_demo_nav(ui: &mut UiRoot<'_>, active_view: DemoView) {
    ui.card(
        Card::new()
            .title("runnerd Bootstrap Demo Views")
            .variant(Variant::Primary)
            .footer("Quick actions")
            .outline(),
        move |ui| {
            ui.label(
                Label::new("Select a view to explore one framework concept")
                    .small()
                    .subtle(),
            );

            let mut views = ListGroup::new().flush(true);
            for view in DemoView::ALL {
                views.push_item(
                    ListGroupItem::new(view.title().to_string())
                        .active(view == active_view)
                        .on_click(view_select_handler(view)),
                );
            }
            ui.list_group(views);

            ui.label(Label::new("Quick actions").small().subtle());
            render_quick_actions(ui);
        },
    );
}

macro_rules! define_view_handlers {
    ($( $variant:ident => $handler:ident, )+) => {
        $(
            fn $handler(ctx: &egui::Context) {
                set_active_view(ctx, DemoView::$variant);
            }
        )+

        fn view_select_handler(view: DemoView) -> fn(&egui::Context) {
            match view {
                $(DemoView::$variant => $handler,)+
            }
        }
    };
}

define_view_handlers! {
    Overview => set_view_overview,
    GridRowCol => set_view_grid_row_col,
    TypographyFontsIcons => set_view_typography_fonts_icons,
    Alerts => set_view_alerts,
    Badges => set_view_badges,
    Labels => set_view_labels,
    Buttons => set_view_buttons,
    ButtonGroupsListGroups => set_view_button_groups_list_groups,
    Dropdowns => set_view_dropdowns,
    Form => set_view_form,
    Cards => set_view_cards,
    ProgressSpinners => set_view_progress_spinners,
    Tables => set_view_tables,
    Plots => set_view_plots,
}
