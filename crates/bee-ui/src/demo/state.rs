#![allow(dead_code)]

const LAST_ACTION_ID: &str = "runnerd_bs.demo.last_action";

const SIMULATED_PROGRESS_VALUE_ID: &str = "runnerd_bs.demo.progress.simulated_value";
const NAVIGATION_PAGE_ID: &str = "runnerd_bs.demo.navigation.page";

const SELECTED_CLUSTER_ID: &str = "runnerd_bs.demo.selection.cluster";
const SELECTED_SERVICE_ID: &str = "runnerd_bs.demo.selection.service";
const SELECTED_TABLE_ROW_ID: &str = "runnerd_bs.demo.selection.table_row";

const DROPDOWNS_CONTROLLED_OPEN_ID: &str = "runnerd_bs.demo.dropdowns.controlled_open";
const DROPDOWNS_PRIMARY_SELECTION_ID: &str = "runnerd_bs.demo.dropdowns.primary_selection";
const DROPDOWNS_SECONDARY_SELECTION_ID: &str = "runnerd_bs.demo.dropdowns.secondary_selection";

const FORM_CHANGE_SUMMARY_ID: &str = "runnerd_bs.demo.forms.change_summary";
const FORM_REQUESTER_EMAIL_ID: &str = "runnerd_bs.demo.forms.requester_email";
const FORM_RUNBOOK_PATH_ID: &str = "runnerd_bs.demo.forms.runbook_path";
const FORM_TARGET_CLUSTER_INDEX_ID: &str = "runnerd_bs.demo.forms.target_cluster_index";
const FORM_CANARY_PERCENT_ID: &str = "runnerd_bs.demo.forms.canary_percent";
const FORM_REQUIRES_DB_MIGRATION_ID: &str = "runnerd_bs.demo.forms.requires_db_migration";
const FORM_NOTIFY_ONCALL_ID: &str = "runnerd_bs.demo.forms.notify_oncall";
const FORM_FREEZE_APPROVED_ID: &str = "runnerd_bs.demo.forms.freeze_approved";
const FORM_LOGIN_USERNAME_ID: &str = "runnerd_bs.demo.forms.login.username";
const FORM_LOGIN_PASSWORD_ID: &str = "runnerd_bs.demo.forms.login.password";
const FORM_RANGE_VARIANT_PRIMARY_ID: &str = "runnerd_bs.demo.forms.range_variant.primary";
const FORM_RANGE_VARIANT_SECONDARY_ID: &str = "runnerd_bs.demo.forms.range_variant.secondary";
const FORM_RANGE_VARIANT_SUCCESS_ID: &str = "runnerd_bs.demo.forms.range_variant.success";
const FORM_RANGE_VARIANT_DANGER_ID: &str = "runnerd_bs.demo.forms.range_variant.danger";
const FORM_RANGE_VARIANT_WARNING_ID: &str = "runnerd_bs.demo.forms.range_variant.warning";
const FORM_RANGE_VARIANT_INFO_ID: &str = "runnerd_bs.demo.forms.range_variant.info";
const FORM_RANGE_VARIANT_LIGHT_ID: &str = "runnerd_bs.demo.forms.range_variant.light";
const FORM_RANGE_VARIANT_DARK_ID: &str = "runnerd_bs.demo.forms.range_variant.dark";

const MENUS_ACTIVE_NAV_ITEM_ID: &str = "runnerd_bs.demo.menus.active_nav_item";
const MENUS_ACTIVE_NAVBAR_ITEM_ID: &str = "runnerd_bs.demo.menus.active_navbar_item";

#[derive(Clone, Debug)]
pub(super) struct DemoState {
    pub dropdowns_controlled_open: bool,
    pub form_change_summary: String,
    pub form_requester_email: String,
    pub form_runbook_path: String,
    pub form_target_cluster_index: usize,
    pub form_canary_percent: f32,
    pub form_requires_db_migration: bool,
    pub form_notify_oncall: bool,
    pub form_freeze_approved: bool,
    pub form_login_username: String,
    pub form_login_password: String,
    pub form_range_variant_primary: f32,
    pub form_range_variant_secondary: f32,
    pub form_range_variant_success: f32,
    pub form_range_variant_danger: f32,
    pub form_range_variant_warning: f32,
    pub form_range_variant_info: f32,
    pub form_range_variant_light: f32,
    pub form_range_variant_dark: f32,
}

pub(super) fn load_demo_state(ctx: &egui::Context) -> DemoState {
    DemoState {
        dropdowns_controlled_open: dropdowns_controlled_open(ctx),
        form_change_summary: form_change_summary(ctx),
        form_requester_email: form_requester_email(ctx),
        form_runbook_path: form_runbook_path(ctx),
        form_target_cluster_index: form_target_cluster_index(ctx),
        form_canary_percent: form_canary_percent(ctx),
        form_requires_db_migration: form_requires_db_migration(ctx),
        form_notify_oncall: form_notify_oncall(ctx),
        form_freeze_approved: form_freeze_approved(ctx),
        form_login_username: form_login_username(ctx),
        form_login_password: form_login_password(ctx),
        form_range_variant_primary: form_range_variant_primary(ctx),
        form_range_variant_secondary: form_range_variant_secondary(ctx),
        form_range_variant_success: form_range_variant_success(ctx),
        form_range_variant_danger: form_range_variant_danger(ctx),
        form_range_variant_warning: form_range_variant_warning(ctx),
        form_range_variant_info: form_range_variant_info(ctx),
        form_range_variant_light: form_range_variant_light(ctx),
        form_range_variant_dark: form_range_variant_dark(ctx),
    }
}

pub(super) fn store_demo_state(ctx: &egui::Context, state: &DemoState) {
    set_dropdowns_controlled_open(ctx, state.dropdowns_controlled_open);
    set_form_change_summary(ctx, state.form_change_summary.clone());
    set_form_requester_email(ctx, state.form_requester_email.clone());
    set_form_runbook_path(ctx, state.form_runbook_path.clone());
    set_form_target_cluster_index(ctx, state.form_target_cluster_index);
    set_form_canary_percent(ctx, state.form_canary_percent);
    set_form_requires_db_migration(ctx, state.form_requires_db_migration);
    set_form_notify_oncall(ctx, state.form_notify_oncall);
    set_form_freeze_approved(ctx, state.form_freeze_approved);
    set_form_login_username(ctx, state.form_login_username.clone());
    set_form_login_password(ctx, state.form_login_password.clone());
    set_form_range_variant_primary(ctx, state.form_range_variant_primary);
    set_form_range_variant_secondary(ctx, state.form_range_variant_secondary);
    set_form_range_variant_success(ctx, state.form_range_variant_success);
    set_form_range_variant_danger(ctx, state.form_range_variant_danger);
    set_form_range_variant_warning(ctx, state.form_range_variant_warning);
    set_form_range_variant_info(ctx, state.form_range_variant_info);
    set_form_range_variant_light(ctx, state.form_range_variant_light);
    set_form_range_variant_dark(ctx, state.form_range_variant_dark);
}

fn persisted_f32(ctx: &egui::Context, id: &'static str, default: f32) -> f32 {
    ctx.data_mut(|data| {
        data.get_persisted::<f32>(egui::Id::new(id))
            .unwrap_or(default)
    })
}

fn set_persisted_f32(ctx: &egui::Context, id: &'static str, value: f32) {
    ctx.data_mut(|data| {
        data.insert_persisted(egui::Id::new(id), value);
    });
}

fn persisted_bool(ctx: &egui::Context, id: &'static str, default: bool) -> bool {
    ctx.data_mut(|data| {
        data.get_persisted::<bool>(egui::Id::new(id))
            .unwrap_or(default)
    })
}

fn set_persisted_bool(ctx: &egui::Context, id: &'static str, value: bool) {
    ctx.data_mut(|data| {
        data.insert_persisted(egui::Id::new(id), value);
    });
}

fn persisted_usize(ctx: &egui::Context, id: &'static str, default: usize) -> usize {
    ctx.data_mut(|data| {
        data.get_persisted::<usize>(egui::Id::new(id))
            .unwrap_or(default)
    })
}

fn set_persisted_usize(ctx: &egui::Context, id: &'static str, value: usize) {
    ctx.data_mut(|data| {
        data.insert_persisted(egui::Id::new(id), value);
    });
}

fn persisted_string(ctx: &egui::Context, id: &'static str, default: &str) -> String {
    ctx.data_mut(|data| {
        data.get_persisted::<String>(egui::Id::new(id))
            .unwrap_or_else(|| default.to_owned())
    })
}

fn set_persisted_string(ctx: &egui::Context, id: &'static str, value: impl Into<String>) {
    ctx.data_mut(|data| {
        data.insert_persisted(egui::Id::new(id), value.into());
    });
}

pub(super) fn last_action(ctx: &egui::Context) -> String {
    persisted_string(ctx, LAST_ACTION_ID, "No interactions yet")
}

pub(super) fn set_last_action(ctx: &egui::Context, action: impl Into<String>) {
    set_persisted_string(ctx, LAST_ACTION_ID, action);
}

pub(super) fn selected_cluster(ctx: &egui::Context) -> usize {
    persisted_usize(ctx, SELECTED_CLUSTER_ID, 0)
}

pub(super) fn set_selected_cluster(ctx: &egui::Context, index: usize) {
    // prod-a, prod-b, staging
    set_persisted_usize(ctx, SELECTED_CLUSTER_ID, index.min(2));
}

pub(super) fn selected_service(ctx: &egui::Context) -> usize {
    persisted_usize(ctx, SELECTED_SERVICE_ID, 0)
}

pub(super) fn set_selected_service(ctx: &egui::Context, index: usize) {
    // scheduler, api-gateway, worker
    set_persisted_usize(ctx, SELECTED_SERVICE_ID, index.min(2));
}

pub(super) fn selected_table_row(ctx: &egui::Context) -> usize {
    persisted_usize(ctx, SELECTED_TABLE_ROW_ID, 0)
}

pub(super) fn set_selected_table_row(ctx: &egui::Context, index: usize) {
    // table currently mirrors service rows
    set_persisted_usize(ctx, SELECTED_TABLE_ROW_ID, index.min(2));
}

pub(super) fn simulated_progress_value(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, SIMULATED_PROGRESS_VALUE_ID, 42.0)
}

pub(super) fn set_simulated_progress_value(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, SIMULATED_PROGRESS_VALUE_ID, value.clamp(0.0, 100.0));
}

pub(super) fn dropdowns_controlled_open(ctx: &egui::Context) -> bool {
    persisted_bool(ctx, DROPDOWNS_CONTROLLED_OPEN_ID, false)
}

pub(super) fn set_dropdowns_controlled_open(ctx: &egui::Context, open: bool) {
    set_persisted_bool(ctx, DROPDOWNS_CONTROLLED_OPEN_ID, open);
}

pub(super) fn dropdown_primary_selection(ctx: &egui::Context) -> String {
    persisted_string(ctx, DROPDOWNS_PRIMARY_SELECTION_ID, "None")
}

pub(super) fn set_dropdown_primary_selection(ctx: &egui::Context, item: impl Into<String>) {
    set_persisted_string(ctx, DROPDOWNS_PRIMARY_SELECTION_ID, item);
}

pub(super) fn dropdown_secondary_selection(ctx: &egui::Context) -> String {
    persisted_string(ctx, DROPDOWNS_SECONDARY_SELECTION_ID, "None")
}

pub(super) fn set_dropdown_secondary_selection(ctx: &egui::Context, item: impl Into<String>) {
    set_persisted_string(ctx, DROPDOWNS_SECONDARY_SELECTION_ID, item);
}

pub(super) fn form_change_summary(ctx: &egui::Context) -> String {
    persisted_string(
        ctx,
        FORM_CHANGE_SUMMARY_ID,
        "api-gateway canary rollout for latency mitigation",
    )
}

pub(super) fn set_form_change_summary(ctx: &egui::Context, value: impl Into<String>) {
    set_persisted_string(ctx, FORM_CHANGE_SUMMARY_ID, value);
}

pub(super) fn form_requester_email(ctx: &egui::Context) -> String {
    persisted_string(ctx, FORM_REQUESTER_EMAIL_ID, "oncall-sre@runnerd.dev")
}

pub(super) fn set_form_requester_email(ctx: &egui::Context, value: impl Into<String>) {
    set_persisted_string(ctx, FORM_REQUESTER_EMAIL_ID, value);
}

pub(super) fn form_runbook_path(ctx: &egui::Context) -> String {
    persisted_string(
        ctx,
        FORM_RUNBOOK_PATH_ID,
        "services/api-gateway/canary-rollout",
    )
}

pub(super) fn set_form_runbook_path(ctx: &egui::Context, value: impl Into<String>) {
    set_persisted_string(ctx, FORM_RUNBOOK_PATH_ID, value);
}

pub(super) fn form_target_cluster_index(ctx: &egui::Context) -> usize {
    persisted_usize(ctx, FORM_TARGET_CLUSTER_INDEX_ID, 1)
}

pub(super) fn set_form_target_cluster_index(ctx: &egui::Context, value: usize) {
    set_persisted_usize(ctx, FORM_TARGET_CLUSTER_INDEX_ID, value);
}

pub(super) fn form_canary_percent(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_CANARY_PERCENT_ID, 25.0)
}

pub(super) fn set_form_canary_percent(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_CANARY_PERCENT_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_requires_db_migration(ctx: &egui::Context) -> bool {
    persisted_bool(ctx, FORM_REQUIRES_DB_MIGRATION_ID, false)
}

pub(super) fn set_form_requires_db_migration(ctx: &egui::Context, value: bool) {
    set_persisted_bool(ctx, FORM_REQUIRES_DB_MIGRATION_ID, value);
}

pub(super) fn form_notify_oncall(ctx: &egui::Context) -> bool {
    persisted_bool(ctx, FORM_NOTIFY_ONCALL_ID, true)
}

pub(super) fn set_form_notify_oncall(ctx: &egui::Context, value: bool) {
    set_persisted_bool(ctx, FORM_NOTIFY_ONCALL_ID, value);
}

pub(super) fn form_freeze_approved(ctx: &egui::Context) -> bool {
    persisted_bool(ctx, FORM_FREEZE_APPROVED_ID, false)
}

pub(super) fn set_form_freeze_approved(ctx: &egui::Context, value: bool) {
    set_persisted_bool(ctx, FORM_FREEZE_APPROVED_ID, value);
}

pub(super) fn form_login_username(ctx: &egui::Context) -> String {
    persisted_string(ctx, FORM_LOGIN_USERNAME_ID, "ops-admin")
}

pub(super) fn set_form_login_username(ctx: &egui::Context, value: impl Into<String>) {
    set_persisted_string(ctx, FORM_LOGIN_USERNAME_ID, value);
}

pub(super) fn form_login_password(ctx: &egui::Context) -> String {
    persisted_string(ctx, FORM_LOGIN_PASSWORD_ID, "")
}

pub(super) fn set_form_login_password(ctx: &egui::Context, value: impl Into<String>) {
    set_persisted_string(ctx, FORM_LOGIN_PASSWORD_ID, value);
}

pub(super) fn form_range_variant_primary(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_PRIMARY_ID, 25.0)
}

pub(super) fn set_form_range_variant_primary(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_PRIMARY_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_range_variant_secondary(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_SECONDARY_ID, 25.0)
}

pub(super) fn set_form_range_variant_secondary(ctx: &egui::Context, value: f32) {
    set_persisted_f32(
        ctx,
        FORM_RANGE_VARIANT_SECONDARY_ID,
        value.clamp(0.0, 100.0),
    );
}

pub(super) fn form_range_variant_success(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_SUCCESS_ID, 25.0)
}

pub(super) fn set_form_range_variant_success(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_SUCCESS_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_range_variant_danger(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_DANGER_ID, 25.0)
}

pub(super) fn set_form_range_variant_danger(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_DANGER_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_range_variant_warning(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_WARNING_ID, 25.0)
}

pub(super) fn set_form_range_variant_warning(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_WARNING_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_range_variant_info(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_INFO_ID, 25.0)
}

pub(super) fn set_form_range_variant_info(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_INFO_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_range_variant_light(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_LIGHT_ID, 25.0)
}

pub(super) fn set_form_range_variant_light(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_LIGHT_ID, value.clamp(0.0, 100.0));
}

pub(super) fn form_range_variant_dark(ctx: &egui::Context) -> f32 {
    persisted_f32(ctx, FORM_RANGE_VARIANT_DARK_ID, 25.0)
}

pub(super) fn set_form_range_variant_dark(ctx: &egui::Context, value: f32) {
    set_persisted_f32(ctx, FORM_RANGE_VARIANT_DARK_ID, value.clamp(0.0, 100.0));
}

pub(super) fn navigation_page(ctx: &egui::Context) -> usize {
    persisted_usize(ctx, NAVIGATION_PAGE_ID, 1)
}

pub(super) fn set_navigation_page(ctx: &egui::Context, page: usize) {
    set_persisted_usize(ctx, NAVIGATION_PAGE_ID, page.max(1));
}

pub(super) fn menus_active_nav_item(ctx: &egui::Context) -> String {
    persisted_string(ctx, MENUS_ACTIVE_NAV_ITEM_ID, "None")
}

pub(super) fn set_menus_active_nav_item(ctx: &egui::Context, item: impl Into<String>) {
    set_persisted_string(ctx, MENUS_ACTIVE_NAV_ITEM_ID, item);
}

pub(super) fn menus_active_navbar_item(ctx: &egui::Context) -> String {
    persisted_string(ctx, MENUS_ACTIVE_NAVBAR_ITEM_ID, "None")
}

pub(super) fn set_menus_active_navbar_item(ctx: &egui::Context, item: impl Into<String>) {
    set_persisted_string(ctx, MENUS_ACTIVE_NAVBAR_ITEM_ID, item);
}

// Compatibility aliases while old temporary views are still wired.
pub(super) fn feedback_progress_value(ctx: &egui::Context) -> f32 {
    simulated_progress_value(ctx)
}

pub(super) fn set_feedback_progress_value(ctx: &egui::Context, value: f32) {
    set_simulated_progress_value(ctx, value);
}

pub(super) fn groups_active_list_item(ctx: &egui::Context) -> usize {
    selected_service(ctx)
}

pub(super) fn set_groups_active_list_item(ctx: &egui::Context, index: usize) {
    set_selected_service(ctx, index);
}

pub(super) fn menus_controlled_dropdown_open(ctx: &egui::Context) -> bool {
    dropdowns_controlled_open(ctx)
}

pub(super) fn set_menus_controlled_dropdown_open(ctx: &egui::Context, open: bool) {
    set_dropdowns_controlled_open(ctx, open);
}

pub(super) fn menus_selected_item(ctx: &egui::Context) -> String {
    dropdown_primary_selection(ctx)
}

pub(super) fn set_menus_selected_item(ctx: &egui::Context, item: impl Into<String>) {
    set_dropdown_primary_selection(ctx, item);
}

pub(super) fn feedback_advance_progress(ctx: &egui::Context) {
    let current = feedback_progress_value(ctx);
    let next = if current >= 100.0 {
        0.0
    } else {
        current + 10.0
    };
    set_feedback_progress_value(ctx, next);
    set_last_action(ctx, format!("Progress: rollout advanced to {:.0}%", next));
}

pub(super) fn cluster_prod_a_clicked(ctx: &egui::Context) {
    set_selected_cluster(ctx, 0);
    set_last_action(ctx, "Cluster selected: prod-a");
}

pub(super) fn cluster_prod_b_clicked(ctx: &egui::Context) {
    set_selected_cluster(ctx, 1);
    set_last_action(ctx, "Cluster selected: prod-b");
}

pub(super) fn cluster_staging_clicked(ctx: &egui::Context) {
    set_selected_cluster(ctx, 2);
    set_last_action(ctx, "Cluster selected: staging");
}

pub(super) fn service_scheduler_clicked(ctx: &egui::Context) {
    set_selected_service(ctx, 0);
    set_last_action(ctx, "Service selected: scheduler");
}

pub(super) fn service_api_gateway_clicked(ctx: &egui::Context) {
    set_selected_service(ctx, 1);
    set_last_action(ctx, "Service selected: api-gateway");
}

pub(super) fn service_worker_clicked(ctx: &egui::Context) {
    set_selected_service(ctx, 2);
    set_last_action(ctx, "Service selected: worker");
}

pub(super) fn pagination_page_1_clicked(ctx: &egui::Context) {
    set_navigation_page(ctx, 1);
    set_last_action(ctx, "Pagination: page 1 selected");
}

pub(super) fn pagination_page_2_clicked(ctx: &egui::Context) {
    set_navigation_page(ctx, 2);
    set_last_action(ctx, "Pagination: page 2 selected");
}

pub(super) fn pagination_page_3_clicked(ctx: &egui::Context) {
    set_navigation_page(ctx, 3);
    set_last_action(ctx, "Pagination: page 3 selected");
}

pub(super) fn close_button_default_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "CloseButton: default close clicked");
}

pub(super) fn close_button_custom_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "CloseButton: custom dismiss clicked");
}

pub(super) fn groups_button_deploy_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "ButtonGroup: Deploy clicked");
}

pub(super) fn groups_button_pause_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "ButtonGroup: Pause clicked");
}

pub(super) fn groups_button_rollback_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "ButtonGroup: Rollback clicked");
}

pub(super) fn groups_button_logs_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "ButtonGroup (vertical): Overview clicked");
}

pub(super) fn groups_button_metrics_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "ButtonGroup (vertical): Logs clicked");
}

pub(super) fn groups_button_events_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "ButtonGroup (vertical): Events clicked");
}

pub(super) fn list_group_overview_clicked(ctx: &egui::Context) {
    set_groups_active_list_item(ctx, 0);
    set_last_action(ctx, "ListGroup: Overview selected");
}

pub(super) fn list_group_replicas_clicked(ctx: &egui::Context) {
    set_groups_active_list_item(ctx, 1);
    set_last_action(ctx, "ListGroup: Logs selected");
}

pub(super) fn list_group_rollout_clicked(ctx: &egui::Context) {
    set_groups_active_list_item(ctx, 2);
    set_last_action(ctx, "ListGroup: Events selected");
}

pub(super) fn nav_overview_clicked(ctx: &egui::Context) {
    set_menus_active_nav_item(ctx, "Overview");
    set_last_action(ctx, "Nav: Overview clicked");
}

pub(super) fn nav_workloads_clicked(ctx: &egui::Context) {
    set_menus_active_nav_item(ctx, "Workloads");
    set_last_action(ctx, "Nav: Workloads clicked");
}

pub(super) fn nav_settings_clicked(ctx: &egui::Context) {
    set_menus_active_nav_item(ctx, "Settings");
    set_last_action(ctx, "Nav: Settings clicked");
}

pub(super) fn navbar_brand_clicked(ctx: &egui::Context) {
    set_last_action(ctx, "Navbar: brand clicked");
}

pub(super) fn navbar_dashboard_clicked(ctx: &egui::Context) {
    set_menus_active_navbar_item(ctx, "Dashboard");
    set_last_action(ctx, "Navbar: Dashboard clicked");
}

pub(super) fn navbar_alerts_clicked(ctx: &egui::Context) {
    set_menus_active_navbar_item(ctx, "Alerts");
    set_last_action(ctx, "Navbar: Alerts clicked");
}

pub(super) fn navbar_help_clicked(ctx: &egui::Context) {
    set_menus_active_navbar_item(ctx, "Help");
    set_last_action(ctx, "Navbar: Help clicked");
}
