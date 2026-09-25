use std::fmt;

use crate::{
    components::{
        ActiveIndexState, AlertProps, BadgeProps, Button, ButtonGroupProps, CardProps,
        CloseButtonProps, ColProps, ColValidationIssue, ContainerProps, ContainerScroll,
        DisclosureState, DropdownProps, FloatingLabelProps, FormCheckProps, FormControlProps,
        FormLabelProps, FormRangeProps, FormSelectProps, FormTextProps, LabelProps, ListGroupProps,
        NavProps, NavbarProps, PaginationProps, PlaceholderProps, ProgressProps, RowProps,
        SpinnerProps, TableProps, TextProps, ValidationFeedbackProps,
    },
    layout::{
        ComputedCol, ComputedLine, GridColSpec, Span, compute_grid_lines, resolve_active_breakpoint,
    },
    plot::{ChartCard, PlotArea},
    render::{Backend, EguiBackend},
};

type PlotBody<'a> = Box<dyn for<'plot_ui> FnOnce(&mut egui_plot::PlotUi<'plot_ui>) + 'a>;
type RawEguiBody<'a> = Box<dyn FnOnce(&mut egui::Ui) + 'a>;
type StyleOverride<'a> = Box<dyn FnOnce(&mut egui::Style) + 'a>;
type StrictErrorReporter<'a> = Box<dyn FnMut(&UiValidationError) + 'a>;

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

pub struct UiRoot<'a> {
    ctx: egui::Context,
    backend: EguiBackend,
    scopes: Vec<BuildScope<'a>>,
    strict_mode: bool,
    strict_mode_errors: Vec<UiValidationError>,
    strict_mode_error_reporter: Option<StrictErrorReporter<'a>>,
    table_render_seq: u64,
}

impl<'a> UiRoot<'a> {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            ctx: ctx.clone(),
            backend: EguiBackend::default(),
            scopes: vec![BuildScope::Root(Vec::new())],
            strict_mode: false,
            strict_mode_errors: Vec::new(),
            strict_mode_error_reporter: None,
            table_render_seq: 0,
        }
    }

    pub fn context(&self) -> &egui::Context {
        &self.ctx
    }

    pub fn set_strict_mode(&mut self, enabled: bool) -> &mut Self {
        self.strict_mode = enabled;
        self
    }

    pub const fn strict_mode_enabled(&self) -> bool {
        self.strict_mode
    }

    pub fn set_strict_mode_error_reporter(
        &mut self,
        reporter: impl FnMut(&UiValidationError) + 'a,
    ) -> &mut Self {
        self.strict_mode_error_reporter = Some(Box::new(reporter));
        self
    }

    pub fn clear_strict_mode_error_reporter(&mut self) -> &mut Self {
        self.strict_mode_error_reporter = None;
        self
    }

    pub fn strict_mode_errors(&self) -> &[UiValidationError] {
        &self.strict_mode_errors
    }

    pub fn take_strict_mode_errors(&mut self) -> Vec<UiValidationError> {
        std::mem::take(&mut self.strict_mode_errors)
    }

    /// Adds a stateful disclosure scope (open/closed) to the semantic tree.
    ///
    /// Controlled state (`&mut bool`) is preferred so callers keep visibility explicit.
    pub fn with_disclosure_state(
        &mut self,
        state: impl Into<DisclosureState<'a>>,
        f: impl FnOnce(&mut UiRoot<'a>),
    ) {
        self.scopes.push(BuildScope::Disclosure {
            state: state.into(),
            children: Vec::new(),
        });
        f(self);

        let Some(BuildScope::Disclosure { state, children }) = self.scopes.pop() else {
            debug_assert!(false, "disclosure scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::Disclosure(DisclosureScopeNode {
            state,
            children,
        }));
    }

    /// Adds a stateful active-index scope to the semantic tree.
    ///
    /// Controlled state (`&mut usize`) is preferred so callers keep selection explicit.
    pub fn with_active_index_state(
        &mut self,
        state: impl Into<ActiveIndexState<'a>>,
        f: impl FnOnce(&mut UiRoot<'a>),
    ) {
        self.scopes.push(BuildScope::ActiveIndex {
            state: state.into(),
            children: Vec::new(),
        });
        f(self);

        let Some(BuildScope::ActiveIndex { state, children }) = self.scopes.pop() else {
            debug_assert!(false, "active-index scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::ActiveIndex(ActiveIndexScopeNode {
            state,
            children,
        }));
    }

    pub(crate) fn interop_raw(&mut self, f: impl FnOnce(&mut egui::Ui) + 'a) {
        self.append_node(SemanticNode::RawEgui(RawEguiNode {
            body: Some(Box::new(f)),
        }));
    }

    pub(crate) fn interop_with_style_override(
        &mut self,
        style_override: impl FnOnce(&mut egui::Style) + 'a,
        f: impl FnOnce(&mut UiRoot<'a>),
    ) {
        self.scopes.push(BuildScope::StyleOverride {
            style_override: Some(Box::new(style_override)),
            children: Vec::new(),
        });
        f(self);

        let Some(BuildScope::StyleOverride {
            style_override,
            children,
        }) = self.scopes.pop()
        else {
            debug_assert!(false, "style-override scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::StyleOverride(StyleOverrideNode {
            style_override,
            children,
        }));
    }

    pub(crate) fn interop_with_theme_variant(
        &mut self,
        theme: egui::Theme,
        f: impl FnOnce(&mut UiRoot<'a>),
    ) {
        self.scopes.push(BuildScope::ThemeVariant {
            theme,
            children: Vec::new(),
        });
        f(self);

        let Some(BuildScope::ThemeVariant { theme, children }) = self.scopes.pop() else {
            debug_assert!(false, "theme-variant scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::ThemeVariant(ThemeVariantNode {
            theme,
            children,
        }));
    }

    pub(crate) fn render_with_central_panel(&mut self) {
        self.render_with_central_panel_frame(egui::Frame::central_panel(&self.ctx.style()));
    }

    pub(crate) fn render_with_central_panel_frame(&mut self, frame: egui::Frame) {
        let mut nodes = self.take_root_nodes();
        self.table_render_seq = 0;

        // `CentralPanel` requires an active egui pass. Compile-focused tests may call
        // `show_root` with a fresh `egui::Context` that has not entered `Context::run` yet.
        if self.ctx.cumulative_frame_nr() == 0 {
            return;
        }

        let ctx = self.ctx.clone();
        egui::CentralPanel::default().frame(frame).show(&ctx, |ui| {
            self.render_nodes(ui, &mut nodes);
        });
    }

    pub fn container(
        &mut self,
        container: impl Into<ContainerProps>,
        f: impl FnOnce(&mut UiRoot<'a>),
    ) {
        let props = container.into();
        self.scopes.push(BuildScope::Container {
            props,
            children: Vec::new(),
        });
        f(self);

        let Some(BuildScope::Container { props, children }) = self.scopes.pop() else {
            debug_assert!(false, "container scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::Container(ContainerNode { props, children }));
    }

    pub fn row(&mut self, row: impl Into<RowProps>, f: impl FnOnce(&mut UiRoot<'a>)) {
        let props = row.into();
        self.scopes.push(BuildScope::Row {
            props,
            cols: Vec::new(),
        });
        f(self);

        let Some(BuildScope::Row { props, cols }) = self.scopes.pop() else {
            debug_assert!(false, "row scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::Row(RowNode { props, cols }));
    }

    pub fn col(&mut self, col: impl Into<ColProps>, f: impl FnOnce(&mut UiRoot<'a>)) {
        let props = col.into();
        self.report_col_validation_issues(&props);

        self.scopes.push(BuildScope::Col {
            props,
            children: Vec::new(),
            implicit: false,
        });
        f(self);

        let Some(BuildScope::Col {
            props,
            children,
            implicit,
        }) = self.scopes.pop()
        else {
            debug_assert!(false, "col scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::Col(ColNode {
            props,
            children,
            implicit,
        }));
    }

    pub fn card(&mut self, card: impl Into<CardProps>, f: impl FnOnce(&mut UiRoot<'a>)) {
        let props = card.into();
        self.scopes.push(BuildScope::Card {
            props,
            children: Vec::new(),
        });
        f(self);

        let Some(BuildScope::Card { props, children }) = self.scopes.pop() else {
            debug_assert!(false, "card scope stack was unexpectedly empty");
            return;
        };

        self.append_node(SemanticNode::Card(CardNode { props, children }));
    }

    pub fn chart_card<F>(&mut self, chart: impl Into<ChartCard>, f: F)
    where
        F: for<'plot_ui> FnOnce(&mut egui_plot::PlotUi<'plot_ui>) + 'a,
    {
        let chart = chart.into();
        self.card(chart.card, move |ui| {
            ui.plot(chart.plot, f);
        });
    }

    pub fn plot<F>(&mut self, plot: impl Into<PlotArea>, f: F)
    where
        F: for<'plot_ui> FnOnce(&mut egui_plot::PlotUi<'plot_ui>) + 'a,
    {
        let props = plot.into();
        self.append_node(SemanticNode::Plot(PlotNode {
            props,
            body: Some(Box::new(f)),
        }));
    }

    pub fn badge(&mut self, badge: impl Into<BadgeProps>) {
        let props = badge.into();
        self.append_node(SemanticNode::Badge(props));
    }

    pub fn label(&mut self, label: impl Into<LabelProps>) {
        let props = label.into();
        self.append_node(SemanticNode::Label(props));
    }

    pub fn text(&mut self, text: impl Into<TextProps>) {
        let props = text.into();
        self.append_node(SemanticNode::Text(props));
    }

    pub fn button(&mut self, button: Button) {
        self.append_node(SemanticNode::Button(button));
    }

    pub fn alert(&mut self, alert: impl Into<AlertProps>) {
        let props = alert.into();
        self.append_node(SemanticNode::Alert(props));
    }

    pub fn progress(&mut self, progress: impl Into<ProgressProps>) {
        let props = progress.into();
        self.append_node(SemanticNode::Progress(props));
    }

    pub fn table(&mut self, table: impl Into<TableProps>) {
        let props = table.into();
        self.append_node(SemanticNode::Table(props));
    }

    pub fn close_button(&mut self, close_button: impl Into<CloseButtonProps>) {
        let props = close_button.into();
        self.append_node(SemanticNode::CloseButton(props));
    }

    pub fn pagination(&mut self, pagination: impl Into<PaginationProps>) {
        let props = pagination.into();
        self.append_node(SemanticNode::Pagination(props));
    }

    pub fn spinner(&mut self, spinner: impl Into<SpinnerProps>) {
        let props = spinner.into();
        self.append_node(SemanticNode::Spinner(props));
    }

    pub fn placeholder(&mut self, placeholder: impl Into<PlaceholderProps>) {
        let props = placeholder.into();
        self.append_node(SemanticNode::Placeholder(props));
    }

    pub fn button_group(&mut self, button_group: impl Into<ButtonGroupProps>) {
        let props = button_group.into();
        self.append_node(SemanticNode::ButtonGroup(props));
    }

    pub fn list_group(&mut self, list_group: impl Into<ListGroupProps>) {
        let props = list_group.into();
        self.append_node(SemanticNode::ListGroup(props));
    }

    pub fn nav(&mut self, nav: impl Into<NavProps>) {
        let props = nav.into();
        self.append_node(SemanticNode::Nav(props));
    }

    pub fn navbar(&mut self, navbar: impl Into<NavbarProps>) {
        let props = navbar.into();
        self.append_node(SemanticNode::Navbar(props));
    }

    pub fn dropdown(
        &mut self,
        state: impl Into<DisclosureState<'a>>,
        dropdown: impl Into<DropdownProps>,
    ) {
        self.append_node(SemanticNode::Dropdown(DropdownNode {
            props: dropdown.into(),
            state: state.into(),
        }));
    }

    pub fn form_label(&mut self, form_label: impl Into<FormLabelProps>) {
        let props = form_label.into();
        self.append_node(SemanticNode::FormLabel(props));
    }

    pub fn form_text(&mut self, form_text: impl Into<FormTextProps>) {
        let props = form_text.into();
        self.append_node(SemanticNode::FormText(props));
    }

    pub fn form_control(
        &mut self,
        value: &'a mut String,
        form_control: impl Into<FormControlProps>,
    ) {
        self.append_node(SemanticNode::FormControl(FormControlNode {
            value,
            props: form_control.into(),
        }));
    }

    pub fn form_select(
        &mut self,
        selected_index: &'a mut usize,
        form_select: impl Into<FormSelectProps>,
    ) {
        self.append_node(SemanticNode::FormSelect(FormSelectNode {
            selected_index,
            props: form_select.into(),
        }));
    }

    pub fn form_check(&mut self, checked: &'a mut bool, form_check: impl Into<FormCheckProps>) {
        self.append_node(SemanticNode::FormCheck(FormCheckNode {
            checked,
            props: form_check.into(),
        }));
    }

    pub fn form_range(&mut self, value: &'a mut f32, form_range: impl Into<FormRangeProps>) {
        self.append_node(SemanticNode::FormRange(FormRangeNode {
            value,
            props: form_range.into(),
        }));
    }

    pub fn floating_label(
        &mut self,
        value: &'a mut String,
        floating_label: impl Into<FloatingLabelProps>,
    ) {
        self.append_node(SemanticNode::FloatingLabel(FloatingLabelNode {
            value,
            props: floating_label.into(),
        }));
    }

    pub fn validation_feedback(&mut self, validation_feedback: impl Into<ValidationFeedbackProps>) {
        let props = validation_feedback.into();
        self.append_node(SemanticNode::ValidationFeedback(props));
    }

    fn append_node(&mut self, node: SemanticNode<'a>) {
        let Some(scope) = self.scopes.last_mut() else {
            debug_assert!(false, "ui scope stack was unexpectedly empty");
            return;
        };

        match scope {
            BuildScope::Root(children)
            | BuildScope::Container { children, .. }
            | BuildScope::Col { children, .. }
            | BuildScope::Card { children, .. }
            | BuildScope::Disclosure { children, .. }
            | BuildScope::ActiveIndex { children, .. }
            | BuildScope::StyleOverride { children, .. }
            | BuildScope::ThemeVariant { children, .. } => children.push(node),
            BuildScope::Row { cols, .. } => match node {
                SemanticNode::Col(col) => cols.push(col),
                node => {
                    if let Some(last_col) = cols.last_mut()
                        && last_col.implicit
                    {
                        last_col.children.push(node);
                        return;
                    }

                    cols.push(ColNode {
                        props: ColProps::default(),
                        children: vec![node],
                        implicit: true,
                    });
                }
            },
        }
    }

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

    fn take_root_nodes(&mut self) -> Vec<SemanticNode<'a>> {
        debug_assert_eq!(self.scopes.len(), 1, "unbalanced semantic scope stack");

        let Some(BuildScope::Root(nodes)) = self.scopes.first_mut() else {
            debug_assert!(false, "root scope was unexpectedly unavailable");
            return Vec::new();
        };

        std::mem::take(nodes)
    }

    fn next_table_render_seq(&mut self) -> u64 {
        let seq = self.table_render_seq;
        self.table_render_seq = self.table_render_seq.wrapping_add(1);
        seq
    }

    fn render_nodes(&mut self, ui: &mut egui::Ui, nodes: &mut [SemanticNode<'a>]) {
        for node in nodes.iter_mut() {
            self.backend.sync_mode_from_ui(ui);
            match node {
                SemanticNode::Container(container) => self.render_container(ui, container),
                SemanticNode::Row(row) => self.render_row(ui, row),
                SemanticNode::Col(col) => self.render_standalone_col(ui, col),
                SemanticNode::Card(card) => self.render_card(ui, card),
                SemanticNode::Plot(plot) => self.render_plot(ui, plot),
                SemanticNode::Disclosure(scope) => self.render_disclosure_scope(ui, scope),
                SemanticNode::ActiveIndex(scope) => self.render_active_index_scope(ui, scope),
                SemanticNode::RawEgui(raw) => self.render_raw_egui(ui, raw),
                SemanticNode::StyleOverride(scope) => self.render_style_override_scope(ui, scope),
                SemanticNode::ThemeVariant(scope) => self.render_theme_variant_scope(ui, scope),
                SemanticNode::Badge(badge) => self.backend.render_badge(ui, badge),
                SemanticNode::Label(label) => self.backend.render_label(ui, label),
                SemanticNode::Text(text) => self.backend.render_text(ui, text),
                SemanticNode::Button(button) => {
                    let response = self.backend.render_button(ui, button);
                    if response.clicked()
                        && let Some(on_click) = button.on_click
                    {
                        on_click(ui.ctx());
                    }
                }
                SemanticNode::Alert(alert) => self.backend.render_alert(ui, alert),
                SemanticNode::Progress(progress) => self.backend.render_progress(ui, progress),
                SemanticNode::Table(table) => {
                    let table_seq = self.next_table_render_seq();
                    let clicked = ui
                        .push_id(("semantic_table", table_seq), |table_ui| {
                            self.backend.render_table(table_ui, table, table_seq)
                        })
                        .inner;
                    for index in clicked {
                        if let Some(on_click) = table.rows.get(index).and_then(|row| row.on_click) {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::CloseButton(close_button) => {
                    let response = self.backend.render_close_button(ui, close_button);
                    if response.clicked()
                        && let Some(on_click) = close_button.on_click
                    {
                        on_click(ui.ctx());
                    }
                }
                SemanticNode::Pagination(pagination) => {
                    let clicked = self.backend.render_pagination(ui, pagination);
                    for index in clicked {
                        if let Some(on_click) =
                            pagination.items.get(index).and_then(|item| item.on_click)
                        {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::Spinner(spinner) => self.backend.render_spinner(ui, spinner),
                SemanticNode::Placeholder(placeholder) => {
                    self.backend.render_placeholder(ui, placeholder)
                }
                SemanticNode::ButtonGroup(button_group) => {
                    let clicked = self.backend.render_button_group(ui, button_group);
                    for index in clicked {
                        if let Some(on_click) = button_group
                            .buttons
                            .get(index)
                            .and_then(|button| button.on_click)
                        {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::ListGroup(list_group) => {
                    let clicked = self.backend.render_list_group(ui, list_group);
                    for index in clicked {
                        if let Some(on_click) =
                            list_group.items.get(index).and_then(|item| item.on_click)
                        {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::Nav(nav) => {
                    let clicked = self.backend.render_nav(ui, nav);
                    for index in clicked {
                        if let Some(on_click) = nav.items.get(index).and_then(|item| item.on_click)
                        {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::Navbar(navbar) => {
                    let result = self.backend.render_navbar(ui, navbar);
                    if result.brand_clicked
                        && let Some(on_brand_click) = navbar.on_brand_click
                    {
                        on_brand_click(ui.ctx());
                    }

                    for index in result.clicked_items {
                        if let Some(on_click) =
                            navbar.items.get(index).and_then(|item| item.on_click)
                        {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::Dropdown(dropdown) => {
                    let open = dropdown.state.current(ui.ctx());
                    let result = self.backend.render_dropdown(ui, &dropdown.props, open);

                    if result.toggle_clicked {
                        dropdown.state.toggle(ui.ctx());
                    }

                    if let Some(index) = result.clicked_item {
                        dropdown.state.set_current(ui.ctx(), false);
                        if let Some(on_click) = dropdown
                            .props
                            .items
                            .get(index)
                            .and_then(|item| item.on_click)
                        {
                            on_click(ui.ctx());
                        }
                    }
                }
                SemanticNode::FormLabel(form_label) => {
                    self.backend.render_form_label(ui, form_label)
                }
                SemanticNode::FormText(form_text) => self.backend.render_form_text(ui, form_text),
                SemanticNode::FormControl(form_control) => {
                    let changed = self.backend.render_form_control(
                        ui,
                        form_control.value,
                        &form_control.props,
                    );
                    if changed && let Some(on_change) = form_control.props.on_change {
                        on_change(ui.ctx());
                    }
                }
                SemanticNode::FormSelect(form_select) => {
                    let changed = self.backend.render_form_select(
                        ui,
                        form_select.selected_index,
                        &form_select.props,
                    );
                    if changed && let Some(on_change) = form_select.props.on_change {
                        on_change(ui.ctx());
                    }
                }
                SemanticNode::FormCheck(form_check) => {
                    let changed =
                        self.backend
                            .render_form_check(ui, form_check.checked, &form_check.props);
                    if changed && let Some(on_change) = form_check.props.on_change {
                        on_change(ui.ctx());
                    }
                }
                SemanticNode::FormRange(form_range) => {
                    let changed =
                        self.backend
                            .render_form_range(ui, form_range.value, &form_range.props);
                    if changed && let Some(on_change) = form_range.props.on_change {
                        on_change(ui.ctx());
                    }
                }
                SemanticNode::FloatingLabel(floating_label) => {
                    let changed = self.backend.render_floating_label(
                        ui,
                        floating_label.value,
                        &floating_label.props,
                    );
                    if changed && let Some(on_change) = floating_label.props.on_change {
                        on_change(ui.ctx());
                    }
                }
                SemanticNode::ValidationFeedback(validation_feedback) => self
                    .backend
                    .render_validation_feedback(ui, validation_feedback),
            }
        }
    }

    fn render_raw_egui(&mut self, ui: &mut egui::Ui, raw: &mut RawEguiNode<'a>) {
        let Some(body) = raw.body.take() else {
            return;
        };

        body(ui);
    }

    fn render_disclosure_scope(&mut self, ui: &mut egui::Ui, scope: &mut DisclosureScopeNode<'a>) {
        let open = scope.state.current(ui.ctx());
        self.backend.begin_disclosure_scope(ui, open);

        if open {
            self.render_nodes(ui, &mut scope.children);
        }

        self.backend.end_disclosure_scope(ui, open);
    }

    fn render_active_index_scope(
        &mut self,
        ui: &mut egui::Ui,
        scope: &mut ActiveIndexScopeNode<'a>,
    ) {
        let active_index = scope.state.current(ui.ctx());
        self.backend.begin_active_index_scope(ui, active_index);
        self.render_nodes(ui, &mut scope.children);
        self.backend.end_active_index_scope(ui, active_index);
    }

    fn render_style_override_scope(
        &mut self,
        ui: &mut egui::Ui,
        scope: &mut StyleOverrideNode<'a>,
    ) {
        let Some(style_override) = scope.style_override.take() else {
            self.render_nodes(ui, &mut scope.children);
            return;
        };

        let mut style = ui.style().as_ref().clone();
        style_override(&mut style);

        ui.scope(|scope_ui| {
            *scope_ui.style_mut() = style;
            self.render_nodes(scope_ui, &mut scope.children);
        });
    }

    fn render_theme_variant_scope(&mut self, ui: &mut egui::Ui, scope: &mut ThemeVariantNode<'a>) {
        let mut themed_style = ui.style().as_ref().clone();
        themed_style.visuals = scope.theme.default_visuals();

        ui.scope(|scope_ui| {
            *scope_ui.style_mut() = themed_style;
            self.render_nodes(scope_ui, &mut scope.children);
        });
    }

    fn render_container(&mut self, ui: &mut egui::Ui, container: &mut ContainerNode<'a>) {
        let props = container.props;
        self.backend.begin_container(ui, &props);

        match props.scroll {
            ContainerScroll::None => {
                self.render_container_body(ui, props, &mut container.children);
            }
            ContainerScroll::Horizontal => {
                egui::ScrollArea::horizontal()
                    .auto_shrink([false, false])
                    .show(ui, |scroll_ui| {
                        self.render_container_body(scroll_ui, props, &mut container.children);
                    });
            }
            ContainerScroll::Vertical => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |scroll_ui| {
                        self.render_container_body(scroll_ui, props, &mut container.children);
                    });
            }
            ContainerScroll::Both => {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |scroll_ui| {
                        self.render_container_body(scroll_ui, props, &mut container.children);
                    });
            }
        }

        self.backend.end_container(ui, &props);
    }

    fn render_container_body(
        &mut self,
        ui: &mut egui::Ui,
        props: ContainerProps,
        children: &mut Vec<SemanticNode<'a>>,
    ) {
        let available_width = ui.available_width();
        let container_width = self
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

            row_ui.allocate_ui_with_layout(
                egui::vec2(container_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |container_ui| {
                    if vertical_padding > 0.0 {
                        container_ui.add_space(vertical_padding);
                    }

                    container_ui.horizontal_top(|padding_ui| {
                        if horizontal_padding > 0.0 {
                            padding_ui.add_space(horizontal_padding);
                        }

                        let content_width = (container_width - horizontal_padding * 2.0).max(0.0);
                        padding_ui.allocate_ui_with_layout(
                            egui::vec2(content_width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |content_ui| {
                                self.render_nodes(content_ui, children);
                            },
                        );

                        if horizontal_padding > 0.0 {
                            padding_ui.add_space(horizontal_padding);
                        }
                    });

                    if vertical_padding > 0.0 {
                        container_ui.add_space(vertical_padding);
                    }
                },
            );
        });
    }

    fn render_row(&mut self, ui: &mut egui::Ui, row: &mut RowNode<'a>) {
        if row.cols.is_empty() {
            return;
        }

        let row_props = row.props;
        let row_width = ui.available_width();
        let active_breakpoint = resolve_active_breakpoint(row_width, self.backend.breakpoints());

        let mut col_specs = Vec::with_capacity(row.cols.len());
        for col in &row.cols {
            col.props.debug_assert_valid();

            let raw_span = col.props.resolve_span_unchecked(active_breakpoint);
            let span = if raw_span.is_valid() {
                raw_span
            } else {
                Span::Auto
            };

            let offset = if col.props.offset <= Span::MAX_COLUMNS {
                col.props.offset
            } else {
                0
            };

            let order = col
                .props
                .order
                .filter(|order| (ColProps::MIN_ORDER..=ColProps::MAX_ORDER).contains(order));

            col_specs.push(GridColSpec {
                span,
                offset,
                order,
            });
        }

        let gutter_x = row_props.spacing().gutter_x();
        let gutter_y = row_props.spacing().gutter_y();

        let computed_lines = compute_grid_lines(&col_specs, row_width, gutter_x, false)
            .or_else(|_| {
                let fallback_specs: Vec<GridColSpec> =
                    row.cols.iter().map(|_| GridColSpec::auto()).collect();
                compute_grid_lines(&fallback_specs, row_width, gutter_x, false)
            })
            .unwrap_or_default();

        for (line_index, line) in computed_lines.iter().enumerate() {
            if line_index > 0 && gutter_y > 0.0 {
                ui.add_space(gutter_y);
            }

            self.render_row_line(ui, row, line, row_props);
        }
    }

    fn render_row_line(
        &mut self,
        ui: &mut egui::Ui,
        row: &mut RowNode<'a>,
        line: &ComputedLine,
        row_props: RowProps,
    ) {
        ui.horizontal_top(|line_ui| {
            self.backend.begin_row(line_ui, &row_props, line);

            let mut cursor_x = 0.0_f32;
            for computed in &line.cols {
                let leading = (computed.x - cursor_x).max(0.0);
                if leading > 0.0 {
                    line_ui.add_space(leading);
                }

                if let Some(col) = row.cols.get_mut(computed.source_index) {
                    self.render_computed_col(line_ui, col, computed);
                }

                cursor_x = computed.x + computed.width;
            }

            self.backend.end_row(line_ui, &row_props, line);
        });
    }

    fn render_computed_col(
        &mut self,
        ui: &mut egui::Ui,
        col: &mut ColNode<'a>,
        computed: &ComputedCol,
    ) {
        let col_props = col.props;
        self.backend.begin_col(ui, &col_props, computed);

        ui.allocate_ui_with_layout(
            egui::vec2(computed.width.max(0.0), 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |col_ui| {
                self.render_nodes(col_ui, &mut col.children);
            },
        );

        self.backend.end_col(ui, &col_props, computed);
    }

    fn render_standalone_col(&mut self, ui: &mut egui::Ui, col: &mut ColNode<'a>) {
        let computed = ComputedCol {
            source_index: 0,
            span: Span::Auto,
            offset: 0,
            order: None,
            x: 0.0,
            width: ui.available_width(),
        };

        self.render_computed_col(ui, col, &computed);
    }

    fn render_card(&mut self, ui: &mut egui::Ui, card: &mut CardNode<'a>) {
        let mut backend = std::mem::take(&mut self.backend);
        let props = card.props.clone();
        let style = backend.card_style(&props);
        let theme = backend.theme().clone();

        backend.render_card(ui, &props, |card_ui| {
            if let Some(header) = &props.header {
                EguiBackend::render_heading_span(card_ui, header, style.header_text, &theme);
                if style.header_gap > 0.0 {
                    card_ui.add_space(style.header_gap);
                }
            }

            self.render_nodes(card_ui, &mut card.children);

            if let Some(footer) = &props.footer {
                if style.footer_gap > 0.0 {
                    card_ui.add_space(style.footer_gap);
                }
                card_ui.separator();
                EguiBackend::render_plain_span(card_ui, footer, style.footer_text, &theme);
            }
        });

        self.backend = backend;
    }

    fn render_plot(&mut self, ui: &mut egui::Ui, plot: &mut PlotNode<'a>) {
        let Some(body) = plot.body.take() else {
            return;
        };

        let style = self.backend.card_style(&CardProps::new());

        if let Some(title) = &plot.props.title {
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

        let plot_widget = plot.props.build_plot(ui.available_width());
        ui.scope(|plot_scope| {
            let visuals = plot_scope.visuals_mut();
            visuals.override_text_color = Some(style.body_text);
            visuals.widgets.noninteractive.fg_stroke.color = style.border;
            visuals.widgets.noninteractive.bg_stroke.color = style.border;

            plot_widget.show(plot_scope, |plot_ui| {
                body(plot_ui);
            });
        });
    }
}

enum BuildScope<'a> {
    Root(Vec<SemanticNode<'a>>),
    Container {
        props: ContainerProps,
        children: Vec<SemanticNode<'a>>,
    },
    Row {
        props: RowProps,
        cols: Vec<ColNode<'a>>,
    },
    Col {
        props: ColProps,
        children: Vec<SemanticNode<'a>>,
        implicit: bool,
    },
    Card {
        props: CardProps,
        children: Vec<SemanticNode<'a>>,
    },
    Disclosure {
        state: DisclosureState<'a>,
        children: Vec<SemanticNode<'a>>,
    },
    ActiveIndex {
        state: ActiveIndexState<'a>,
        children: Vec<SemanticNode<'a>>,
    },
    StyleOverride {
        style_override: Option<StyleOverride<'a>>,
        children: Vec<SemanticNode<'a>>,
    },
    ThemeVariant {
        theme: egui::Theme,
        children: Vec<SemanticNode<'a>>,
    },
}

enum SemanticNode<'a> {
    Container(ContainerNode<'a>),
    Row(RowNode<'a>),
    Col(ColNode<'a>),
    Card(CardNode<'a>),
    Plot(PlotNode<'a>),
    Disclosure(DisclosureScopeNode<'a>),
    ActiveIndex(ActiveIndexScopeNode<'a>),
    RawEgui(RawEguiNode<'a>),
    StyleOverride(StyleOverrideNode<'a>),
    ThemeVariant(ThemeVariantNode<'a>),
    Badge(BadgeProps),
    Label(LabelProps),
    Text(TextProps),
    Button(Button),
    Alert(AlertProps),
    Progress(ProgressProps),
    Table(TableProps),
    CloseButton(CloseButtonProps),
    Pagination(PaginationProps),
    Spinner(SpinnerProps),
    Placeholder(PlaceholderProps),
    ButtonGroup(ButtonGroupProps),
    ListGroup(ListGroupProps),
    Nav(NavProps),
    Navbar(NavbarProps),
    Dropdown(DropdownNode<'a>),
    FormLabel(FormLabelProps),
    FormText(FormTextProps),
    FormControl(FormControlNode<'a>),
    FormSelect(FormSelectNode<'a>),
    FormCheck(FormCheckNode<'a>),
    FormRange(FormRangeNode<'a>),
    FloatingLabel(FloatingLabelNode<'a>),
    ValidationFeedback(ValidationFeedbackProps),
}

struct ContainerNode<'a> {
    props: ContainerProps,
    children: Vec<SemanticNode<'a>>,
}

struct RowNode<'a> {
    props: RowProps,
    cols: Vec<ColNode<'a>>,
}

struct ColNode<'a> {
    props: ColProps,
    children: Vec<SemanticNode<'a>>,
    implicit: bool,
}

struct CardNode<'a> {
    props: CardProps,
    children: Vec<SemanticNode<'a>>,
}

struct PlotNode<'a> {
    props: PlotArea,
    body: Option<PlotBody<'a>>,
}

struct DisclosureScopeNode<'a> {
    state: DisclosureState<'a>,
    children: Vec<SemanticNode<'a>>,
}

struct ActiveIndexScopeNode<'a> {
    state: ActiveIndexState<'a>,
    children: Vec<SemanticNode<'a>>,
}

struct RawEguiNode<'a> {
    body: Option<RawEguiBody<'a>>,
}

struct StyleOverrideNode<'a> {
    style_override: Option<StyleOverride<'a>>,
    children: Vec<SemanticNode<'a>>,
}

struct ThemeVariantNode<'a> {
    theme: egui::Theme,
    children: Vec<SemanticNode<'a>>,
}

struct DropdownNode<'a> {
    props: DropdownProps,
    state: DisclosureState<'a>,
}

struct FormControlNode<'a> {
    value: &'a mut String,
    props: FormControlProps,
}

struct FormSelectNode<'a> {
    selected_index: &'a mut usize,
    props: FormSelectProps,
}

struct FormCheckNode<'a> {
    checked: &'a mut bool,
    props: FormCheckProps,
}

struct FormRangeNode<'a> {
    value: &'a mut f32,
    props: FormRangeProps,
}

struct FloatingLabelNode<'a> {
    value: &'a mut String,
    props: FloatingLabelProps,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Col;

    #[test]
    fn state_scopes_build_without_panicking() {
        let ctx = egui::Context::default();
        let mut open = true;
        let mut active_index = 0_usize;
        let mut ui = UiRoot::new(&ctx);

        ui.with_disclosure_state(&mut open, |ui| {
            ui.with_active_index_state(&mut active_index, |_ui| {});
        });

        // Compile-smoke for tree construction: rendering is gated by egui frame lifecycle.
        ui.render_with_central_panel();
    }

    #[test]
    fn strict_mode_reports_invalid_col_props() {
        let ctx = egui::Context::default();
        let mut ui = UiRoot::new(&ctx);

        ui.set_strict_mode(true);
        ui.col(Col::new().xs(Span::N(0)).offset(13).order(99), |_ui| {});

        let errors = ui.take_strict_mode_errors();
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
        let mut ui = UiRoot::new(&ctx);
        ui.set_strict_mode(true);

        let sink = Rc::clone(&reported);
        ui.set_strict_mode_error_reporter(move |error| sink.borrow_mut().push(error.to_string()));

        ui.col(Col::new().xs(Span::N(0)), |_ui| {});

        assert_eq!(reported.borrow().len(), 1);
        assert!(reported.borrow()[0].contains("ColProps.xs"));
    }
}
