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
//! ## Immediate mode and responses
//!
//! `UiRoot` is a thin wrapper around `egui::Ui`. Like egui, it draws each component as soon
//! as it's called, so interactive components report input from the same pass:
//!
//! | `UiRoot` method | Returns |
//! | --- | --- |
//! | `button`, `close_button` | `egui::Response` (`.clicked()`, `.hovered()`, `.on_hover_text()`, ...) |
//! | `button_group`, `list_group`, `nav`, `pagination`, `table`, `dropdown` | [`ItemClickResponse`](components::ItemClickResponse) (`.clicked()`, `.clicked_index()`, `.item_clicked(i)`) |
//! | `navbar` | [`NavbarResponse`](components::NavbarResponse) (item clicks plus `.brand_clicked()`) |
//! | `form_control`, `form_select`, `form_check`, `form_range`, `floating_label` | [`ChangeResponse`](components::ChangeResponse) (`.changed()`) |
//! | `container`, `row`, `col`, `hstack`, `card`, `with_*` scopes, `raw` | the closure's return value |
//! | display-only widgets (`badge`, `label`, `text`, `alert`, ...) | `()` |
//!
//! ```no_run
//! use bee_ui::prelude::*;
//!
//! let ctx = egui::Context::default();
//! let mut selected_row = 0;
//! let mut replicas = 3.0_f32;
//!
//! let _ = ctx.run(egui::RawInput::default(), |ctx| {
//!     show_root(ctx, |ui| {
//!         if ui.button(Button::primary("Deploy")).clicked() {
//!             // deploy();
//!         }
//!
//!         let services = Table::new()
//!             .headers(["Service", "State"])
//!             .row(["scheduler", "Healthy"])
//!             .row(["api-gateway", "Degraded"]);
//!         if let Some(row) = ui.table(services).clicked_index() {
//!             selected_row = row;
//!         }
//!
//!         if ui.form_range(&mut replicas, FormRange::new(1.0, 10.0)).changed() {
//!             // scale(replicas);
//!         }
//!     });
//! });
//! ```
//!
//! Controlled values (`&mut String`, `&mut bool`, `&mut usize`, ...) are updated before the
//! method returns, so plain local borrows work; nothing needs to outlive the closure.
//! `on_click(fn)` / `on_change(fn)` callbacks still work and fire right after the widget is
//! drawn.
//!
//! ### Row sizing passes
//!
//! A `Row` can't know every column's width until all of its columns have been declared, so it
//! lays out from the column specs it recorded on the previous pass (the same approach as
//! `egui::Grid`). When a row appears for the first time, or its columns change, it stores the
//! new specs and calls `egui::Context::request_discard`, and egui re-runs the pass before
//! anything is shown. As a result:
//!
//! - Build closures must be safe to run twice in one frame. Use elapsed time rather than
//!   counting passes or frames.
//! - With egui's default `Options::max_passes` of 2, an outer and a nested row changing in the
//!   same frame can show one frame of stale layout. Raise `max_passes` if that matters.
//! - Rows are identified by their position among their siblings. Give rows whose position
//!   changes (for example, rows inside a filtered list) a stable
//!   [`Row::id_salt`](components::Row::id_salt) so they keep their stored layout.
//!
//! All entry points ([`show_root`], [`show_root_with_frame`], [`show_in_ui`]) must run inside
//! an egui pass (`Context::run` or eframe's `update`).
//!
//! ## Rich `Span` labels and captions
//!
//! Most display-text component APIs accept `impl Into<bee_ui::core::Span>`, so existing
//! `&str` and `String` call sites continue to work while enabling rich styled runs when needed.
//! `Button` labels are intentionally plain text (`impl Into<String>`) with an optional icon.
//!
//! ```no_run
//! use bee_ui::{
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
//! use bee_ui::prelude::*;
//!
//! let ctx = egui::Context::default();
//!
//! let _ = ctx.run(egui::RawInput::default(), |ctx| {
//!     show_root(ctx, |ui| {
//!         ui.container(Container::fluid(), |ui| {
//!             ui.row(Row::new().gutter_x(16.0).gutter_y(12.0), |ui| {
//!                 // xs: full width, md+: 8/12
//!                 ui.col(Col::new().xs(12).md(8), |ui| {
//!                     ui.card(Card::new().title("Primary panel"), |ui| {
//!                         ui.label(Label::new("Main content"));
//!                     });
//!                 });
//!
//!                 // xs: full width, md+: 4/12
//!                 ui.col(Col::new().xs(12).md(4), |ui| {
//!                     ui.card(Card::new().title("Side panel"), |ui| {
//!                         ui.badge(Badge::info("Responsive"));
//!                     });
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
//! use bee_ui::prelude::*;
//!
//! let ctx = egui::Context::default();
//!
//! let _ = ctx.run(egui::RawInput::default(), |ctx| {
//!     show_root(ctx, |ui| {
//!         ui.container(Container::fluid(), |ui| {
//!             ui.row(Row::new().gutter_x(12.0).gutter_y(12.0), |ui| {
//!                 ui.col(Col::new().xs(12).lg(8), |ui| {
//!                     ui.card(Card::new().title("CPU Usage").footer("Rolling 5m"), |ui| {
//!                         ui.badge(Badge::success("Healthy").as_pill());
//!                         ui.plot(
//!                             PlotArea::new("cpu_usage")
//!                                 .title("CPU %")
//!                                 .axis_labels("Sample", "Percent")
//!                                 .height(180.0)
//!                                 .legend(true),
//!                             |_plot_ui| {
//!                                 // Add egui_plot lines/points here.
//!                             },
//!                         );
//!                     });
//!                 });
//!
//!                 ui.col(Col::new().xs(12).lg(4), |ui| {
//!                     ui.chart_card(
//!                         ChartCard::new("Latency", "latency_plot")
//!                             .variant(Variant::Info)
//!                             .plot_height(180.0),
//!                         |_plot_ui| {
//!                             // Add egui_plot lines/points here.
//!                         },
//!                     );
//!                 });
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
//! use bee_ui::{core::UiTheme, layout::Breakpoints};
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
//! - `bee_ui::interop::with_raw_egui` for direct `egui::Context` access.
//!
//! ```no_run
//! use bee_ui::prelude::*;
//!
//! let ctx = egui::Context::default();
//!
//! let _ = ctx.run(egui::RawInput::default(), |ctx| {
//!     show_root(ctx, |ui| {
//!         ui.with_theme_variant(egui::Theme::Dark, |ui| {
//!             ui.with_style_override(
//!                 |style| {
//!                     style.spacing.item_spacing = egui::vec2(4.0, 4.0);
//!                 },
//!                 |ui| {
//!                     ui.raw(|egui_ui| {
//!                         egui_ui.label("raw egui label");
//!                     });
//!
//!                     bee_ui::interop::with_raw_egui(ui, |context| {
//!                         let _ = context.pixels_per_point();
//!                     });
//!
//!                     ui.badge(Badge::info("Interop"));
//!                 },
//!             );
//!         });
//!     });
//! });
//! ```

pub mod chart;
pub mod components;
pub mod core;
pub mod demo;
pub mod interop;
pub mod layout;
pub mod prelude;
pub mod render;
pub mod theme;

use crate::core::context::RootState;
pub use crate::core::context::UiRoot;

/// Draws semantic components into a `CentralPanel` of `ctx` and returns `f`'s value.
///
/// Components draw immediately, so responses (`.clicked()`, `.changed()`, ...) can be
/// checked inside `f`. Must be called inside an egui pass (`Context::run` or eframe's
/// `update`).
pub fn show_root<R>(ctx: &egui::Context, f: impl FnOnce(&mut UiRoot<'_>) -> R) -> R {
    show_root_with_frame(ctx, egui::Frame::central_panel(&ctx.style()), f)
}

/// Like [`show_root`], but draws the `CentralPanel` with a custom `frame`.
pub fn show_root_with_frame<R>(
    ctx: &egui::Context,
    frame: egui::Frame,
    f: impl FnOnce(&mut UiRoot<'_>) -> R,
) -> R {
    egui::CentralPanel::default()
        .frame(frame)
        .show(ctx, |ui| show_in_ui(ui, f))
        .inner
}

/// Draws semantic components inline into an existing `egui::Ui` and returns `f`'s value.
///
/// Use this for reusable widgets that receive a parent `egui::Ui` (for example, from inside
/// `UiRoot::raw`). Components draw into `ui` as they're called, so `f` can return whatever
/// it read from their responses.
pub fn show_in_ui<R>(ui: &mut egui::Ui, f: impl FnOnce(&mut UiRoot<'_>) -> R) -> R {
    let mut state = RootState::default();
    f(&mut UiRoot::new(ui, &mut state))
}
