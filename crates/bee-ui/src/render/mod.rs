use crate::{
    components::{
        AlertProps, BadgeProps, Button, ButtonGroupProps, CardProps, CloseButtonProps, ColProps,
        ContainerProps, DropdownProps, FloatingLabelProps, FormCheckProps, FormControlProps,
        FormLabelProps, FormRangeProps, FormSelectProps, FormTextProps, LabelProps, ListGroupProps,
        NavProps, NavbarProps, PaginationProps, PlaceholderProps, ProgressProps, RowProps,
        SpinnerProps, TableProps, TextProps, ValidationFeedbackProps,
    },
    layout::{ComputedCol, ComputedLine},
};

pub mod egui_backend;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DropdownRenderResult {
    pub toggle_clicked: bool,
    pub clicked_item: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NavbarRenderResult {
    pub brand_clicked: bool,
    pub clicked_items: Vec<usize>,
}

pub(crate) trait Backend {
    fn begin_container(&mut self, ui: &mut egui::Ui, props: &ContainerProps);
    fn end_container(&mut self, ui: &mut egui::Ui, props: &ContainerProps);

    fn begin_row(&mut self, ui: &mut egui::Ui, props: &RowProps, line: &ComputedLine);
    fn end_row(&mut self, ui: &mut egui::Ui, props: &RowProps, line: &ComputedLine);

    fn begin_col(&mut self, ui: &mut egui::Ui, props: &ColProps, computed: &ComputedCol);
    fn end_col(&mut self, ui: &mut egui::Ui, props: &ColProps, computed: &ComputedCol);

    fn begin_disclosure_scope(&mut self, ui: &mut egui::Ui, open: bool);
    fn end_disclosure_scope(&mut self, ui: &mut egui::Ui, open: bool);

    fn begin_active_index_scope(&mut self, ui: &mut egui::Ui, active_index: usize);
    fn end_active_index_scope(&mut self, ui: &mut egui::Ui, active_index: usize);

    fn render_card<R, F>(&mut self, ui: &mut egui::Ui, props: &CardProps, body: F) -> R
    where
        F: FnOnce(&mut egui::Ui) -> R;

    fn render_badge(&mut self, ui: &mut egui::Ui, props: &BadgeProps);

    fn render_label(&mut self, ui: &mut egui::Ui, props: &LabelProps);

    fn render_text(&mut self, ui: &mut egui::Ui, props: &TextProps);

    fn render_button(&mut self, ui: &mut egui::Ui, button: &Button) -> egui::Response;

    fn render_alert(&mut self, ui: &mut egui::Ui, props: &AlertProps);

    fn render_progress(&mut self, ui: &mut egui::Ui, props: &ProgressProps);

    fn render_table(&mut self, ui: &mut egui::Ui, props: &TableProps, id_salt: u64) -> Vec<usize>;

    fn render_close_button(
        &mut self,
        ui: &mut egui::Ui,
        props: &CloseButtonProps,
    ) -> egui::Response;

    fn render_pagination(&mut self, ui: &mut egui::Ui, props: &PaginationProps) -> Vec<usize>;

    fn render_spinner(&mut self, ui: &mut egui::Ui, props: &SpinnerProps);

    fn render_placeholder(&mut self, ui: &mut egui::Ui, props: &PlaceholderProps);

    fn render_button_group(&mut self, ui: &mut egui::Ui, props: &ButtonGroupProps) -> Vec<usize>;

    fn render_list_group(&mut self, ui: &mut egui::Ui, props: &ListGroupProps) -> Vec<usize>;

    fn render_nav(&mut self, ui: &mut egui::Ui, props: &NavProps) -> Vec<usize>;

    fn render_navbar(&mut self, ui: &mut egui::Ui, props: &NavbarProps) -> NavbarRenderResult;

    fn render_dropdown(
        &mut self,
        ui: &mut egui::Ui,
        props: &DropdownProps,
        open: bool,
    ) -> DropdownRenderResult;

    fn render_form_label(&mut self, ui: &mut egui::Ui, props: &FormLabelProps);

    fn render_form_text(&mut self, ui: &mut egui::Ui, props: &FormTextProps);

    fn render_form_control(
        &mut self,
        ui: &mut egui::Ui,
        value: &mut String,
        props: &FormControlProps,
    ) -> bool;

    fn render_form_select(
        &mut self,
        ui: &mut egui::Ui,
        selected_index: &mut usize,
        props: &FormSelectProps,
    ) -> bool;

    fn render_form_check(
        &mut self,
        ui: &mut egui::Ui,
        checked: &mut bool,
        props: &FormCheckProps,
    ) -> bool;

    fn render_form_range(
        &mut self,
        ui: &mut egui::Ui,
        value: &mut f32,
        props: &FormRangeProps,
    ) -> bool;

    fn render_floating_label(
        &mut self,
        ui: &mut egui::Ui,
        value: &mut String,
        props: &FloatingLabelProps,
    ) -> bool;

    fn render_validation_feedback(&mut self, ui: &mut egui::Ui, props: &ValidationFeedbackProps);
}

pub use egui_backend::EguiBackend;
