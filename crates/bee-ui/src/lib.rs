//! `runnerd-bs` provides Bootstrap-like semantic components on top of `egui`.
//!
//! The API targets **semantic parity** (`Container`, `Row`, `Col`, `Card`, `Badge`, `Label`,
//! `Button`, and plot wrappers) rather than pixel-perfect CSS/Bootstrap rendering.
//!
//! ## Bootstrap-to-`runnerd-bs` mapping
//!
//! | Bootstrap concept | `runnerd-bs` API |
//! | --- | --- |
//! | `.container` / `.container-fluid` | `Container::fixed()` / `Container::fluid()` |
//! | `.row` + gutter utilities | `Row::new().gutter_x(...).gutter_y(...)` |
//! | `.col-*` responsive grid | `Col::new().xs(...).sm(...).md(...).lg(...)` |
//! | `.card` with header/footer | `Card::new().title(...).footer(...)` |
//! | `.badge.bg-*` / `.rounded-pill` | `Badge::new(...).variant(...).pill(true)` |
//! | Contextual text style variants | `Label::new(...).variant(...)` |
//! | Contextual button variants | `Button::new(...).variant(...)` |
//! | Card with embedded chart | `ChartCard` + `PlotArea` |
//!
//! ## Rich `Span` labels and captions
//!
//! Most display-text component APIs accept `impl Into<runnerd_bs::core::Span>`, so existing
//! `&str` and `String` call sites continue to work while enabling rich styled runs when needed.
//! `Button` labels are intentionally plain text (`impl Into<String>`) with an optional icon.
//!
//! ```no_run
//! use runnerd_bs::{
//!     core::{Span, SpanText, SpanTextStyle},
//!     prelude::*,
//! };
//!
//! let status: Span = Span::from(
//!     SpanText::new("Healthy")
//!         .style(SpanTextStyle::new().strong(true).color(egui::Color32::LIGHT_GREEN)),
//! );
//!
//! let button = Button::new(String::from("Restart service"));
//! let badge = Badge::new(status);
//!
//! let _ = (button, badge);
//! ```
//!
//! ## Responsive grid tutorial
//!
//! `runnerd-bs` resolves responsive `Col` spans by active breakpoint, then packs each row into
//! a 12-column line budget (wrapping when needed).
//!
//! ```no_run
//! use runnerd_bs::prelude::*;
//!
//! let ctx = egui::Context::default();
//!
//! show_root(&ctx, |ui| {
//!     ui.container(Container::fluid(), |ui| {
//!         ui.row(Row::new().gutter_x(16.0).gutter_y(12.0), |ui| {
//!             // xs: full width, md+: 8/12
//!             ui.col(Col::new().xs(12).md(8), |ui| {
//!                 ui.card(Card::new().title("Primary panel"), |ui| {
//!                     ui.label(Label::new("Main content"));
//!                 });
//!             });
//!
//!             // xs: full width, md+: 4/12
//!             ui.col(Col::new().xs(12).md(4), |ui| {
//!                 ui.card(Card::new().title("Side panel"), |ui| {
//!                     ui.badge(Badge::info("Responsive"));
//!                 });
//!             });
//!         });
//!     });
//! });
//! ```
//!
//! ## Dashboard example: Card + Badge + Plot
//!
//! ```no_run
//! use runnerd_bs::prelude::*;
//!
//! let ctx = egui::Context::default();
//!
//! show_root(&ctx, |ui| {
//!     ui.container(Container::fluid(), |ui| {
//!         ui.row(Row::new().gutter_x(12.0).gutter_y(12.0), |ui| {
//!             ui.col(Col::new().xs(12).lg(8), |ui| {
//!                 ui.card(Card::new().title("CPU Usage").footer("Rolling 5m"), |ui| {
//!                     ui.badge(Badge::success("Healthy").as_pill());
//!                     ui.plot(
//!                         PlotArea::new("cpu_usage")
//!                             .title("CPU %")
//!                             .axis_labels("Sample", "Percent")
//!                             .height(180.0)
//!                             .legend(true),
//!                         |_plot_ui| {
//!                             // Add egui_plot lines/points here.
//!                         },
//!                     );
//!                 });
//!             });
//!
//!             ui.col(Col::new().xs(12).lg(4), |ui| {
//!                 ui.chart_card(
//!                     ChartCard::new("Latency", "latency_plot")
//!                         .variant(Variant::Info)
//!                         .plot_height(180.0),
//!                     |_plot_ui| {
//!                         // Add egui_plot lines/points here.
//!                     },
//!                 );
//!             });
//!         });
//!     });
//! });
//! ```
//!
//! ## Theming guide (twill token integration path)
//!
//! `runnerd-bs` is designed to map cleanly from `twill` (or equivalent) design tokens.
//! A practical path is:
//! 1. read tokens from your app theme layer,
//! 2. map them into `UiTheme`,
//! 3. apply those values consistently through semantic components.
//!
//! ```
//! use runnerd_bs::{core::UiTheme, layout::Breakpoints};
//!
//! #[derive(Clone, Copy)]
//! struct AppTokens {
//!     sm: u16,
//!     md: u16,
//!     lg: u16,
//!     xl: u16,
//!     xxl: u16,
//!     space_md: f32,
//!     space_lg: f32,
//! }
//!
//! fn theme_from_tokens(tokens: AppTokens) -> UiTheme {
//!     let mut theme = UiTheme::default();
//!     theme.breakpoints = Breakpoints {
//!         sm: tokens.sm,
//!         md: tokens.md,
//!         lg: tokens.lg,
//!         xl: tokens.xl,
//!         xxl: tokens.xxl,
//!     };
//!     theme.spacing.md = tokens.space_md;
//!     theme.spacing.lg = tokens.space_lg;
//!     theme
//! }
//!
//! let _theme = theme_from_tokens(AppTokens {
//!     sm: 576,
//!     md: 768,
//!     lg: 992,
//!     xl: 1200,
//!     xxl: 1400,
//!     space_md: 8.0,
//!     space_lg: 12.0,
//! });
//! ```
//!
//! ## Raw `egui` interop guide
//!
//! Use escape hatches when you need lower-level `egui` control in a semantic layout:
//! - `UiRoot::raw` for direct `egui::Ui` access,
//! - `UiRoot::with_style_override` for scoped style mutation,
//! - `UiRoot::with_theme_variant` for light/dark subtree switching,
//! - `runnerd_bs::interop::with_raw_egui` for direct `egui::Context` access.
//!
//! ```no_run
//! use runnerd_bs::prelude::*;
//!
//! let ctx = egui::Context::default();
//!
//! show_root(&ctx, |ui| {
//!     ui.with_theme_variant(egui::Theme::Dark, |ui| {
//!         ui.with_style_override(
//!             |style| {
//!                 style.spacing.item_spacing = egui::vec2(4.0, 4.0);
//!             },
//!             |ui| {
//!                 ui.raw(|egui_ui| {
//!                     egui_ui.label("raw egui label");
//!                 });
//!
//!                 runnerd_bs::interop::with_raw_egui(ui, |context| {
//!                     let _ = context.pixels_per_point();
//!                 });
//!
//!                 ui.badge(Badge::info("Interop"));
//!             },
//!         );
//!     });
//! });
//! ```

pub mod components;
pub mod core;
pub mod demo;
pub mod interop;
pub mod layout;
pub mod plot;
pub mod prelude;
pub mod render;
pub mod theme;

pub use crate::core::context::UiRoot;

/// Entry point for building a semantic `runnerd-bs` tree in an `egui::Context`.
pub fn show_root<'a>(ctx: &egui::Context, f: impl FnOnce(&mut UiRoot<'a>)) {
    let mut ui_root = UiRoot::new(ctx);
    f(&mut ui_root);
    ui_root.render_with_central_panel();
}

/// Builds a semantic `runnerd-bs` tree and renders it inline into an existing `egui::Ui`.
///
/// Use this for reusable widgets that receive a parent `egui::Ui` (for example, from inside
/// `UiRoot::raw`). Controlled state bound with `form_*` is written back before this returns.
pub fn show_in_ui<'a>(ui: &mut egui::Ui, f: impl FnOnce(&mut UiRoot<'a>)) {
    let mut ui_root = UiRoot::new(ui.ctx());
    f(&mut ui_root);
    ui_root.render_in_ui(ui);
}
