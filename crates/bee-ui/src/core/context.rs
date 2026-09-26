use std::fmt;

use crate::{
    components::{
        ActiveIndexState, AlertProps, BadgeProps, Button, ButtonGroupProps, CardProps,
        CloseButtonProps, ColProps, ColValidationIssue, ContainerProps, ContainerScroll,
        DisclosureState, DropdownProps, FloatingLabelProps, FormCheckProps, FormControlProps,
        FormLabelProps, FormRangeProps, FormSelectProps, FormTextProps, HStack, LabelProps,
        ListGroupProps, NavProps, NavbarProps, PaginationProps, PlaceholderProps, ProgressProps,
        RowProps, SpinnerProps, TableProps, TextProps, ValidationFeedbackProps,
    },
    core::{
        response::{ChangeResponse, ItemClickResponse, NavbarResponse},
        row_layout::RowCursor,
    },
    layout::{ComputedCol, Span, resolve_active_breakpoint},
    plot::{
        ChartCard, PlotArea, PlotDataSource, TimeSeriesDataset, TimeSeriesPlot,
        TimeSeriesPlotProps, ViewportState,
    },
    render::{Backend, EguiBackend},
};

type StrictErrorReporter = Box<dyn FnMut(&UiValidationError)>;
type ClickHandler = fn(&egui::Context);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiValidationError {
    pub component: &'static str,
    pub prop: &'static str,
    pub value: String,
    pub detail: String,
}

impl UiValidationError {
    fn new(
        component: &'static str,
        prop: &'static str,
        value: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            component,
            prop,
            value: value.into(),
            detail: detail.into(),
        }
    }

    fn from_col_issue(issue: ColValidationIssue) -> Self {
        match issue {
            ColValidationIssue::SpanOutOfRange { prop, columns } => Self::new(
                "ColProps",
                prop,
                columns.to_string(),
                format!(
                    "span must be in {}..={} or `Span::Auto`",
                    Span::MIN_COLUMNS,
                    Span::MAX_COLUMNS
                ),
            ),
            ColValidationIssue::OffsetOutOfRange { offset, max } => Self::new(
                "ColProps",
                "offset",
                offset.to_string(),
                format!("offset must be in 0..={} for a 12-column grid", max),
            ),
            ColValidationIssue::OrderOutOfRange { order, min, max } => Self::new(
                "ColProps",
                "order",
                order.to_string(),
                format!(
                    "order must be in {}..={} for semantic Bootstrap-like ordering",
                    min, max
                ),
            ),
        }
    }
}

impl fmt::Display for UiValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid semantic prop `{}.{}` value `{}`: {}",
            self.component, self.prop, self.value, self.detail
        )
    }
}

impl std::error::Error for UiValidationError {}

/// State shared by every `UiRoot` created during one `show_root` / `show_in_ui` call.
///
/// It has no lifetime parameter so child `UiRoot`s can reborrow it freely.
#[derive(Default)]
pub(crate) struct RootState {
    backend: EguiBackend,
    strict_mode: bool,
    strict_mode_errors: Vec<UiValidationError>,
    strict_mode_error_reporter: Option<StrictErrorReporter>,
    table_render_seq: u64,
}

impl RootState {
    fn report_col_validation_issues(&mut self, props: &ColProps) {
        if !self.strict_mode {
            return;
        }

        for issue in props.validation_issues() {
            self.report_validation_error(UiValidationError::from_col_issue(issue));
        }
    }

    fn report_validation_error(&mut self, error: UiValidationError) {
        if !self.strict_mode {
            return;
        }

        if let Some(reporter) = self.strict_mode_error_reporter.as_mut() {
            reporter(&error);
        }

        self.strict_mode_errors.push(error);
    }

    fn next_table_render_seq(&mut self) -> u64 {
        let seq = self.table_render_seq;
        self.table_render_seq = self.table_render_seq.wrapping_add(1);
        seq
    }
}

/// Immediate-mode semantic builder wrapping an `egui::Ui`.
///
/// Every component method draws as soon as it is called. Scope methods (`container`, `col`,
/// `card`, ...) run their closure inside the matching egui scope and return its value.
pub struct UiRoot<'ui> {
    ui: &'ui mut egui::Ui,
    state: &'ui mut RootState,
    /// Set while this `UiRoot` is the body of a `row`: every component then takes the next
    /// column slot instead of drawing into `ui` directly.
    row: Option<&'ui mut RowCursor>,
}

impl<'ui> UiRoot<'ui> {
    pub(crate) fn new(ui: &'ui mut egui::Ui, state: &'ui mut RootState) -> Self {
        Self {
            ui,
            state,
            row: None,
        }
    }

    pub fn context(&self) -> &egui::Context {
        self.ui.ctx()
    }

    pub fn set_strict_mode(&mut self, enabled: bool) -> &mut Self {
        self.state.strict_mode = enabled;
        self
    }

    pub const fn strict_mode_enabled(&self) -> bool {
        self.state.strict_mode
    }

    pub fn set_strict_mode_error_reporter(
        &mut self,
        reporter: impl FnMut(&UiValidationError) + 'static,
    ) -> &mut Self {
        self.state.strict_mode_error_reporter = Some(Box::new(reporter));
        self
    }

    pub fn clear_strict_mode_error_reporter(&mut self) -> &mut Self {
        self.state.strict_mode_error_reporter = None;
        self
    }

    pub fn strict_mode_errors(&self) -> &[UiValidationError] {
        &self.state.strict_mode_errors
    }

    pub fn take_strict_mode_errors(&mut self) -> Vec<UiValidationError> {
        std::mem::take(&mut self.state.strict_mode_errors)
    }

    fn sync_theme_mode(&mut self) {
        self.state.backend.sync_mode_from_ui(self.ui);
    }

    const fn in_row(&self) -> bool {
        self.row.is_some()
    }

    /// Draws a component called directly inside a row into its own auto column.
    fn loose_child<R>(&mut self, f: impl FnOnce(&mut UiRoot<'_>) -> R) -> R {
        self.row_col(ColProps::default(), f)
    }

    /// Draws `f` into the next column slot of the enclosing row.
    fn row_col<R>(&mut self, props: ColProps, f: impl FnOnce(&mut UiRoot<'_>) -> R) -> R {
        let Some(cursor) = self.row.as_deref_mut() else {
            debug_assert!(false, "`row_col` called outside a row");
            return f(self);
        };
        let row_ui = &mut *self.ui;
        let state = &mut *self.state;
        let row_props = *cursor.props();

        let (line_index, computed) = cursor.next_placement(&props);
        if cursor.open_line_index() != Some(line_index) {
            if let Some(line) = cursor.close_line() {
                state.backend.end_row(row_ui, &row_props, &line);
            }
            let line = cursor.open_line(line_index, &computed);
            state.backend.begin_row(row_ui, &row_props, line);
        }

        let slot = cursor.slot_rect(&computed);
        state.backend.begin_col(row_ui, &props, &computed);

        let mut builder = egui::UiBuilder::new()
            .id_salt(("bee_ui_row_col", computed.source_index))
            .max_rect(slot)
            .layout(egui::Layout::top_down(egui::Align::Min));
        if cursor.is_first_appearance() {
            // This pass is discarded once the row has measured its columns, like `egui::Grid`'s
            // first frame, so don't show the placeholder layout if the discard is denied.
            builder = builder.invisible();
        }
        let mut col_ui = row_ui.new_child(builder);
        let inner = f(&mut UiRoot::new(&mut col_ui, state));
        cursor.record_col(slot, col_ui.min_rect());

        state.backend.end_col(row_ui, &props, &computed);
        inner
    }

    /// Runs `f` only while the disclosure state is open, returning `None` when closed.
    ///
    /// Controlled state (`&mut bool`) is preferred so callers keep visibility explicit.
    pub fn with_disclosure_state<'s, R>(
        &mut self,
        state: impl Into<DisclosureState<'s>>,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> Option<R> {
        let disclosure = state.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.with_disclosure_state(disclosure, f));
        }
        self.sync_theme_mode();

        let open = disclosure.current(self.ui.ctx());
        self.state.backend.begin_disclosure_scope(self.ui, open);
        let inner = if open { Some(f(self)) } else { None };
        self.state.backend.end_disclosure_scope(self.ui, open);
        inner
    }

    /// Runs `f` inside an active-index scope.
    ///
    /// Controlled state (`&mut usize`) is preferred so callers keep selection explicit.
    pub fn with_active_index_state<'s, R>(
        &mut self,
        state: impl Into<ActiveIndexState<'s>>,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        let active_index_state = state.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.with_active_index_state(active_index_state, f));
        }
        self.sync_theme_mode();

        let active_index = active_index_state.current(self.ui.ctx());
        self.state
            .backend
            .begin_active_index_scope(self.ui, active_index);
        let inner = f(self);
        self.state
            .backend
            .end_active_index_scope(self.ui, active_index);
        inner
    }

    pub(crate) fn interop_raw<R>(&mut self, f: impl FnOnce(&mut egui::Ui) -> R) -> R {
        if self.in_row() {
            return self.loose_child(|ui| ui.interop_raw(f));
        }
        self.sync_theme_mode();
        f(self.ui)
    }

    pub(crate) fn interop_with_style_override<R>(
        &mut self,
        style_override: impl FnOnce(&mut egui::Style),
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        if self.in_row() {
            return self.loose_child(|ui| ui.interop_with_style_override(style_override, f));
        }
        self.sync_theme_mode();

        let mut style = self.ui.style().as_ref().clone();
        style_override(&mut style);

        let state = &mut *self.state;
        self.ui
            .scope(|scope_ui| {
                *scope_ui.style_mut() = style;
                f(&mut UiRoot::new(scope_ui, state))
            })
            .inner
    }

    pub(crate) fn interop_with_theme_variant<R>(
        &mut self,
        theme: egui::Theme,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        if self.in_row() {
            return self.loose_child(|ui| ui.interop_with_theme_variant(theme, f));
        }
        self.sync_theme_mode();

        let mut themed_style = self.ui.style().as_ref().clone();
        themed_style.visuals = theme.default_visuals();

        let state = &mut *self.state;
        self.ui
            .scope(|scope_ui| {
                *scope_ui.style_mut() = themed_style;
                f(&mut UiRoot::new(scope_ui, state))
            })
            .inner
    }

    pub fn container<R>(
        &mut self,
        container: impl Into<ContainerProps>,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        let props = container.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.container(props, f));
        }
        self.sync_theme_mode();

        let ui = &mut *self.ui;
        let state = &mut *self.state;
        state.backend.begin_container(ui, &props);

        let scroll_area = match props.scroll {
            ContainerScroll::None => None,
            ContainerScroll::Horizontal => Some(egui::ScrollArea::horizontal()),
            ContainerScroll::Vertical => Some(egui::ScrollArea::vertical()),
            ContainerScroll::Both => Some(egui::ScrollArea::both()),
        };

        let inner = match scroll_area {
            None => container_body(ui, state, props, f),
            Some(scroll_area) => {
                scroll_area
                    .auto_shrink([false, false])
                    .show(ui, |scroll_ui| container_body(scroll_ui, state, props, f))
                    .inner
            }
        };

        state.backend.end_container(ui, &props);
        inner
    }

    /// Lays out children as columns of a 12-column grid.
    ///
    /// Each `col` call takes the next column; any other component called directly in the row
    /// becomes its own auto-width column. Columns wrap onto new lines when their spans exceed
    /// 12, separated by the row's vertical gutter.
    ///
    /// Columns are drawn immediately, so the row places them using the specs it recorded on the
    /// previous pass. When the specs change (including the row's first appearance), the pass is
    /// discarded with `egui::Context::request_discard` and re-run with the corrected layout, so
    /// build code must be safe to run twice in one frame. See [`Row::id_salt`] for rows whose
    /// position among their siblings changes.
    ///
    /// [`Row::id_salt`]: crate::components::Row::id_salt
    pub fn row<R>(&mut self, row: impl Into<RowProps>, f: impl FnOnce(&mut UiRoot<'_>) -> R) -> R {
        let props: RowProps = row.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.row(props, f));
        }
        self.sync_theme_mode();

        let id = match props.id_salt {
            Some(salt) => self.ui.make_persistent_id(salt),
            None => self.ui.next_auto_id(),
        };
        let available = self.ui.available_rect_before_wrap();
        let breakpoint =
            resolve_active_breakpoint(available.width(), self.state.backend.breakpoints());
        let line_gap = self.ui.spacing().item_spacing.y + props.spacing().gutter_y();
        let mut cursor = RowCursor::new(self.ui.ctx(), id, props, breakpoint, available, line_gap);

        let mut builder = egui::UiBuilder::new()
            .max_rect(available)
            .layout(egui::Layout::top_down(egui::Align::Min));
        if let Some(salt) = props.id_salt {
            builder = builder.id_salt(salt);
        }
        let mut row_ui = self.ui.new_child(builder);

        let state = &mut *self.state;
        let inner = f(&mut UiRoot {
            ui: &mut row_ui,
            state: &mut *state,
            row: Some(&mut cursor),
        });

        if let Some(line) = cursor.close_line() {
            state.backend.end_row(&mut row_ui, &props, &line);
        }
        if let Some(row_rect) = cursor.finish(self.ui.ctx()) {
            self.ui.advance_cursor_after_rect(row_rect);
        }
        inner
    }

    /// Draws `f` as a grid column.
    ///
    /// Inside a `row`, it takes the row's next column slot. Elsewhere it fills the available
    /// width.
    pub fn col<R>(&mut self, col: impl Into<ColProps>, f: impl FnOnce(&mut UiRoot<'_>) -> R) -> R {
        let props = col.into();
        self.state.report_col_validation_issues(&props);
        if self.in_row() {
            return self.row_col(props, f);
        }
        self.sync_theme_mode();

        let ui = &mut *self.ui;
        let state = &mut *self.state;
        let computed = ComputedCol {
            source_index: 0,
            span: Span::Auto,
            offset: 0,
            order: None,
            x: 0.0,
            width: ui.available_width(),
        };

        state.backend.begin_col(ui, &props, &computed);
        let inner = ui
            .allocate_ui_with_layout(
                egui::vec2(computed.width.max(0.0), 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |col_ui| f(&mut UiRoot::new(col_ui, state)),
            )
            .inner;
        state.backend.end_col(ui, &props, &computed);
        inner
    }

    pub fn hstack<R>(
        &mut self,
        stack: impl Into<HStack>,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        let props = stack.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.hstack(props, f));
        }
        self.sync_theme_mode();

        let state = &mut *self.state;
        let body = |stack_ui: &mut egui::Ui| {
            stack_ui.spacing_mut().item_spacing.x = props.resolved_gap();
            f(&mut UiRoot::new(stack_ui, state))
        };

        if props.wrap {
            self.ui.horizontal_wrapped(body).inner
        } else {
            self.ui.horizontal(body).inner
        }
    }

    pub fn card<R>(
        &mut self,
        card: impl Into<CardProps>,
        f: impl FnOnce(&mut UiRoot<'_>) -> R,
    ) -> R {
        let props = card.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.card(props, f));
        }
        self.sync_theme_mode();

        let state = &mut *self.state;
        // `render_card` keeps the backend borrowed while the body runs, so components inside the
        // card draw with a fresh backend until the outer one is put back.
        let mut backend = std::mem::take(&mut state.backend);
        let style = backend.card_style(&props);
        let theme = backend.theme().clone();

        let inner = backend.render_card(self.ui, &props, |card_ui| {
            if let Some(header) = &props.header {
                EguiBackend::render_heading_span(card_ui, header, style.header_text, &theme);
                if style.header_gap > 0.0 {
                    card_ui.add_space(style.header_gap);
                }
            }

            let inner = f(&mut UiRoot::new(card_ui, state));

            if let Some(footer) = &props.footer {
                if style.footer_gap > 0.0 {
                    card_ui.add_space(style.footer_gap);
                }
                card_ui.separator();
                EguiBackend::render_plain_span(card_ui, footer, style.footer_text, &theme);
            }

            inner
        });

        state.backend = backend;
        inner
    }

    pub fn chart_card<R, F>(&mut self, chart: impl Into<ChartCard>, f: F) -> R
    where
        F: for<'plot_ui> FnOnce(&mut egui_plot::PlotUi<'plot_ui>) -> R,
    {
        let chart = chart.into();
        self.card(chart.card, |ui| ui.plot(chart.plot, f))
    }

    pub fn plot<R, F>(&mut self, plot: impl Into<PlotArea>, f: F) -> R
    where
        F: for<'plot_ui> FnOnce(&mut egui_plot::PlotUi<'plot_ui>) -> R,
    {
        let props = plot.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.plot(props, f));
        }
        self.sync_theme_mode();

        let ui = &mut *self.ui;
        let style = self.state.backend.card_style(&CardProps::new());

        if let Some(title) = &props.title {
            let bold_face = crate::theme::fonts::FontFace::PropoBold;
            let rich_text = if crate::theme::fonts::ensure_face_registered(ui.ctx(), bold_face) {
                egui::RichText::new(title)
                    .family(crate::theme::fonts::font_family_for(bold_face))
                    .color(style.header_text)
            } else {
                egui::RichText::new(title).strong().color(style.header_text)
            };

            ui.label(rich_text);
            if style.header_gap > 0.0 {
                ui.add_space(style.header_gap);
            }
        }

        let plot_widget = props.build_plot(ui.available_width());
        ui.scope(|plot_scope| {
            let visuals = plot_scope.visuals_mut();
            visuals.override_text_color = Some(style.body_text);
            visuals.widgets.noninteractive.fg_stroke.color = style.border;
            visuals.widgets.noninteractive.bg_stroke.color = style.border;

            plot_widget.show(plot_scope, f).inner
        })
        .inner
    }

    pub fn time_series_plot(
        &mut self,
        plot: impl Into<TimeSeriesPlotProps>,
        dataset: &TimeSeriesDataset,
        viewport: &mut ViewportState,
    ) {
        let props = plot.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.time_series_plot(props, dataset, viewport));
        }
        self.sync_theme_mode();
        TimeSeriesPlot::show(self.ui, &props, dataset, viewport);
    }

    pub fn time_series_plot_from_source<S>(
        &mut self,
        plot: impl Into<TimeSeriesPlotProps>,
        source: &S,
        viewport: &mut ViewportState,
    ) where
        S: PlotDataSource + ?Sized,
    {
        let props = plot.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.time_series_plot_from_source(props, source, viewport));
        }
        self.sync_theme_mode();
        let dataset = source.snapshot();
        TimeSeriesPlot::show(self.ui, &props, &dataset, viewport);
    }

    pub fn badge(&mut self, badge: impl Into<BadgeProps>) {
        let props = badge.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.badge(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_badge(self.ui, &props);
    }

    pub fn label(&mut self, label: impl Into<LabelProps>) {
        let props = label.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.label(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_label(self.ui, &props);
    }

    pub fn text(&mut self, text: impl Into<TextProps>) {
        let props = text.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.text(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_text(self.ui, &props);
    }

    /// Draws a button and returns its `egui::Response`, so `.clicked()`, `.hovered()`,
    /// `.on_hover_text()` and friends work as in egui. `Button::on_click` still fires on click.
    pub fn button(&mut self, button: Button) -> egui::Response {
        if self.in_row() {
            return self.loose_child(|ui| ui.button(button));
        }
        self.sync_theme_mode();
        let response = self.state.backend.render_button(self.ui, &button);
        if response.clicked()
            && let Some(on_click) = button.on_click
        {
            on_click(self.ui.ctx());
        }
        response
    }

    pub fn alert(&mut self, alert: impl Into<AlertProps>) {
        let props = alert.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.alert(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_alert(self.ui, &props);
    }

    pub fn progress(&mut self, progress: impl Into<ProgressProps>) {
        let props = progress.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.progress(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_progress(self.ui, &props);
    }

    /// Draws a table and reports which row, if any, was clicked.
    pub fn table(&mut self, table: impl Into<TableProps>) -> ItemClickResponse {
        let props = table.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.table(props));
        }
        self.sync_theme_mode();

        let table_seq = self.state.next_table_render_seq();
        let backend = &mut self.state.backend;
        let clicked = self
            .ui
            .push_id(("semantic_table", table_seq), |table_ui| {
                backend.render_table(table_ui, &props, table_seq)
            })
            .inner;

        dispatch_item_clicks(self.ui.ctx(), &clicked, |index| {
            props.rows.get(index).and_then(|row| row.on_click)
        })
    }

    /// Draws a close button and returns its `egui::Response`.
    pub fn close_button(&mut self, close_button: impl Into<CloseButtonProps>) -> egui::Response {
        let props = close_button.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.close_button(props));
        }
        self.sync_theme_mode();
        let response = self.state.backend.render_close_button(self.ui, &props);
        if response.clicked()
            && let Some(on_click) = props.on_click
        {
            on_click(self.ui.ctx());
        }
        response
    }

    /// Draws pagination and reports which page item, if any, was clicked.
    pub fn pagination(&mut self, pagination: impl Into<PaginationProps>) -> ItemClickResponse {
        let props = pagination.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.pagination(props));
        }
        self.sync_theme_mode();
        let clicked = self.state.backend.render_pagination(self.ui, &props);
        dispatch_item_clicks(self.ui.ctx(), &clicked, |index| {
            props.items.get(index).and_then(|item| item.on_click)
        })
    }

    pub fn spinner(&mut self, spinner: impl Into<SpinnerProps>) {
        let props = spinner.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.spinner(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_spinner(self.ui, &props);
    }

    pub fn placeholder(&mut self, placeholder: impl Into<PlaceholderProps>) {
        let props = placeholder.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.placeholder(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_placeholder(self.ui, &props);
    }

    /// Draws a button group and reports which button, if any, was clicked.
    pub fn button_group(&mut self, button_group: impl Into<ButtonGroupProps>) -> ItemClickResponse {
        let props = button_group.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.button_group(props));
        }
        self.sync_theme_mode();
        let clicked = self.state.backend.render_button_group(self.ui, &props);
        dispatch_item_clicks(self.ui.ctx(), &clicked, |index| {
            props.buttons.get(index).and_then(|button| button.on_click)
        })
    }

    /// Draws a list group and reports which item, if any, was clicked.
    pub fn list_group(&mut self, list_group: impl Into<ListGroupProps>) -> ItemClickResponse {
        let props = list_group.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.list_group(props));
        }
        self.sync_theme_mode();
        let clicked = self.state.backend.render_list_group(self.ui, &props);
        dispatch_item_clicks(self.ui.ctx(), &clicked, |index| {
            props.items.get(index).and_then(|item| item.on_click)
        })
    }

    /// Draws a nav and reports which item, if any, was clicked.
    pub fn nav(&mut self, nav: impl Into<NavProps>) -> ItemClickResponse {
        let props = nav.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.nav(props));
        }
        self.sync_theme_mode();
        let clicked = self.state.backend.render_nav(self.ui, &props);
        dispatch_item_clicks(self.ui.ctx(), &clicked, |index| {
            props.items.get(index).and_then(|item| item.on_click)
        })
    }

    /// Draws a navbar and reports item and brand clicks.
    pub fn navbar(&mut self, navbar: impl Into<NavbarProps>) -> NavbarResponse {
        let props = navbar.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.navbar(props));
        }
        self.sync_theme_mode();
        let result = self.state.backend.render_navbar(self.ui, &props);

        if result.brand_clicked
            && let Some(on_brand_click) = props.on_brand_click
        {
            on_brand_click(self.ui.ctx());
        }

        let items = dispatch_item_clicks(self.ui.ctx(), &result.clicked_items, |index| {
            props.items.get(index).and_then(|item| item.on_click)
        });
        NavbarResponse::new(items, result.brand_clicked)
    }

    /// Draws a dropdown and reports which menu item, if any, was clicked. Opening and closing
    /// the menu is handled internally and isn't reported as a click.
    pub fn dropdown<'s>(
        &mut self,
        state: impl Into<DisclosureState<'s>>,
        dropdown: impl Into<DropdownProps>,
    ) -> ItemClickResponse {
        let mut disclosure = state.into();
        let props = dropdown.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.dropdown(disclosure, props));
        }
        self.sync_theme_mode();

        let open = disclosure.current(self.ui.ctx());
        let result = self.state.backend.render_dropdown(self.ui, &props, open);

        if result.toggle_clicked {
            disclosure.toggle(self.ui.ctx());
        }

        if let Some(index) = result.clicked_item {
            disclosure.set_current(self.ui.ctx(), false);
            if let Some(on_click) = props.items.get(index).and_then(|item| item.on_click) {
                on_click(self.ui.ctx());
            }
        }
        ItemClickResponse::new(result.clicked_item)
    }

    pub fn form_label(&mut self, form_label: impl Into<FormLabelProps>) {
        let props = form_label.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.form_label(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_form_label(self.ui, &props);
    }

    pub fn form_text(&mut self, form_text: impl Into<FormTextProps>) {
        let props = form_text.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.form_text(props));
        }
        self.sync_theme_mode();
        self.state.backend.render_form_text(self.ui, &props);
    }

    pub fn form_control(
        &mut self,
        value: &mut String,
        form_control: impl Into<FormControlProps>,
    ) -> ChangeResponse {
        let props = form_control.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.form_control(value, props));
        }
        self.sync_theme_mode();
        let changed = self
            .state
            .backend
            .render_form_control(self.ui, value, &props);
        dispatch_change(self.ui.ctx(), changed, props.on_change)
    }

    pub fn form_select(
        &mut self,
        selected_index: &mut usize,
        form_select: impl Into<FormSelectProps>,
    ) -> ChangeResponse {
        let props = form_select.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.form_select(selected_index, props));
        }
        self.sync_theme_mode();
        let changed = self
            .state
            .backend
            .render_form_select(self.ui, selected_index, &props);
        dispatch_change(self.ui.ctx(), changed, props.on_change)
    }

    pub fn form_check(
        &mut self,
        checked: &mut bool,
        form_check: impl Into<FormCheckProps>,
    ) -> ChangeResponse {
        let props = form_check.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.form_check(checked, props));
        }
        self.sync_theme_mode();
        let changed = self
            .state
            .backend
            .render_form_check(self.ui, checked, &props);
        dispatch_change(self.ui.ctx(), changed, props.on_change)
    }

    pub fn form_range(
        &mut self,
        value: &mut f32,
        form_range: impl Into<FormRangeProps>,
    ) -> ChangeResponse {
        let props = form_range.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.form_range(value, props));
        }
        self.sync_theme_mode();
        let changed = self.state.backend.render_form_range(self.ui, value, &props);
        dispatch_change(self.ui.ctx(), changed, props.on_change)
    }

    pub fn floating_label(
        &mut self,
        value: &mut String,
        floating_label: impl Into<FloatingLabelProps>,
    ) -> ChangeResponse {
        let props = floating_label.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.floating_label(value, props));
        }
        self.sync_theme_mode();
        let changed = self
            .state
            .backend
            .render_floating_label(self.ui, value, &props);
        dispatch_change(self.ui.ctx(), changed, props.on_change)
    }

    pub fn validation_feedback(&mut self, validation_feedback: impl Into<ValidationFeedbackProps>) {
        let props = validation_feedback.into();
        if self.in_row() {
            return self.loose_child(|ui| ui.validation_feedback(props));
        }
        self.sync_theme_mode();
        self.state
            .backend
            .render_validation_feedback(self.ui, &props);
    }
}

fn container_body<R>(
    ui: &mut egui::Ui,
    state: &mut RootState,
    props: ContainerProps,
    f: impl FnOnce(&mut UiRoot<'_>) -> R,
) -> R {
    let available_width = ui.available_width();
    let container_width = state
        .backend
        .resolve_container_width(&props, available_width);

    let horizontal_padding = props.padding.gutter_x();
    let vertical_padding = props.padding.gutter_y();

    ui.horizontal_top(|row_ui| {
        if !props.fluid {
            let leading_space = ((available_width - container_width) * 0.5).max(0.0);
            if leading_space > 0.0 {
                row_ui.add_space(leading_space);
            }
        }

        row_ui
            .allocate_ui_with_layout(
                egui::vec2(container_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |container_ui| {
                    if vertical_padding > 0.0 {
                        container_ui.add_space(vertical_padding);
                    }

                    let inner = container_ui
                        .horizontal_top(|padding_ui| {
                            if horizontal_padding > 0.0 {
                                padding_ui.add_space(horizontal_padding);
                            }

                            let content_width =
                                (container_width - horizontal_padding * 2.0).max(0.0);
                            let inner = padding_ui
                                .allocate_ui_with_layout(
                                    egui::vec2(content_width, 0.0),
                                    egui::Layout::top_down(egui::Align::Min),
                                    |content_ui| f(&mut UiRoot::new(content_ui, state)),
                                )
                                .inner;

                            if horizontal_padding > 0.0 {
                                padding_ui.add_space(horizontal_padding);
                            }

                            inner
                        })
                        .inner;

                    if vertical_padding > 0.0 {
                        container_ui.add_space(vertical_padding);
                    }

                    inner
                },
            )
            .inner
    })
    .inner
}

/// Fires each clicked item's `on_click` handler and builds the collection's response.
fn dispatch_item_clicks(
    ctx: &egui::Context,
    clicked: &[usize],
    handler_for: impl Fn(usize) -> Option<ClickHandler>,
) -> ItemClickResponse {
    for &index in clicked {
        if let Some(on_click) = handler_for(index) {
            on_click(ctx);
        }
    }
    ItemClickResponse::from_clicked(clicked)
}

/// Fires `on_change` if the value changed and builds the input's response.
fn dispatch_change(
    ctx: &egui::Context,
    changed: bool,
    on_change: Option<ClickHandler>,
) -> ChangeResponse {
    if changed && let Some(on_change) = on_change {
        on_change(ctx);
    }
    ChangeResponse::new(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Col, Row};

    /// Runs one egui pass and builds `f` into a fresh root, returning the last pass's value.
    fn run_frame<R>(ctx: &egui::Context, mut f: impl FnMut(&mut UiRoot<'_>) -> R) -> R {
        let mut result = None;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            result = Some(crate::show_root(ctx, &mut f));
        });
        result.expect("egui pass should run the build closure")
    }

    #[test]
    fn state_scopes_render_without_panicking() {
        let ctx = egui::Context::default();
        let mut open = true;
        let mut active_index = 0_usize;

        run_frame(&ctx, |ui| {
            ui.with_disclosure_state(&mut open, |ui| {
                ui.with_active_index_state(&mut active_index, |_ui| {});
            });
        });
    }

    #[test]
    fn closed_disclosure_skips_its_body() {
        let ctx = egui::Context::default();
        let mut open = false;

        let (closed, opened) = run_frame(&ctx, |ui| {
            let closed = ui.with_disclosure_state(&mut open, |_ui| "closed body");
            let opened = ui.with_disclosure_state(
                DisclosureState::uncontrolled("bee-ui.test.disclosure.open", true),
                |_ui| "open body",
            );
            (closed, opened)
        });

        assert_eq!(closed, None);
        assert_eq!(opened, Some("open body"));
    }

    #[test]
    fn scopes_return_their_closure_values() {
        let ctx = egui::Context::default();

        let values = run_frame(&ctx, |ui| {
            let container = ui.container(ContainerProps::fluid(), |ui| {
                ui.card(CardProps::new(), |ui| ui.col(Col::new(), |_ui| 1))
            });
            let stack = ui.hstack(HStack::new().gap(4.0), |ui| ui.raw(|_egui_ui| 2));
            let styled = ui.with_style_override(|_style| {}, |_ui| 3);
            let themed = ui.with_theme_variant(egui::Theme::Light, |_ui| 4);
            (container, stack, styled, themed)
        });

        assert_eq!(values, (1, 2, 3, 4));
    }

    #[test]
    fn standalone_col_fills_available_width() {
        let ctx = egui::Context::default();

        let (available, col_width) = run_frame(&ctx, |ui| {
            let available = ui.raw(|egui_ui| egui_ui.available_width());
            let col_width = ui.col(Col::new(), |ui| ui.raw(|egui_ui| egui_ui.available_width()));
            (available, col_width)
        });

        assert!(available > 0.0);
        assert_eq!(col_width, available);
    }

    #[test]
    fn strict_mode_reports_invalid_col_props() {
        let ctx = egui::Context::default();

        let errors = run_frame(&ctx, |ui| {
            ui.set_strict_mode(true);
            ui.col(Col::new().xs(Span::N(0)).offset(13).order(99), |_ui| {});
            ui.take_strict_mode_errors()
        });

        assert_eq!(errors.len(), 3);
        assert!(
            errors
                .iter()
                .any(|error| error.to_string().contains("ColProps.xs")
                    && error.to_string().contains("value `0`"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.to_string().contains("ColProps.offset")
                    && error.to_string().contains("value `13`"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.to_string().contains("ColProps.order")
                    && error.to_string().contains("value `99`"))
        );
    }

    #[test]
    fn strict_mode_can_publish_errors_to_optional_reporter() {
        use std::{cell::RefCell, rc::Rc};

        let reported = Rc::new(RefCell::new(Vec::<String>::new()));
        let ctx = egui::Context::default();

        run_frame(&ctx, |ui| {
            reported.borrow_mut().clear();
            ui.set_strict_mode(true);

            let sink = Rc::clone(&reported);
            ui.set_strict_mode_error_reporter(move |error| {
                sink.borrow_mut().push(error.to_string())
            });

            ui.col(Col::new().xs(Span::N(0)), |_ui| {});
        });

        assert_eq!(reported.borrow().len(), 1);
        assert!(reported.borrow()[0].contains("ColProps.xs"));
    }

    const TOLERANCE: f32 = 0.5;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "expected {expected}, got {actual}"
        );
    }

    /// Runs one frame on a fixed-size screen, returning the last pass's value and how many
    /// passes egui ran (more than one means a pass was discarded).
    fn run_counted<R>(ctx: &egui::Context, f: impl FnMut(&mut UiRoot<'_>) -> R) -> (R, usize) {
        run_with_events(ctx, Vec::new(), f)
    }

    /// Like [`run_counted`], but feeds `events` into the frame's input.
    fn run_with_events<R>(
        ctx: &egui::Context,
        events: Vec<egui::Event>,
        mut f: impl FnMut(&mut UiRoot<'_>) -> R,
    ) -> (R, usize) {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 800.0),
            )),
            events,
            ..Default::default()
        };
        let mut passes = 0;
        let mut result = None;
        let _ = ctx.run(input, |ctx| {
            passes += 1;
            result = Some(crate::show_root(ctx, &mut f));
        });
        (
            result.expect("egui pass should run the build closure"),
            passes,
        )
    }

    /// The rectangle a column body may draw into.
    fn col_rect(ui: &mut UiRoot<'_>, col: Col) -> egui::Rect {
        ui.col(col, |ui| ui.raw(|egui_ui| egui_ui.max_rect()))
    }

    fn two_or_three_cols(ui: &mut UiRoot<'_>, three: bool) {
        ui.row(Row::new(), |ui| {
            ui.col(Col::new(), |_ui| {});
            ui.col(Col::new(), |_ui| {});
            if three {
                ui.col(Col::new(), |_ui| {});
            }
        });
    }

    #[test]
    fn row_first_appearance_requests_a_discard_and_stable_rows_do_not() {
        let ctx = egui::Context::default();

        let ((), passes) = run_counted(&ctx, |ui| two_or_three_cols(ui, false));
        assert_eq!(passes, 2, "first appearance should re-run the pass");

        let ((), passes) = run_counted(&ctx, |ui| two_or_three_cols(ui, false));
        assert_eq!(passes, 1, "a stable row should not discard");
    }

    #[test]
    fn adding_a_column_requests_exactly_one_discard() {
        let ctx = egui::Context::default();
        run_counted(&ctx, |ui| two_or_three_cols(ui, false));
        run_counted(&ctx, |ui| two_or_three_cols(ui, false));

        let ((), passes) = run_counted(&ctx, |ui| two_or_three_cols(ui, true));
        assert_eq!(passes, 2);

        let ((), passes) = run_counted(&ctx, |ui| two_or_three_cols(ui, true));
        assert_eq!(passes, 1);
    }

    #[test]
    fn empty_row_does_not_discard_or_allocate() {
        let ctx = egui::Context::default();

        let ((before, after), passes) = run_counted(&ctx, |ui| {
            let before = ui.raw(|egui_ui| egui_ui.cursor().top());
            ui.row(Row::new(), |_ui| {});
            let after = ui.raw(|egui_ui| egui_ui.cursor().top());
            (before, after)
        });

        assert_eq!(passes, 1);
        assert_eq!(before, after);
    }

    #[test]
    fn auto_columns_split_the_remaining_width_equally() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let row_left = ui.raw(|egui_ui| egui_ui.available_rect_before_wrap().left());
            let width = ui.raw(|egui_ui| egui_ui.available_width());
            let rects = ui.row(Row::new().gutter_x(10.0), |ui| {
                [
                    col_rect(ui, Col::new().xs(6)),
                    col_rect(ui, Col::new()),
                    col_rect(ui, Col::new()),
                ]
            });
            (row_left, width, rects)
        };

        run_counted(&ctx, build);
        let ((row_left, width, [fixed, auto_a, auto_b]), passes) = run_counted(&ctx, build);
        assert_eq!(passes, 1);

        let unit = (width - 2.0 * 10.0) / 12.0;
        assert_close(fixed.width(), 6.0 * unit);
        assert_close(auto_a.width(), 3.0 * unit);
        assert_close(auto_b.width(), 3.0 * unit);

        assert_close(fixed.left(), row_left);
        assert_close(auto_a.left(), fixed.right() + 10.0);
        assert_close(auto_b.left(), auto_a.right() + 10.0);
        assert_close(fixed.top(), auto_a.top());
        assert_close(fixed.top(), auto_b.top());
    }

    #[test]
    fn row_wraps_each_loose_child_in_its_own_col() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let width = ui.raw(|egui_ui| egui_ui.available_width());
            let (first, second) = ui.row(Row::new(), |ui| {
                let first = ui.raw(|egui_ui| egui_ui.max_rect());
                ui.label(LabelProps::new("loose label"));
                let second = ui.raw(|egui_ui| egui_ui.max_rect());
                (first, second)
            });
            (width, first, second)
        };

        run_counted(&ctx, build);
        let ((width, first, second), _) = run_counted(&ctx, build);

        assert_close(first.width(), width / 3.0);
        assert_close(second.width(), width / 3.0);
        assert_close(second.left() - first.left(), 2.0 * width / 3.0);
    }

    #[test]
    fn hstack_keeps_children_together_inside_a_row() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let width = ui.raw(|egui_ui| egui_ui.available_width());
            let (stack, raw) = ui.row(Row::new(), |ui| {
                let stack = ui.hstack(HStack::new(), |ui| {
                    ui.label(LabelProps::new("a"));
                    ui.label(LabelProps::new("b"));
                    ui.raw(|egui_ui| egui_ui.max_rect())
                });
                let raw = ui.raw(|egui_ui| egui_ui.max_rect());
                (stack, raw)
            });
            (width, stack, raw)
        };

        run_counted(&ctx, build);
        let ((width, stack, raw), _) = run_counted(&ctx, build);

        assert_close(raw.width(), width / 2.0);
        assert!(stack.right() <= raw.left() + TOLERANCE);
    }

    #[test]
    fn wrapped_lines_start_below_the_tallest_column_plus_gutter() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let item_spacing_y = ui.raw(|egui_ui| egui_ui.spacing().item_spacing.y);
            let (short_bottom, tall_bottom, wrapped) = ui.row(Row::new().gutter_y(8.0), |ui| {
                let short_bottom = ui.col(Col::new().xs(6), |ui| {
                    ui.raw(|egui_ui| {
                        egui_ui.allocate_space(egui::vec2(10.0, 20.0));
                        egui_ui.min_rect().bottom()
                    })
                });
                let tall_bottom = ui.col(Col::new().xs(6), |ui| {
                    ui.raw(|egui_ui| {
                        egui_ui.allocate_space(egui::vec2(10.0, 50.0));
                        egui_ui.min_rect().bottom()
                    })
                });
                let wrapped = col_rect(ui, Col::new().xs(12));
                (short_bottom, tall_bottom, wrapped)
            });
            let after_row = ui.raw(|egui_ui| egui_ui.cursor().top());
            (
                item_spacing_y,
                short_bottom,
                tall_bottom,
                wrapped,
                after_row,
            )
        };

        run_counted(&ctx, build);
        let ((item_spacing_y, short_bottom, tall_bottom, wrapped, after_row), passes) =
            run_counted(&ctx, build);

        assert_eq!(passes, 1);
        assert!(short_bottom < tall_bottom);
        assert_close(wrapped.top(), tall_bottom + item_spacing_y + 8.0);
        // The empty wrapped column still claims its line, and the row's height is allocated
        // in the parent so later widgets start below it.
        assert!(after_row >= wrapped.top());
    }

    #[test]
    fn row_allocates_its_height_in_the_parent() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let content_bottom = ui.row(Row::new(), |ui| {
                ui.col(Col::new(), |ui| {
                    ui.raw(|egui_ui| {
                        egui_ui.allocate_space(egui::vec2(10.0, 40.0));
                        egui_ui.min_rect().bottom()
                    })
                })
            });
            let next_top = ui.raw(|egui_ui| egui_ui.cursor().top());
            (content_bottom, next_top)
        };

        run_counted(&ctx, build);
        let ((content_bottom, next_top), _) = run_counted(&ctx, build);
        assert!(next_top >= content_bottom);
    }

    #[test]
    fn nested_rows_settle_in_the_first_frame() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            ui.row(Row::new(), |ui| {
                ui.col(Col::new().xs(6), |ui| {
                    ui.row(Row::new(), |ui| {
                        ui.col(Col::new(), |_ui| {});
                        ui.col(Col::new(), |_ui| {});
                    });
                });
                ui.col(Col::new().xs(6), |_ui| {});
            });
        };

        let ((), passes) = run_counted(&ctx, build);
        assert_eq!(passes, 2);
        let ((), passes) = run_counted(&ctx, build);
        assert_eq!(passes, 1);
    }

    #[test]
    fn id_salt_keeps_the_cached_layout_when_a_row_moves() {
        fn build(ui: &mut UiRoot<'_>, with_heading: bool, row: Row) {
            if with_heading {
                ui.label(LabelProps::new("heading"));
            }
            ui.row(row, |ui| {
                ui.col(Col::new(), |_ui| {});
                ui.col(Col::new(), |_ui| {});
            });
        }

        let salted = egui::Context::default();
        run_counted(&salted, |ui| {
            build(ui, false, Row::new().id_salt("metrics"))
        });
        let ((), passes) =
            run_counted(&salted, |ui| build(ui, true, Row::new().id_salt("metrics")));
        assert_eq!(
            passes, 1,
            "a salted row keeps its layout when siblings change"
        );

        let unsalted = egui::Context::default();
        run_counted(&unsalted, |ui| build(ui, false, Row::new()));
        let ((), passes) = run_counted(&unsalted, |ui| build(ui, true, Row::new()));
        assert_eq!(
            passes, 2,
            "an unsalted row re-measures when its position changes"
        );
    }

    /// Pointer events for a primary-button press and release at `pos` within one frame.
    fn click_at(pos: egui::Pos2) -> Vec<egui::Event> {
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        vec![egui::Event::PointerMoved(pos), button(true), button(false)]
    }

    #[test]
    fn button_reports_clicked_on_the_click_frame_only() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static ON_CLICK_CALLS: AtomicUsize = AtomicUsize::new(0);

        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let response = ui.button(Button::primary("Deploy").on_click(|_ctx| {
                ON_CLICK_CALLS.fetch_add(1, Ordering::SeqCst);
            }));
            // Read the counter in the same pass to show `on_click` isn't deferred.
            (
                response.rect,
                response.clicked(),
                ON_CLICK_CALLS.load(Ordering::SeqCst),
            )
        };

        let ((rect, clicked, calls), _) = run_counted(&ctx, build);
        assert!(!clicked);
        assert_eq!(calls, 0);
        assert!(rect.is_positive());

        let ((_, clicked, calls), passes) = run_with_events(&ctx, click_at(rect.center()), build);
        assert_eq!(passes, 1);
        assert!(clicked, "the click should be reported in the same pass");
        assert_eq!(calls, 1, "`on_click` should fire in the same pass");

        let ((_, clicked, calls), _) = run_counted(&ctx, build);
        assert!(
            !clicked,
            "the click should not carry over to the next frame"
        );
        assert_eq!(calls, 1);
    }

    #[test]
    fn clicking_outside_a_button_does_not_report_a_click() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            let response = ui.button(Button::primary("Deploy"));
            (response.rect, response.clicked())
        };

        let ((rect, _), _) = run_counted(&ctx, build);
        let outside = rect.right_bottom() + egui::vec2(50.0, 50.0);
        let ((_, clicked), _) = run_with_events(&ctx, click_at(outside), build);
        assert!(!clicked);
    }

    #[test]
    fn button_inside_a_row_reports_clicks() {
        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            ui.row(Row::new(), |ui| {
                ui.col(Col::new(), |_ui| {});
                ui.col(Col::new(), |ui| {
                    let response = ui.button(Button::primary("Deploy"));
                    (response.rect, response.clicked())
                })
            })
        };

        // The first frame measures the row and discards; the second is stable.
        run_counted(&ctx, build);
        let ((rect, clicked), passes) = run_counted(&ctx, build);
        assert_eq!(passes, 1);
        assert!(!clicked);

        let ((_, clicked), passes) = run_with_events(&ctx, click_at(rect.center()), build);
        assert_eq!(passes, 1);
        assert!(clicked);

        let ((_, clicked), _) = run_counted(&ctx, build);
        assert!(!clicked);
    }

    #[test]
    fn list_group_reports_the_clicked_index_on_the_click_frame_only() {
        const ITEMS: usize = 3;

        let ctx = egui::Context::default();
        let build = |ui: &mut UiRoot<'_>| {
            // A standalone col's `min_rect` is exactly the list's bounds.
            ui.col(Col::new(), |ui| {
                let response = ui.list_group(
                    ListGroupProps::new()
                        .flush(true)
                        .item("Overview")
                        .item("Jobs")
                        .item("Alerts"),
                );
                (ui.raw(|egui_ui| egui_ui.min_rect()), response)
            })
        };

        let ((list_rect, response), _) = run_counted(&ctx, build);
        assert!(!response.clicked());
        assert!(list_rect.is_positive());

        // Flush list items have no spacing between them, so each takes an equal share.
        let item_height = list_rect.height() / ITEMS as f32;
        let second_item = egui::pos2(list_rect.left() + 10.0, list_rect.top() + 1.5 * item_height);

        let ((_, response), passes) = run_with_events(&ctx, click_at(second_item), build);
        assert_eq!(passes, 1);
        assert!(response.clicked());
        assert_eq!(response.clicked_index(), Some(1));
        assert!(response.item_clicked(1));
        assert!(!response.item_clicked(0));

        let ((_, response), _) = run_counted(&ctx, build);
        assert!(!response.clicked());
        assert_eq!(response.clicked_index(), None);
    }
}
