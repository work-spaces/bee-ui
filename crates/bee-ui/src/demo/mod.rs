use crate::prelude::*;

mod nav;
mod state;
mod views;

const ACTIVE_VIEW_ID: &str = "runnerd_bs.demo.active_view";

pub(super) const VARIANTS: [(&str, Variant); 8] = [
    ("Primary", Variant::Primary),
    ("Secondary", Variant::Secondary),
    ("Success", Variant::Success),
    ("Danger", Variant::Danger),
    ("Warning", Variant::Warning),
    ("Info", Variant::Info),
    ("Light", Variant::Light),
    ("Dark", Variant::Dark),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DemoView {
    Overview,
    GridRowCol,
    TypographyFontsIcons,
    Alerts,
    Badges,
    Labels,
    Buttons,
    ButtonGroupsListGroups,
    Dropdowns,
    Form,
    Cards,
    ProgressSpinners,
    Tables,
    Plots,
}

impl DemoView {
    pub(super) const ALL: [Self; 14] = [
        Self::Overview,
        Self::GridRowCol,
        Self::TypographyFontsIcons,
        Self::Alerts,
        Self::Badges,
        Self::Labels,
        Self::Buttons,
        Self::ButtonGroupsListGroups,
        Self::Dropdowns,
        Self::Form,
        Self::Cards,
        Self::ProgressSpinners,
        Self::Tables,
        Self::Plots,
    ];

    const fn storage_value(self) -> u8 {
        match self {
            Self::Overview => 0,
            Self::GridRowCol => 1,
            Self::TypographyFontsIcons => 2,
            Self::Alerts => 3,
            Self::Badges => 4,
            Self::Labels => 5,
            Self::Buttons => 6,
            Self::ButtonGroupsListGroups => 7,
            Self::Dropdowns => 8,
            Self::Cards => 9,
            Self::ProgressSpinners => 10,
            Self::Tables => 11,
            Self::Plots => 12,
            Self::Form => 13,
        }
    }

    const fn from_storage_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Overview),
            1 => Some(Self::GridRowCol),
            2 => Some(Self::TypographyFontsIcons),
            3 => Some(Self::Alerts),
            4 => Some(Self::Badges),
            5 => Some(Self::Labels),
            6 => Some(Self::Buttons),
            7 => Some(Self::ButtonGroupsListGroups),
            8 => Some(Self::Dropdowns),
            9 => Some(Self::Cards),
            10 => Some(Self::ProgressSpinners),
            11 => Some(Self::Tables),
            12 => Some(Self::Plots),
            13 => Some(Self::Form),
            _ => None,
        }
    }

    pub(super) const fn title(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::GridRowCol => "Grid",
            Self::TypographyFontsIcons => "Typography",
            Self::Alerts => "Alerts",
            Self::Badges => "Badges",
            Self::Labels => "Labels",
            Self::Buttons => "Buttons",
            Self::ButtonGroupsListGroups => "Button/List Groups",
            Self::Dropdowns => "Dropdowns",
            Self::Form => "Forms",
            Self::Cards => "Cards",
            Self::ProgressSpinners => "Progress/Spinners",
            Self::Tables => "Tables",
            Self::Plots => "Plots",
        }
    }

    pub(super) const fn subtitle(self) -> &'static str {
        match self {
            Self::Overview => "Scenario intro and demo orientation",
            Self::GridRowCol => "Row/Col gutters, spans, offsets, and ordering",
            Self::TypographyFontsIcons => "Text, labels, rich spans, fonts, and icons",
            Self::Alerts => "Incident, deployment, and service-health alerts",
            Self::Badges => "Status chips, counts, and tagging patterns",
            Self::Labels => "Section metadata labels, sizes, and subtle states",
            Self::Buttons => "Operator actions and callback-driven state",
            Self::ButtonGroupsListGroups => "Grouped actions and list-driven workflows",
            Self::Dropdowns => "Menu disclosure, alignment, and disabled states",
            Self::Form => "Production rollout request workflows with validation",
            Self::Cards => "Summary and detail card composition patterns",
            Self::ProgressSpinners => "Rollout progress and background checks",
            Self::Tables => "Operational datasets with styled and interactive rows",
            Self::Plots => "Latency, error, and throughput trend visualizations",
        }
    }

    pub(super) const fn category(self) -> &'static str {
        match self {
            Self::Overview => "Core",
            Self::GridRowCol => "Layout",
            Self::TypographyFontsIcons => "Typography",
            Self::Alerts => "Feedback",
            Self::Badges => "Feedback",
            Self::Labels => "Typography",
            Self::Buttons => "Actions",
            Self::ButtonGroupsListGroups => "Actions",
            Self::Dropdowns => "Navigation",
            Self::Form => "Inputs",
            Self::Cards => "Composition",
            Self::ProgressSpinners => "Feedback",
            Self::Tables => "Data",
            Self::Plots => "Data",
        }
    }
}

/// Render the full `runnerd-bs` bootstrap-style demo.
pub fn show_bootstrap_demo(ctx: &egui::Context) {
    crate::theme::install_default_theme(ctx);

    let mut demo_state = state::load_demo_state(ctx);

    {
        let mut ui_root = UiRoot::new(ctx);
        bootstrap_demo_container(&mut ui_root, &mut demo_state);
        ui_root.render_with_central_panel_frame(
            egui::Frame::central_panel(&ctx.style()).inner_margin(0),
        );
    }

    state::store_demo_state(ctx, &demo_state);
}

/// Renders a view-based demo shell with left-side navigation and right-side focused content.
pub(super) fn bootstrap_demo_container<'a>(
    ui: &mut UiRoot<'a>,
    demo_state: &'a mut state::DemoState,
) {
    let active_view = active_view(ui.context());

    ui.container(
        ContainerProps::fluid()
            .max_width(1_280.0)
            .padding(Spacing::all(12.0))
            .scroll_y(),
        move |ui| {
            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                ui.col(Col::new().xs(12).lg(3), |ui| {
                    nav::render_demo_nav(ui, active_view);
                });

                ui.col(Col::new().xs(12).lg(9), |ui| {
                    views::render_demo_view(ui, active_view, demo_state);
                });
            });
        },
    );
}

fn active_view(ctx: &egui::Context) -> DemoView {
    ctx.data_mut(|data| {
        data.get_persisted::<u8>(egui::Id::new(ACTIVE_VIEW_ID))
            .and_then(DemoView::from_storage_value)
            .unwrap_or(DemoView::Overview)
    })
}

pub(super) fn set_active_view(ctx: &egui::Context, view: DemoView) {
    ctx.data_mut(|data| {
        data.insert_persisted(egui::Id::new(ACTIVE_VIEW_ID), view.storage_value());
    });
}

pub(super) fn render_quick_actions(ui: &mut UiRoot<'_>) {
    let (toggle_button_text, toggle_button_hover) = match ui.context().theme() {
        egui::Theme::Dark => ("Toggle theme (Light)", "Switch the demo to the light theme"),
        egui::Theme::Light => ("Toggle theme (Dark)", "Switch the demo to the dark theme"),
    };

    ui.button(
        Button::secondary(toggle_button_text)
            .hover_text(toggle_button_hover)
            .on_click(toggle_demo_theme),
    );
    ui.button(
        Button::secondary("Zoom +")
            .hover_text("Zoom in")
            .on_click(zoom_in),
    );
    ui.button(
        Button::secondary("Zoom -")
            .hover_text("Zoom out")
            .on_click(zoom_out),
    );
}

pub(super) fn theme_badge_text(theme: egui::Theme) -> &'static str {
    match theme {
        egui::Theme::Dark => "Theme: Dark",
        egui::Theme::Light => "Theme: Light",
    }
}

fn toggle_demo_theme(ctx: &egui::Context) {
    crate::theme::toggle_theme(ctx);
}

fn zoom_in(ctx: &egui::Context) {
    let next_zoom = (ctx.zoom_factor() + 0.1).clamp(0.5, 3.0);
    ctx.set_zoom_factor(next_zoom);
}

fn zoom_out(ctx: &egui::Context) {
    let next_zoom = (ctx.zoom_factor() - 0.1).clamp(0.5, 3.0);
    ctx.set_zoom_factor(next_zoom);
}
