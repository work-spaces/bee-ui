use crate::{
    components::{
        AlertProps, BadgeProps, Button, ButtonGroupProps, CardProps, CloseButtonProps, ColProps,
        ContainerProps, DropdownProps, FloatingLabelProps, FormCheckKind, FormCheckProps,
        FormControlProps, FormLabelProps, FormRangeProps, FormSelectProps, FormTextProps,
        LabelProps, LabelSize, ListGroupProps, NavProps, NavbarProps, PaginationProps,
        PlaceholderProps, ProgressProps, RowProps, SpinnerProps, TableProps, TextProps,
        ValidationFeedbackProps,
    },
    core::{
        Span, SpanFont, SpanIcon, SpanRun, SpanText, SpanTextStyle,
        theme::{CardStyle, FormValidationState, NavItemState, TableStyle, TextSize, UiTheme},
    },
    layout::{Breakpoints, ComputedCol, ComputedLine},
    render::{Backend, DropdownRenderResult, NavbarRenderResult},
    theme::fonts::{self, FontFace},
};

#[derive(Clone, Debug, Default)]
pub struct EguiBackend {
    row_item_spacing_stack: Vec<egui::Vec2>,
    theme: UiTheme,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpanBaseline {
    PlainText,
    ControlLabel,
    EmphasisChip,
    Heading,
    CodeLike,
}

impl SpanBaseline {
    const fn default_face(self) -> FontFace {
        match self {
            Self::PlainText => FontFace::PropoText,
            Self::ControlLabel => FontFace::PropoSemiBold,
            Self::EmphasisChip => FontFace::MonoExtraBold,
            Self::Heading => FontFace::PropoBold,
            Self::CodeLike => FontFace::MonoRegular,
        }
    }

    const fn prefers_strong_fallback(self) -> bool {
        matches!(
            self,
            Self::ControlLabel | Self::EmphasisChip | Self::Heading
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct SpanRenderDefaults {
    baseline: SpanBaseline,
    color: Option<egui::Color32>,
    size: Option<f32>,
    force_strong: bool,
    force_underline: bool,
}

impl SpanRenderDefaults {
    fn new(baseline: SpanBaseline) -> Self {
        Self {
            baseline,
            color: None,
            size: None,
            force_strong: false,
            force_underline: false,
        }
    }

    fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }

    fn size(mut self, size: f32) -> Self {
        self.size = Some(size.max(0.0));
        self
    }

    fn strong(mut self) -> Self {
        self.force_strong = true;
        self
    }
}

#[derive(Clone, Debug)]
struct ResolvedSpanTextStyle {
    color: Option<egui::Color32>,
    background: Option<egui::Color32>,
    size: Option<f32>,
    family: Option<egui::FontFamily>,
    strong: bool,
    weak: bool,
    italics: bool,
    underline: bool,
    strikethrough: bool,
}

impl EguiBackend {
    pub(crate) fn breakpoints(&self) -> Breakpoints {
        self.theme.breakpoints
    }

    pub(crate) fn sync_mode_from_ui(&mut self, ui: &egui::Ui) {
        self.theme.sync_mode_from_dark_mode(ui.visuals().dark_mode);
    }

    pub(crate) fn theme(&self) -> &UiTheme {
        &self.theme
    }

    pub(crate) fn card_style(&self, props: &CardProps) -> CardStyle {
        self.theme.resolve_card_style(props.variant, props.outlined)
    }

    pub(crate) fn resolve_container_width(
        &self,
        props: &ContainerProps,
        available_width: f32,
    ) -> f32 {
        let available_width = normalize_non_negative(available_width);

        let mut width = if props.fluid {
            available_width
        } else {
            bootstrap_fixed_container_width(available_width)
        };

        if let Some(max_width) = props.max_width {
            width = width.min(normalize_non_negative(max_width));
        }

        width.clamp(0.0, available_width)
    }

    fn resolve_span_font_family(ctx: &egui::Context, font: &SpanFont) -> Option<egui::FontFamily> {
        match font {
            SpanFont::Proportional => Some(egui::FontFamily::Proportional),
            SpanFont::Monospace => Some(egui::FontFamily::Monospace),
            SpanFont::Embedded(face) => {
                if fonts::ensure_face_registered(ctx, *face) {
                    Some(fonts::font_family_for(*face))
                } else {
                    None
                }
            }
        }
    }

    fn strong_face_for_baseline(baseline: SpanBaseline) -> FontFace {
        match baseline {
            SpanBaseline::PlainText | SpanBaseline::ControlLabel | SpanBaseline::Heading => {
                FontFace::PropoBold
            }
            SpanBaseline::EmphasisChip | SpanBaseline::CodeLike => FontFace::MonoExtraBold,
        }
    }

    fn resolve_span_text_style(
        ctx: &egui::Context,
        text_style: &SpanTextStyle,
        defaults: SpanRenderDefaults,
    ) -> ResolvedSpanTextStyle {
        let mut family = None;
        let mut pending_default_face = false;

        if let Some(font) = &text_style.font {
            family = Self::resolve_span_font_family(ctx, font);
        } else {
            let default_face = defaults.baseline.default_face();
            if fonts::ensure_face_registered(ctx, default_face) {
                family = Some(fonts::font_family_for(default_face));
            } else {
                pending_default_face = true;
            }
        }

        let mut strong = text_style.strong || defaults.force_strong;
        if pending_default_face && defaults.baseline.prefers_strong_fallback() {
            strong = true;
        }

        if strong && text_style.font.is_none() {
            let stronger_face = Self::strong_face_for_baseline(defaults.baseline);
            if fonts::ensure_face_registered(ctx, stronger_face) {
                family = Some(fonts::font_family_for(stronger_face));
            }
        }

        ResolvedSpanTextStyle {
            color: text_style.color.or(defaults.color),
            background: text_style.background,
            size: text_style.size.or(defaults.size),
            family,
            strong,
            weak: text_style.weak,
            italics: text_style.italics,
            underline: text_style.underline || defaults.force_underline,
            strikethrough: text_style.strikethrough,
        }
    }

    fn rich_text_for_span_text(
        ctx: &egui::Context,
        text: &SpanText,
        defaults: SpanRenderDefaults,
    ) -> egui::RichText {
        let resolved = Self::resolve_span_text_style(ctx, &text.style, defaults);

        let mut rich = egui::RichText::new(&text.text);

        if let Some(color) = resolved.color {
            rich = rich.color(color);
        }

        if let Some(background) = resolved.background {
            rich = rich.background_color(background);
        }

        if let Some(size) = resolved.size {
            rich = rich.size(size);
        }

        if let Some(family) = resolved.family {
            rich = rich.family(family);
        }

        if resolved.strong {
            rich = rich.strong();
        }

        if resolved.weak {
            rich = rich.weak();
        }

        if resolved.italics {
            rich = rich.italics();
        }

        if resolved.underline {
            rich = rich.underline();
        }

        if resolved.strikethrough {
            rich = rich.strikethrough();
        }

        rich
    }

    fn widget_text_format_for_span_text(
        ui: &egui::Ui,
        text: &SpanText,
        defaults: SpanRenderDefaults,
    ) -> egui::TextFormat {
        let resolved = Self::resolve_span_text_style(ui.ctx(), &text.style, defaults);

        let mut color = resolved.color.unwrap_or_else(|| ui.visuals().text_color());
        if resolved.weak {
            color = color.gamma_multiply(0.72_f32);
        }

        let mut font_id = egui::TextStyle::Body.resolve(ui.style());
        if let Some(size) = resolved.size {
            font_id.size = size;
        }
        if let Some(family) = resolved.family {
            font_id.family = family;
        }

        let underline = if resolved.underline {
            egui::Stroke::new(1.0_f32, color)
        } else {
            egui::Stroke::NONE
        };
        let strikethrough = if resolved.strikethrough {
            egui::Stroke::new(1.0_f32, color)
        } else {
            egui::Stroke::NONE
        };

        let mut format = egui::TextFormat::simple(font_id, color);
        if let Some(background) = resolved.background {
            format.background = background;
        }
        format.italics = resolved.italics;
        format.underline = underline;
        format.strikethrough = strikethrough;

        format
    }

    fn render_span_runs(
        ui: &mut egui::Ui,
        span: &Span,
        defaults: SpanRenderDefaults,
        theme: &UiTheme,
    ) {
        ui.scope(|scope_ui| {
            scope_ui.spacing_mut().item_spacing.x = 0.0;

            for run in &span.runs {
                match run {
                    SpanRun::Text(text) => {
                        if text.text.is_empty() {
                            continue;
                        }

                        scope_ui.label(Self::rich_text_for_span_text(
                            scope_ui.ctx(),
                            text,
                            defaults,
                        ));
                    }
                    SpanRun::Icon(icon) => {
                        if icon.gap_before > 0.0 {
                            scope_ui.add_space(icon.gap_before);
                        }

                        let tint = icon.resolve_tint(theme, defaults.color);
                        icon.render_with_tint(scope_ui, tint);

                        if icon.gap_after > 0.0 {
                            scope_ui.add_space(icon.gap_after);
                        }
                    }
                }
            }
        });
    }

    fn render_span_freeform(
        ui: &mut egui::Ui,
        span: &Span,
        defaults: SpanRenderDefaults,
        theme: &UiTheme,
    ) {
        if span.wrap {
            ui.horizontal_wrapped(|row_ui| Self::render_span_runs(row_ui, span, defaults, theme));
        } else {
            ui.horizontal(|row_ui| Self::render_span_runs(row_ui, span, defaults, theme));
        }
    }

    pub(crate) fn render_plain_span(
        ui: &mut egui::Ui,
        span: &Span,
        color: egui::Color32,
        theme: &UiTheme,
    ) {
        Self::render_span_freeform(
            ui,
            span,
            SpanRenderDefaults::new(SpanBaseline::PlainText).color(color),
            theme,
        );
    }

    pub(crate) fn render_heading_span(
        ui: &mut egui::Ui,
        span: &Span,
        color: egui::Color32,
        theme: &UiTheme,
    ) {
        Self::render_span_freeform(
            ui,
            span,
            SpanRenderDefaults::new(SpanBaseline::Heading).color(color),
            theme,
        );
    }

    fn span_to_widget_text(
        ui: &egui::Ui,
        span: &Span,
        defaults: SpanRenderDefaults,
    ) -> egui::WidgetText {
        let mut job = egui::text::LayoutJob::default();
        let mut has_text = false;

        for run in &span.runs {
            let SpanRun::Text(text) = run else {
                continue;
            };

            if text.text.is_empty() {
                continue;
            }

            has_text = true;
            let format = Self::widget_text_format_for_span_text(ui, text, defaults);
            job.append(&text.text, 0.0, format);
        }

        if has_text {
            egui::WidgetText::from(job)
        } else {
            egui::WidgetText::from(span.plain_text())
        }
    }

    fn span_to_button(
        ui: &egui::Ui,
        span: &Span,
        defaults: SpanRenderDefaults,
        theme: &UiTheme,
    ) -> egui::Button<'static> {
        let mut first_icon: Option<&SpanIcon> = None;
        let mut icon_before_text = false;
        let mut has_text = false;

        for run in &span.runs {
            match run {
                SpanRun::Text(text) if !text.text.is_empty() => {
                    has_text = true;
                }
                SpanRun::Icon(icon) if first_icon.is_none() => {
                    first_icon = Some(icon);
                    icon_before_text = !has_text;
                }
                _ => {}
            }
        }

        match (first_icon, has_text) {
            (Some(icon), true) => {
                let text = Self::span_to_widget_text(ui, span, defaults);
                let icon_tint = icon.resolve_tint(theme, defaults.color);
                let icon_follows_text_color = icon_tint.is_none() && icon.uses_current_color();
                let icon_image = icon.egui_image(icon_tint);

                if icon_before_text {
                    egui::Button::new((icon_image, text))
                        .image_tint_follows_text_color(icon_follows_text_color)
                } else {
                    egui::Button::new((text, icon_image))
                        .image_tint_follows_text_color(icon_follows_text_color)
                }
            }
            (Some(icon), false) => {
                let icon_tint = icon.resolve_tint(theme, defaults.color);
                let icon_follows_text_color = icon_tint.is_none() && icon.uses_current_color();
                let icon_image = icon.egui_image(icon_tint);

                egui::Button::new(icon_image).image_tint_follows_text_color(icon_follows_text_color)
            }
            (None, _) => {
                let text = Self::span_to_widget_text(ui, span, defaults);
                egui::Button::new(text)
            }
        }
    }

    fn style_button(
        button: egui::Button<'static>,
        fill: egui::Color32,
        border: egui::Color32,
        border_width: f32,
        corner_radius_value: u8,
    ) -> egui::Button<'static> {
        let mut button = button.corner_radius(corner_radius(corner_radius_value));
        if fill != egui::Color32::TRANSPARENT {
            button = button.fill(fill);
        }
        if border_width > 0.0 && border != egui::Color32::TRANSPARENT {
            button = button.stroke(egui::Stroke::new(border_width, border));
        }
        button
    }

    fn render_text_edit(
        ui: &mut egui::Ui,
        value: &mut String,
        props: &FormControlProps,
    ) -> egui::Response {
        let desired_width = normalize_non_negative(props.width.unwrap_or(ui.available_width()));

        let mut text_edit = egui::TextEdit::singleline(value)
            .desired_width(desired_width)
            .password(props.password);

        if let Some(placeholder) = &props.placeholder {
            text_edit = text_edit.hint_text(placeholder);
        }

        ui.add_enabled(!props.disabled, text_edit)
    }
}

impl Backend for EguiBackend {
    fn begin_container(&mut self, _ui: &mut egui::Ui, _props: &ContainerProps) {}

    fn end_container(&mut self, _ui: &mut egui::Ui, _props: &ContainerProps) {}

    fn begin_row(&mut self, ui: &mut egui::Ui, _props: &RowProps, _line: &ComputedLine) {
        let previous = ui.spacing().item_spacing;
        self.row_item_spacing_stack.push(previous);
        ui.spacing_mut().item_spacing = egui::vec2(0.0, previous.y);
    }

    fn end_row(&mut self, ui: &mut egui::Ui, _props: &RowProps, _line: &ComputedLine) {
        if let Some(previous) = self.row_item_spacing_stack.pop() {
            ui.spacing_mut().item_spacing = previous;
        }
    }

    fn begin_col(&mut self, _ui: &mut egui::Ui, _props: &ColProps, _computed: &ComputedCol) {}

    fn end_col(&mut self, _ui: &mut egui::Ui, _props: &ColProps, _computed: &ComputedCol) {}

    fn begin_disclosure_scope(&mut self, _ui: &mut egui::Ui, _open: bool) {}

    fn end_disclosure_scope(&mut self, _ui: &mut egui::Ui, _open: bool) {}

    fn begin_active_index_scope(&mut self, _ui: &mut egui::Ui, _active_index: usize) {}

    fn end_active_index_scope(&mut self, _ui: &mut egui::Ui, _active_index: usize) {}

    fn render_card<R, F>(&mut self, ui: &mut egui::Ui, props: &CardProps, body: F) -> R
    where
        F: FnOnce(&mut egui::Ui) -> R,
    {
        let style = self.theme.resolve_card_style(props.variant, props.outlined);

        let mut frame = egui::Frame::group(ui.style());
        frame.fill = style.fill;
        frame.stroke = egui::Stroke::new(style.border_width, style.border);
        frame.corner_radius = corner_radius(style.corner_radius);
        frame.inner_margin =
            egui::Margin::symmetric(as_i8(style.body_padding_x), as_i8(style.body_padding_y));

        let card_width = normalize_non_negative(ui.available_width());
        let card_height = if props.fill_height {
            normalize_non_negative(ui.available_height())
        } else {
            0.0
        };
        ui.allocate_ui_with_layout(
            egui::vec2(card_width, card_height),
            egui::Layout::top_down(egui::Align::Min),
            |card_slot_ui| {
                frame
                    .show(card_slot_ui, |card_ui| {
                        card_ui.set_min_width(card_ui.available_width());
                        body(card_ui)
                    })
                    .inner
            },
        )
        .inner
    }

    fn render_badge(&mut self, ui: &mut egui::Ui, props: &BadgeProps) {
        let style = self.theme.resolve_badge_style(props.variant, props.pill);

        ui.scope(|scope_ui| {
            scope_ui.spacing_mut().button_padding = egui::vec2(style.padding_x, style.padding_y);

            let badge = Self::span_to_button(
                scope_ui,
                &props.text,
                SpanRenderDefaults::new(SpanBaseline::EmphasisChip)
                    .color(style.text)
                    .size(style.text_size),
                &self.theme,
            )
            .sense(egui::Sense::hover())
            .fill(style.fill)
            .stroke(egui::Stroke::new(style.border_width, style.border))
            .corner_radius(corner_radius(style.corner_radius));

            scope_ui.add(badge);
        });
    }

    fn render_label(&mut self, ui: &mut egui::Ui, props: &LabelProps) {
        let size = match props.size {
            LabelSize::Small => TextSize::Small,
            LabelSize::Medium => TextSize::Medium,
            LabelSize::Large => TextSize::Large,
        };

        let style = self
            .theme
            .resolve_label_style(props.variant, size, props.muted);

        Self::render_span_freeform(
            ui,
            &props.text,
            SpanRenderDefaults::new(SpanBaseline::PlainText)
                .color(style.text)
                .size(style.text_size),
            &self.theme,
        );
    }

    fn render_text(&mut self, ui: &mut egui::Ui, props: &TextProps) {
        let style = self
            .theme
            .resolve_label_style(props.variant, TextSize::Medium, props.muted);

        Self::render_span_freeform(
            ui,
            &props.text,
            SpanRenderDefaults::new(SpanBaseline::PlainText)
                .color(style.text)
                .size(style.text_size),
            &self.theme,
        );
    }

    fn render_button(&mut self, ui: &mut egui::Ui, button: &Button) -> egui::Response {
        let style = self.theme.resolve_button_style(button.variant);

        let response = ui
            .scope(|scope_ui| {
                scope_ui.spacing_mut().button_padding =
                    egui::vec2(style.padding_x, style.padding_y);

                let text = egui::RichText::new(&button.text)
                    .color(style.text)
                    .size(style.text_size);

                let widget = if let Some(icon) = button.icon {
                    let icon = SpanIcon::new(icon);
                    let icon_follows_text_color = icon.uses_current_color();
                    let icon_image = icon.egui_image(None);

                    if button.icon_after_text {
                        egui::Button::new((text, icon_image))
                            .image_tint_follows_text_color(icon_follows_text_color)
                    } else {
                        egui::Button::new((icon_image, text))
                            .image_tint_follows_text_color(icon_follows_text_color)
                    }
                } else {
                    egui::Button::new(text)
                };

                let widget = Self::style_button(
                    widget,
                    style.fill,
                    style.border,
                    style.border_width,
                    style.corner_radius,
                );

                scope_ui.add_enabled(button.enabled, widget)
            })
            .inner;

        if let Some(hover_text) = &button.hover_text {
            response.on_hover_text(hover_text)
        } else {
            response
        }
    }

    fn render_alert(&mut self, ui: &mut egui::Ui, props: &AlertProps) {
        let style = self
            .theme
            .resolve_alert_style(props.variant, props.outlined);

        let mut frame = egui::Frame::group(ui.style());
        frame.fill = style.fill;
        frame.stroke = egui::Stroke::new(style.border_width, style.border);
        frame.corner_radius = corner_radius(style.corner_radius);
        frame.inner_margin =
            egui::Margin::symmetric(as_i8(style.padding_x), as_i8(style.padding_y));

        frame.show(ui, |alert_ui| {
            Self::render_span_freeform(
                alert_ui,
                &props.text,
                SpanRenderDefaults::new(SpanBaseline::PlainText).color(style.text),
                &self.theme,
            );
        });

        if style.margin_bottom > 0.0 {
            ui.add_space(style.margin_bottom);
        }
    }

    fn render_progress(&mut self, ui: &mut egui::Ui, props: &ProgressProps) {
        let style = self.theme.resolve_progress_style(props.variant);

        let mut progress = egui::ProgressBar::new(props.fraction())
            .fill(style.fill)
            .desired_width(ui.available_width())
            .desired_height(style.height)
            .animate(style.transition_seconds > 0.0);

        if let Some(label) = &props.label {
            progress = progress.text(Self::span_to_widget_text(
                ui,
                label,
                SpanRenderDefaults::new(SpanBaseline::PlainText)
                    .color(style.text)
                    .size(style.text_size),
            ));
        } else if props.show_percentage {
            progress = progress.text(
                egui::RichText::new(format!("{:.0}%", props.percent_value()))
                    .color(style.text)
                    .size(style.text_size),
            );
        }

        ui.add(progress);
    }

    fn render_table(&mut self, ui: &mut egui::Ui, props: &TableProps, id_salt: u64) -> Vec<usize> {
        let column_count = props.column_count();
        if column_count == 0 {
            return Vec::new();
        }

        struct SemanticTableDelegate<'a> {
            props: &'a TableProps,
            style: TableStyle,
            row_height: f32,
            clicked_rows: std::collections::BTreeSet<usize>,
        }

        impl<'a> SemanticTableDelegate<'a> {
            fn new(props: &'a TableProps, style: TableStyle, row_height: f32) -> Self {
                Self {
                    props,
                    style,
                    row_height,
                    clicked_rows: std::collections::BTreeSet::new(),
                }
            }

            fn row_fill_for(&self, row_index: usize, active: bool, hovered: bool) -> egui::Color32 {
                if active {
                    self.style.row_active_fill
                } else if hovered {
                    self.style.row_hover_fill
                } else if self.props.striped && row_index % 2 == 1 {
                    self.style.row_alt_fill
                } else {
                    self.style.row_fill
                }
            }
        }

        impl egui_table::TableDelegate for SemanticTableDelegate<'_> {
            fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::HeaderCellInfo) {
                let header = self
                    .props
                    .headers
                    .get(cell.col_range.start)
                    .cloned()
                    .unwrap_or_default();

                let cell_rect = ui.max_rect();
                ui.painter()
                    .rect_filled(cell_rect, corner_radius(0), self.style.header_fill);
                ui.painter().rect_stroke(
                    cell_rect,
                    corner_radius(0),
                    egui::Stroke::new(self.style.border_width, self.style.border),
                    egui::StrokeKind::Inside,
                );

                let content_rect =
                    cell_rect.shrink2(egui::vec2(self.style.cell_padding, self.style.cell_padding));
                let mut content_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );

                content_ui.label(EguiBackend::span_to_widget_text(
                    ui,
                    &header,
                    SpanRenderDefaults::new(SpanBaseline::Heading)
                        .color(self.style.header_text)
                        .strong(),
                ));
            }

            fn row_ui(&mut self, ui: &mut egui::Ui, row_nr: u64) {
                let row_index = row_nr as usize;
                let Some(row) = self.props.rows.get(row_index) else {
                    return;
                };

                let hovered = self.props.hover
                    && row.on_click.is_some()
                    && ui.rect_contains_pointer(ui.max_rect());

                let fill = self.row_fill_for(row_index, row.active, hovered);
                ui.painter()
                    .rect_filled(ui.max_rect(), corner_radius(0), fill);
            }

            fn cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::CellInfo) {
                let row_index = cell.row_nr as usize;
                let Some(row) = self.props.rows.get(row_index) else {
                    return;
                };

                let span = row.cells.get(cell.col_nr).cloned().unwrap_or_default();

                let cell_rect = ui.max_rect();
                ui.painter().rect_stroke(
                    cell_rect,
                    corner_radius(0),
                    egui::Stroke::new(self.style.border_width, self.style.border),
                    egui::StrokeKind::Inside,
                );

                let clickable = row.on_click.is_some();
                let mut response = ui.interact(
                    cell_rect,
                    egui::Id::new((
                        "runnerd-bs-table-cell",
                        cell.table_id,
                        cell.row_nr,
                        cell.col_nr,
                    )),
                    if clickable {
                        egui::Sense::click()
                    } else {
                        egui::Sense::hover()
                    },
                );

                let content_rect =
                    cell_rect.shrink2(egui::vec2(self.style.cell_padding, self.style.cell_padding));
                let mut content_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );

                content_ui.label(EguiBackend::span_to_widget_text(
                    ui,
                    &span,
                    SpanRenderDefaults::new(SpanBaseline::PlainText).color(self.style.row_text),
                ));

                if clickable {
                    response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
                }

                if response.clicked() {
                    self.clicked_rows.insert(row_index);
                }
            }

            fn default_row_height(&self) -> f32 {
                self.row_height
            }
        }

        let style = self.theme.resolve_table_style(props.variant, props.striped);
        let row_height =
            (ui.text_style_height(&egui::TextStyle::Body) + style.cell_padding * 2.0).max(20.0);

        // In some parent layouts `available_width` can reflect only the remaining cursor space,
        // which may be much smaller than the allocated container width.
        // Prefer the larger of the two so tables size like other full-width components.
        let parent_width = normalize_non_negative(ui.available_width().max(ui.max_rect().width()));
        let default_col_width = (parent_width / column_count as f32).max(80.0);

        let columns = (0..column_count)
            .map(|_| {
                egui_table::Column::new(default_col_width)
                    .range(egui::Rangef::new(48.0, f32::INFINITY))
                    .resizable(props.resizable_columns)
            })
            .collect::<Vec<_>>();

        let headers = if props.headers.is_empty() {
            Vec::new()
        } else {
            vec![egui_table::HeaderRow::new(row_height)]
        };

        let mut delegate = SemanticTableDelegate::new(props, style, row_height);

        // `egui_table` expects its viewport size to be defined ahead of time.
        // In unconstrained layouts this can otherwise collapse to header-only rendering.
        let header_height = if props.headers.is_empty() {
            0.0
        } else {
            row_height
        };
        let body_height = row_height * props.rows.len() as f32;
        let table_height = (header_height + body_height).max(row_height);

        let table_scope_id = ui.next_auto_id();

        ui.push_id(table_scope_id, |scoped_parent_ui| {
            scoped_parent_ui.allocate_ui_with_layout(
                egui::vec2(parent_width, table_height),
                egui::Layout::top_down(egui::Align::Min),
                |table_ui| {
                    table_ui.set_min_size(egui::vec2(parent_width, table_height));

                    let table_id_salt = ("runnerd-bs-table", id_salt);
                    let table_id =
                        egui_table::TableState::id(table_ui, egui::Id::new(table_id_salt));

                    // Seed initial column widths so first render uses equal-width columns
                    // instead of shrinking to content in egui_table's first sizing pass.
                    // Also version this initialization so we can correct older persisted
                    // table states that were captured before this behavior.
                    let init_version_id = table_id.with("runnerd-bs-init-version");
                    let init_version = table_ui
                        .ctx()
                        .data_mut(|d| d.get_persisted::<u8>(init_version_id));
                    let needs_init = init_version != Some(1);
                    let has_state =
                        egui_table::TableState::load(table_ui.ctx(), table_id).is_some();

                    if needs_init || !has_state {
                        let mut initial_state = egui_table::TableState {
                            parent_width: Some(parent_width),
                            ..Default::default()
                        };
                        for (index, column) in columns.iter().enumerate() {
                            initial_state
                                .col_widths
                                .insert(column.id_for(index), default_col_width);
                        }
                        initial_state.store(table_ui.ctx(), table_id);
                        table_ui
                            .ctx()
                            .data_mut(|d| d.insert_persisted(init_version_id, 1_u8));
                    }

                    let _ = egui_table::Table::new()
                        .id_salt(table_id_salt)
                        .num_rows(props.rows.len() as u64)
                        .columns(columns)
                        .num_sticky_cols(props.sticky_columns.min(column_count))
                        .headers(headers)
                        .auto_size_mode(props.auto_size_mode)
                        .show(table_ui, &mut delegate);
                },
            );
        });

        delegate.clicked_rows.into_iter().collect()
    }

    fn render_close_button(
        &mut self,
        ui: &mut egui::Ui,
        props: &CloseButtonProps,
    ) -> egui::Response {
        let style = self.theme.resolve_close_button_style();

        let label_color = if props.enabled {
            style.text
        } else {
            style.disabled_text
        };

        let widget = egui::Button::new(Self::span_to_widget_text(
            ui,
            &props.label,
            SpanRenderDefaults::new(SpanBaseline::ControlLabel)
                .color(label_color)
                .size(style.text_size),
        ))
        .small()
        .frame(false)
        .fill(egui::Color32::TRANSPARENT)
        .corner_radius(corner_radius(style.corner_radius));

        let response = ui.add_enabled(props.enabled, widget);
        if let Some(hover_text) = &props.hover_text {
            response.on_hover_text(hover_text)
        } else {
            response
        }
    }

    fn render_pagination(&mut self, ui: &mut egui::Ui, props: &PaginationProps) -> Vec<usize> {
        if props.items.is_empty() {
            return Vec::new();
        }

        let mut clicked = Vec::new();

        ui.horizontal(|pagination_ui| {
            for (index, item) in props.items.iter().enumerate() {
                let state = if item.disabled {
                    NavItemState::Disabled
                } else if item.active {
                    NavItemState::Active
                } else {
                    NavItemState::Default
                };
                let style = self
                    .theme
                    .resolve_pagination_item_style(props.variant, state);

                let response = pagination_ui
                    .scope(|item_ui| {
                        item_ui.spacing_mut().button_padding =
                            egui::vec2(style.padding_x, style.padding_y);
                        let button = Self::style_button(
                            Self::span_to_button(
                                item_ui,
                                &item.text,
                                SpanRenderDefaults::new(SpanBaseline::ControlLabel)
                                    .color(style.text)
                                    .size(style.text_size),
                                &self.theme,
                            ),
                            if item.active {
                                style.active_fill
                            } else {
                                style.fill
                            },
                            if item.active {
                                style.active_border
                            } else {
                                style.border
                            },
                            style.border_width,
                            style.corner_radius,
                        );

                        item_ui.add_enabled(!item.disabled, button)
                    })
                    .inner;

                if response.clicked() {
                    clicked.push(index);
                }
            }
        });

        clicked
    }

    fn render_spinner(&mut self, ui: &mut egui::Ui, props: &SpinnerProps) {
        let style = self.theme.resolve_spinner_style(props.variant);
        let size = if props.size > 0.0 {
            normalize_non_negative(props.size)
        } else {
            style.size
        };

        ui.horizontal(|spinner_ui| {
            spinner_ui.spacing_mut().item_spacing =
                egui::vec2(style.label_gap, spinner_ui.spacing().item_spacing.y);

            spinner_ui.add_sized(
                egui::vec2(size, size),
                egui::Spinner::new().size(size).color(style.color),
            );

            if let Some(label) = &props.label {
                Self::render_span_freeform(
                    spinner_ui,
                    label,
                    SpanRenderDefaults::new(SpanBaseline::PlainText)
                        .color(style.text)
                        .size(style.text_size),
                    &self.theme,
                );
            }
        });
    }

    fn render_placeholder(&mut self, ui: &mut egui::Ui, props: &PlaceholderProps) {
        let style = self
            .theme
            .resolve_placeholder_style(props.variant, props.pulse);
        let width = normalize_non_negative(props.width.unwrap_or(ui.available_width()));
        let height = normalize_non_negative(props.height);

        let mut frame = egui::Frame::new();
        frame.fill = style.fill;
        frame.corner_radius = corner_radius(style.corner_radius);
        frame.show(ui, |placeholder_ui| {
            placeholder_ui.allocate_space(egui::vec2(width, height));
        });
    }

    fn render_button_group(&mut self, ui: &mut egui::Ui, props: &ButtonGroupProps) -> Vec<usize> {
        if props.buttons.is_empty() {
            return Vec::new();
        }

        let mut clicked = Vec::new();
        let style = self.theme.resolve_button_group_style();

        if props.vertical {
            ui.vertical(|group_ui| {
                group_ui.spacing_mut().item_spacing =
                    egui::vec2(group_ui.spacing().item_spacing.x, style.gap);
                for (index, button) in props.buttons.iter().enumerate() {
                    let response = self.render_button(group_ui, button);
                    if response.clicked() {
                        clicked.push(index);
                    }
                }
            });
        } else {
            ui.horizontal(|group_ui| {
                group_ui.spacing_mut().item_spacing =
                    egui::vec2(style.gap, group_ui.spacing().item_spacing.y);
                for (index, button) in props.buttons.iter().enumerate() {
                    let response = self.render_button(group_ui, button);
                    if response.clicked() {
                        clicked.push(index);
                    }
                }
            });
        }

        clicked
    }

    fn render_list_group(&mut self, ui: &mut egui::Ui, props: &ListGroupProps) -> Vec<usize> {
        if props.items.is_empty() {
            return Vec::new();
        }

        let mut clicked = Vec::new();

        ui.vertical(|list_ui| {
            if props.flush {
                list_ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            }

            for (index, item) in props.items.iter().enumerate() {
                let state = if item.disabled {
                    NavItemState::Disabled
                } else if item.active {
                    NavItemState::Active
                } else {
                    NavItemState::Default
                };
                let style =
                    self.theme
                        .resolve_list_group_item_style(item.variant, state, props.flush);

                let response = list_ui
                    .scope(|item_ui| {
                        item_ui.spacing_mut().button_padding =
                            egui::vec2(style.padding_x, style.padding_y);
                        let button = Self::style_button(
                            Self::span_to_button(
                                item_ui,
                                &item.text,
                                SpanRenderDefaults::new(SpanBaseline::PlainText)
                                    .color(style.text)
                                    .size(style.text_size),
                                &self.theme,
                            ),
                            if item.active {
                                style.action_active_fill
                            } else {
                                style.fill
                            },
                            style.border,
                            style.border_width,
                            style.corner_radius,
                        );

                        item_ui
                            .add_enabled_ui(!item.disabled, |row_ui| {
                                row_ui.add_sized(egui::vec2(row_ui.available_width(), 0.0), button)
                            })
                            .inner
                    })
                    .inner;

                if response.clicked() {
                    clicked.push(index);
                }
            }
        });

        clicked
    }

    fn render_nav(&mut self, ui: &mut egui::Ui, props: &NavProps) -> Vec<usize> {
        if props.items.is_empty() {
            return Vec::new();
        }

        let mut clicked = Vec::new();

        let render_items = |nav_ui: &mut egui::Ui, clicked: &mut Vec<usize>| {
            let has_equal_width_items = props.fill || props.justified;
            let item_width = if has_equal_width_items {
                (nav_ui.available_width() / props.items.len() as f32).max(0.0)
            } else {
                0.0
            };

            for (index, item) in props.items.iter().enumerate() {
                let state = if item.disabled {
                    NavItemState::Disabled
                } else if item.active {
                    NavItemState::Active
                } else {
                    NavItemState::Default
                };
                let style = self.theme.resolve_nav_item_style(props.variant, state);

                let response = nav_ui
                    .scope(|item_ui| {
                        item_ui.spacing_mut().button_padding =
                            egui::vec2(style.padding_x, style.padding_y);
                        let button = Self::style_button(
                            Self::span_to_button(
                                item_ui,
                                &item.text,
                                SpanRenderDefaults::new(SpanBaseline::PlainText).color(style.text),
                                &self.theme,
                            ),
                            if item.active {
                                style.hover_fill
                            } else {
                                style.fill
                            },
                            if item.active {
                                style.hover_border
                            } else {
                                style.border
                            },
                            style.border_width,
                            style.corner_radius,
                        );

                        if has_equal_width_items {
                            item_ui
                                .add_enabled_ui(!item.disabled, |row_ui| {
                                    row_ui.add_sized(egui::vec2(item_width, 0.0), button)
                                })
                                .inner
                        } else {
                            item_ui.add_enabled(!item.disabled, button)
                        }
                    })
                    .inner;

                if response.clicked() {
                    clicked.push(index);
                }
            }
        };

        if props.vertical {
            ui.vertical(|nav_ui| {
                if props.fill || props.justified {
                    nav_ui.spacing_mut().item_spacing =
                        egui::vec2(0.0, nav_ui.spacing().item_spacing.y);
                }
                render_items(nav_ui, &mut clicked);
            });
        } else {
            ui.horizontal_wrapped(|nav_ui| {
                render_items(nav_ui, &mut clicked);
            });
        }

        clicked
    }

    fn render_navbar(&mut self, ui: &mut egui::Ui, props: &NavbarProps) -> NavbarRenderResult {
        if props.is_empty() {
            return NavbarRenderResult::default();
        }

        let style = self.theme.resolve_navbar_style(props.variant);
        let mut result = NavbarRenderResult::default();

        let mut frame = egui::Frame::group(ui.style());
        frame.fill = style.fill;
        frame.stroke = egui::Stroke::new(style.border_width, style.border);
        frame.corner_radius = corner_radius(style.corner_radius);
        frame.inner_margin = egui::Margin::symmetric(style.padding_x, style.padding_y);

        frame.show(ui, |navbar_ui| {
            navbar_ui.horizontal_wrapped(|row_ui| {
                row_ui.spacing_mut().item_spacing =
                    egui::vec2(style.item_gap, row_ui.spacing().item_spacing.y);

                if let Some(brand) = &props.brand {
                    let brand_text = Self::span_to_widget_text(
                        row_ui,
                        brand,
                        SpanRenderDefaults::new(SpanBaseline::Heading).color(style.brand_text),
                    );
                    let brand_response = if props.on_brand_click.is_some() {
                        row_ui.add(egui::Label::new(brand_text).sense(egui::Sense::click()))
                    } else {
                        row_ui.label(brand_text)
                    };

                    if brand_response.clicked() {
                        result.brand_clicked = true;
                    }

                    if !props.items.is_empty() {
                        row_ui.separator();
                    }
                }

                for (index, item) in props.items.iter().enumerate() {
                    let state = if item.disabled {
                        NavItemState::Disabled
                    } else if item.active {
                        NavItemState::Active
                    } else {
                        NavItemState::Default
                    };
                    let mut item_style = self.theme.resolve_nav_item_style(props.variant, state);
                    item_style.padding_x = style.nav_link_padding_x;
                    let text_color = match state {
                        NavItemState::Active => style.active_text,
                        NavItemState::Disabled => style.disabled_text,
                        NavItemState::Default => item_style.text,
                    };

                    let response = row_ui
                        .scope(|item_ui| {
                            item_ui.spacing_mut().button_padding =
                                egui::vec2(item_style.padding_x, item_style.padding_y);
                            let button = Self::style_button(
                                Self::span_to_button(
                                    item_ui,
                                    &item.text,
                                    SpanRenderDefaults::new(SpanBaseline::PlainText)
                                        .color(text_color),
                                    &self.theme,
                                ),
                                if item.active {
                                    item_style.hover_fill
                                } else {
                                    item_style.fill
                                },
                                if item.active {
                                    item_style.hover_border
                                } else {
                                    item_style.border
                                },
                                item_style.border_width,
                                item_style.corner_radius,
                            );

                            item_ui.add_enabled(!item.disabled, button)
                        })
                        .inner;
                    if response.clicked() {
                        result.clicked_items.push(index);
                    }
                }
            });
        });

        result
    }

    fn render_dropdown(
        &mut self,
        ui: &mut egui::Ui,
        props: &DropdownProps,
        open: bool,
    ) -> DropdownRenderResult {
        let mut result = DropdownRenderResult::default();
        let style = self.theme.resolve_dropdown_style(props.variant, open);
        let selected_text = props
            .items
            .iter()
            .find(|item| item.active)
            .map(|item| item.text.as_str())
            .unwrap_or(props.label.as_str());
        let selected_text_color = if props.disabled {
            style.item_disabled_text
        } else {
            style.toggle_text
        };

        ui.add_enabled_ui(!props.disabled, |enabled_ui| {
            enabled_ui.scope(|combo_ui| {
                combo_ui.spacing_mut().button_padding =
                    egui::vec2(style.toggle_padding_x, style.toggle_padding_y);

                let popup_visuals = combo_ui.visuals().clone();
                let border_stroke =
                    egui::Stroke::new(style.toggle_border_width, style.toggle_border);

                {
                    let visuals = combo_ui.visuals_mut();

                    visuals.widgets.inactive.bg_fill = style.toggle_fill;
                    visuals.widgets.inactive.weak_bg_fill = style.toggle_fill;
                    visuals.widgets.inactive.bg_stroke = border_stroke;
                    visuals.widgets.inactive.fg_stroke.color = style.toggle_text;

                    visuals.widgets.hovered.bg_fill = style.toggle_fill;
                    visuals.widgets.hovered.weak_bg_fill = style.toggle_fill;
                    visuals.widgets.hovered.bg_stroke = border_stroke;
                    visuals.widgets.hovered.fg_stroke.color = style.toggle_text;

                    visuals.widgets.active.bg_fill = style.toggle_fill;
                    visuals.widgets.active.weak_bg_fill = style.toggle_fill;
                    visuals.widgets.active.bg_stroke = border_stroke;
                    visuals.widgets.active.fg_stroke.color = style.toggle_text;

                    visuals.widgets.open.bg_fill = style.toggle_fill;
                    visuals.widgets.open.weak_bg_fill = style.toggle_fill;
                    visuals.widgets.open.bg_stroke = border_stroke;
                    visuals.widgets.open.fg_stroke.color = style.toggle_text;

                    visuals.widgets.noninteractive.bg_fill = style.toggle_fill;
                    visuals.widgets.noninteractive.weak_bg_fill = style.toggle_fill;
                    visuals.widgets.noninteractive.bg_stroke = border_stroke;
                    visuals.widgets.noninteractive.fg_stroke.color = style.item_disabled_text;
                }

                let combo_response = egui::ComboBox::from_id_salt(combo_ui.id().with((
                    "dropdown",
                    props.label.as_str(),
                    props.items.len(),
                )))
                .selected_text(egui::RichText::new(selected_text).color(selected_text_color))
                .show_ui(combo_ui, |options_ui| {
                    *options_ui.visuals_mut() = popup_visuals.clone();

                    for (index, item) in props.items.iter().enumerate() {
                        let text_color = if item.disabled {
                            style.item_disabled_text
                        } else if item.active {
                            style.item_active_text
                        } else {
                            style.item_text
                        };

                        let response = options_ui
                            .add_enabled_ui(!item.disabled, |item_ui| {
                                item_ui.selectable_label(
                                    item.active,
                                    egui::RichText::new(&item.text).color(text_color),
                                )
                            })
                            .inner;

                        if response.clicked() {
                            result.clicked_item = Some(index);
                        }
                    }
                });

                if combo_response.response.clicked() {
                    result.toggle_clicked = true;
                }
            });
        });

        result
    }

    fn render_form_label(&mut self, ui: &mut egui::Ui, props: &FormLabelProps) {
        let style = self
            .theme
            .resolve_form_label_style(props.variant, props.muted || props.required);

        let response = ui
            .horizontal(|label_ui| {
                Self::render_span_freeform(
                    label_ui,
                    &props.text,
                    SpanRenderDefaults::new(SpanBaseline::PlainText)
                        .color(style.text)
                        .size(style.text_size),
                    &self.theme,
                );

                if props.required {
                    label_ui.label(
                        egui::RichText::new("*")
                            .color(style.required_text)
                            .size(style.text_size),
                    );
                }
            })
            .response;

        if let Some(hover_text) = &props.hover_text {
            response.on_hover_text(hover_text.as_str());
        }
    }

    fn render_form_text(&mut self, ui: &mut egui::Ui, props: &FormTextProps) {
        let style = self
            .theme
            .resolve_form_text_style(props.variant, props.validation);

        if style.margin_top > 0.0 {
            ui.add_space(style.margin_top);
        }

        Self::render_span_freeform(
            ui,
            &props.text,
            SpanRenderDefaults::new(SpanBaseline::PlainText)
                .color(style.text)
                .size(style.text_size),
            &self.theme,
        );
    }

    fn render_form_control(
        &mut self,
        ui: &mut egui::Ui,
        value: &mut String,
        props: &FormControlProps,
    ) -> bool {
        Self::render_text_edit(ui, value, props).changed()
    }

    fn render_form_select(
        &mut self,
        ui: &mut egui::Ui,
        selected_index: &mut usize,
        props: &FormSelectProps,
    ) -> bool {
        let style =
            self.theme
                .resolve_form_control_style(props.variant, props.validation, props.disabled);
        let selected_text = props
            .selected_text(*selected_index)
            .cloned()
            .or_else(|| props.placeholder.clone())
            .unwrap_or_else(|| "Select...".into());
        let width = normalize_non_negative(props.width.unwrap_or(ui.available_width()));

        ui.add_enabled_ui(!props.disabled, |enabled_ui| {
            let mut changed = false;
            egui::ComboBox::from_id_salt(enabled_ui.next_auto_id())
                .width(width)
                .selected_text(Self::span_to_widget_text(
                    enabled_ui,
                    &selected_text,
                    SpanRenderDefaults::new(SpanBaseline::ControlLabel)
                        .color(style.text)
                        .size(style.text_size),
                ))
                .show_ui(enabled_ui, |options_ui| {
                    for (index, option) in props.options.iter().enumerate() {
                        let option_color = if option.disabled {
                            style.placeholder
                        } else {
                            style.text
                        };
                        let option_text = Self::span_to_widget_text(
                            options_ui,
                            &option.text,
                            SpanRenderDefaults::new(SpanBaseline::PlainText)
                                .color(option_color)
                                .size(style.text_size),
                        );

                        changed |= options_ui
                            .add_enabled_ui(!option.disabled, |item_ui| {
                                item_ui.selectable_value(selected_index, index, option_text)
                            })
                            .inner
                            .changed();
                    }
                });

            changed
        })
        .inner
    }

    fn render_form_check(
        &mut self,
        ui: &mut egui::Ui,
        checked: &mut bool,
        props: &FormCheckProps,
    ) -> bool {
        let style =
            self.theme
                .resolve_form_check_style(props.variant, props.validation, props.disabled);

        let text_color = if props.disabled {
            style.disabled_text
        } else {
            style.text
        };
        let label = Self::span_to_widget_text(
            ui,
            &props.label,
            SpanRenderDefaults::new(SpanBaseline::ControlLabel)
                .color(text_color)
                .size(style.text_size),
        );

        let response = match props.kind {
            FormCheckKind::Checkbox => {
                ui.add_enabled(!props.disabled, egui::Checkbox::new(checked, label))
            }
            FormCheckKind::Radio => {
                let mut response =
                    ui.add_enabled(!props.disabled, egui::RadioButton::new(*checked, label));
                if response.clicked() && !*checked {
                    *checked = true;
                    response.mark_changed();
                }
                response
            }
        };

        let changed = response.changed();
        if let Some(hover_text) = &props.hover_text {
            response.on_hover_text(hover_text.as_str());
        }

        changed
    }

    fn render_form_range(
        &mut self,
        ui: &mut egui::Ui,
        value: &mut f32,
        props: &FormRangeProps,
    ) -> bool {
        let style =
            self.theme
                .resolve_form_range_style(props.variant, props.validation, props.disabled);
        let tone = self.theme.resolve_variant(props.variant);
        let accent = match props.validation {
            FormValidationState::None => tone.fill,
            FormValidationState::Valid => self.theme.palette.form_valid,
            FormValidationState::Invalid => self.theme.palette.form_invalid,
        };

        let (min, max) = props.normalized_bounds();
        let step = props.resolved_step().map(f64::from);

        ui.scope(|scope_ui| {
            let available_width = scope_ui.available_width();
            {
                let spacing = scope_ui.spacing_mut();
                spacing.item_spacing.x = style.label_gap;
                spacing.interact_size.y = (spacing.interact_size.y * 0.6).max(8.0);
            }

            {
                let visuals = scope_ui.visuals_mut();
                let rounded = corner_radius(self.theme.radius.pill);
                let border = egui::Stroke::new(1.0_f32, tone.border);

                visuals.handle_shape = egui::style::HandleShape::Circle;

                visuals.widgets.inactive.bg_fill = tone.bg_subtle;
                visuals.widgets.inactive.weak_bg_fill = tone.bg_subtle;
                visuals.widgets.inactive.bg_stroke = border;
                visuals.widgets.inactive.fg_stroke.color = style.text;
                visuals.widgets.inactive.corner_radius = rounded;

                visuals.widgets.hovered.bg_fill = accent;
                visuals.widgets.hovered.weak_bg_fill = tone.bg_subtle;
                visuals.widgets.hovered.bg_stroke = border;
                visuals.widgets.hovered.fg_stroke.color = style.text;
                visuals.widgets.hovered.corner_radius = rounded;

                visuals.widgets.active.bg_fill = accent;
                visuals.widgets.active.weak_bg_fill = tone.bg_subtle;
                visuals.widgets.active.bg_stroke = border;
                visuals.widgets.active.fg_stroke.color = style.text;
                visuals.widgets.active.corner_radius = rounded;

                visuals.widgets.open.bg_fill = accent;
                visuals.widgets.open.weak_bg_fill = tone.bg_subtle;
                visuals.widgets.open.bg_stroke = border;
                visuals.widgets.open.fg_stroke.color = style.text;
                visuals.widgets.open.corner_radius = rounded;

                visuals.widgets.noninteractive.bg_fill = tone.bg_subtle;
                visuals.widgets.noninteractive.weak_bg_fill = tone.bg_subtle;
                visuals.widgets.noninteractive.bg_stroke = border;
                visuals.widgets.noninteractive.fg_stroke.color = style.disabled_text;
                visuals.widgets.noninteractive.corner_radius = rounded;
            }

            let mut slider = egui::Slider::new(value, min..=max).show_value(props.show_value);
            if let Some(step) = step {
                slider = slider.step_by(step);
            }

            let mut label_text = None;
            if let Some(label) = &props.label {
                let text_color = if props.disabled {
                    style.disabled_text
                } else {
                    style.text
                };
                let text = Self::span_to_widget_text(
                    scope_ui,
                    label,
                    SpanRenderDefaults::new(SpanBaseline::ControlLabel)
                        .color(text_color)
                        .size(style.text_size),
                );
                label_text = Some(text.clone());
                slider = slider.text(text);
            }

            // `egui::Slider` sizes its track from `Spacing::slider_width` and ignores the
            // size passed to `add_sized`, so derive the track width from the requested outer
            // width minus the value display and label.
            let outer_width = if props.fill_width {
                Some(available_width)
            } else {
                props.width.map(normalize_non_negative)
            };

            if let Some(outer_width) = outer_width {
                let item_gap = scope_ui.spacing().item_spacing.x;
                let mut reserved = 0.0;
                if props.show_value {
                    reserved += scope_ui.spacing().interact_size.x + item_gap;
                }
                if let Some(text) = label_text {
                    let galley = text.into_galley(
                        scope_ui,
                        Some(egui::TextWrapMode::Extend),
                        f32::INFINITY,
                        egui::TextStyle::Body,
                    );
                    reserved += galley.size().x + item_gap;
                }

                scope_ui.spacing_mut().slider_width = (outer_width - reserved).max(1.0);
            }

            scope_ui.add_enabled(!props.disabled, slider).changed()
        })
        .inner
    }

    fn render_floating_label(
        &mut self,
        ui: &mut egui::Ui,
        value: &mut String,
        props: &FloatingLabelProps,
    ) -> bool {
        let label_style = self
            .theme
            .resolve_form_label_style(props.variant, value.is_empty());
        let control_props = FormControlProps {
            placeholder: props.placeholder.clone(),
            variant: props.variant,
            validation: props.validation,
            disabled: props.disabled,
            width: props.width,
            password: props.password,
            on_change: None,
        };

        ui.vertical(|floating_ui| {
            Self::render_span_freeform(
                floating_ui,
                &props.label,
                SpanRenderDefaults::new(SpanBaseline::PlainText)
                    .color(label_style.text)
                    .size(label_style.text_size),
                &self.theme,
            );

            Self::render_text_edit(floating_ui, value, &control_props).changed()
        })
        .inner
    }

    fn render_validation_feedback(&mut self, ui: &mut egui::Ui, props: &ValidationFeedbackProps) {
        let style = self
            .theme
            .resolve_validation_feedback_style(props.variant, props.state);

        let prefix = match props.state {
            FormValidationState::None => "",
            FormValidationState::Valid => "✓ ",
            FormValidationState::Invalid => "⚠ ",
        };

        let mut feedback = Span::from(prefix);
        feedback.runs.extend(props.text.runs.clone());

        Self::render_span_freeform(
            ui,
            &feedback,
            SpanRenderDefaults::new(SpanBaseline::PlainText)
                .color(style.text)
                .size(style.text_size),
            &self.theme,
        );
    }
}

pub(crate) fn bootstrap_fixed_container_width(viewport_width: f32) -> f32 {
    let width = normalize_non_negative(viewport_width);

    if width >= 1_400.0 {
        1_320.0
    } else if width >= 1_200.0 {
        1_140.0
    } else if width >= 992.0 {
        960.0
    } else if width >= 768.0 {
        720.0
    } else if width >= 576.0 {
        540.0
    } else {
        width
    }
}

fn corner_radius(radius: u8) -> egui::CornerRadius {
    egui::CornerRadius::same(radius)
}

fn normalize_non_negative(value: f32) -> f32 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn as_i8(value: f32) -> i8 {
    value.round().clamp(f32::from(i8::MIN), f32::from(i8::MAX)) as i8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_container_width_uses_bootstrap_thresholds() {
        assert_eq!(bootstrap_fixed_container_width(575.0), 575.0);
        assert_eq!(bootstrap_fixed_container_width(576.0), 540.0);
        assert_eq!(bootstrap_fixed_container_width(768.0), 720.0);
        assert_eq!(bootstrap_fixed_container_width(992.0), 960.0);
        assert_eq!(bootstrap_fixed_container_width(1_200.0), 1_140.0);
        assert_eq!(bootstrap_fixed_container_width(1_400.0), 1_320.0);
    }

    #[test]
    fn container_width_respects_fluid_and_max_width() {
        let backend = EguiBackend::default();

        let fluid = ContainerProps::fluid().max_width(640.0);
        assert_eq!(backend.resolve_container_width(&fluid, 1_000.0), 640.0);

        let fixed = ContainerProps::fixed();
        assert_eq!(backend.resolve_container_width(&fixed, 1_000.0), 960.0);

        let fixed_capped = ContainerProps::fixed().max_width(800.0);
        assert_eq!(
            backend.resolve_container_width(&fixed_capped, 1_000.0),
            800.0
        );
    }

    #[test]
    fn backend_uses_theme_breakpoints() {
        let backend = EguiBackend::default();
        assert_eq!(backend.breakpoints(), Breakpoints::bootstrap());
    }

    #[test]
    fn span_to_button_supports_icons_and_text() {
        use crate::core::{RichText, SvgIcon};

        let ctx = egui::Context::default();
        let theme = UiTheme::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // Text only
                let span_text = RichText::new().text("Action");
                let _ = EguiBackend::span_to_button(
                    ui,
                    &span_text,
                    SpanRenderDefaults::new(SpanBaseline::ControlLabel),
                    &theme,
                );

                // Trailing icon
                let span_trailing = RichText::new()
                    .text("Action")
                    .icon(SpanIcon::new(SvgIcon::Flag));
                let _ = EguiBackend::span_to_button(
                    ui,
                    &span_trailing,
                    SpanRenderDefaults::new(SpanBaseline::ControlLabel),
                    &theme,
                );

                // Leading icon
                let span_leading = RichText::new()
                    .icon(SpanIcon::new(SvgIcon::Flag))
                    .text("Action");
                let _ = EguiBackend::span_to_button(
                    ui,
                    &span_leading,
                    SpanRenderDefaults::new(SpanBaseline::ControlLabel),
                    &theme,
                );

                // Icon only
                let span_icon_only = RichText::new().icon(SpanIcon::new(SvgIcon::Flag));
                let _ = EguiBackend::span_to_button(
                    ui,
                    &span_icon_only,
                    SpanRenderDefaults::new(SpanBaseline::ControlLabel),
                    &theme,
                );
            });
        });
    }
}
