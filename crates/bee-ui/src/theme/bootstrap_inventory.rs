use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Write,
};

/// Phase 1 target scope: parity for already-implemented component families and
/// high-impact global token prefixes.
pub const PHASE1_TARGET_PREFIXES: [&str; 17] = [
    "btn",
    "alert",
    "badge",
    "card",
    "dropdown",
    "nav",
    "navbar",
    "pagination",
    "list-group",
    "form",
    "table",
    "progress",
    "spinner",
    "body",
    "border",
    "focus",
    "link",
];

const TOKEN_PREFIX: &str = "--bs-";

// Longest multi-segment prefixes first so they win before short prefixes.
// Keep this list minimal so broad families (for example `form-*`) naturally
// collapse to first-segment coverage (`form`) in the fallback branch.
const COVERAGE_PREFIX_MATCHERS: [&str; 8] = [
    "list-group",
    "pagination",
    "dropdown",
    "progress",
    "spinner",
    "navbar",
    "table",
    "alert",
];

// Bootstrap token names currently represented in runnerd-bs token/theme model.
const MAPPED_BOOTSTRAP_TOKEN_NAMES: &[&str] = &[
    "--bs-alert-border-radius",
    "--bs-border-color",
    "--bs-border-color-translucent",
    "--bs-border-radius",
    "--bs-border-radius-2xl",
    "--bs-border-radius-lg",
    "--bs-border-radius-pill",
    "--bs-border-radius-sm",
    "--bs-border-radius-xl",
    "--bs-border-radius-xxl",
    "--bs-border-width",
    "--bs-body-bg",
    "--bs-body-bg-rgb",
    "--bs-body-color",
    "--bs-body-color-rgb",
    "--bs-body-font-family",
    "--bs-body-font-size",
    "--bs-body-font-weight",
    "--bs-body-line-height",
    "--bs-box-shadow",
    "--bs-box-shadow-inset",
    "--bs-box-shadow-lg",
    "--bs-box-shadow-sm",
    "--bs-btn-padding-x",
    "--bs-btn-padding-y",
    "--bs-dark",
    "--bs-dark-bg-subtle",
    "--bs-dark-border-subtle",
    "--bs-dark-text-emphasis",
    "--bs-danger",
    "--bs-danger-bg-subtle",
    "--bs-danger-border-subtle",
    "--bs-danger-text-emphasis",
    "--bs-dropdown-border-radius",
    "--bs-emphasis-color",
    "--bs-emphasis-color-rgb",
    "--bs-focus-ring-color",
    "--bs-focus-ring-opacity",
    "--bs-focus-ring-width",
    "--bs-font-monospace",
    "--bs-font-sans-serif",
    "--bs-form-invalid-color",
    "--bs-form-valid-color",
    "--bs-info",
    "--bs-info-bg-subtle",
    "--bs-info-border-subtle",
    "--bs-info-text-emphasis",
    "--bs-input-group-addon-padding-x",
    "--bs-input-group-addon-padding-y",
    "--bs-light",
    "--bs-light-bg-subtle",
    "--bs-light-border-subtle",
    "--bs-light-text-emphasis",
    "--bs-link-color",
    "--bs-link-color-rgb",
    "--bs-link-hover-color",
    "--bs-link-hover-color-rgb",
    "--bs-nav-link-padding-x",
    "--bs-nav-link-padding-y",
    "--bs-pagination-border-radius",
    "--bs-primary",
    "--bs-primary-bg-subtle",
    "--bs-primary-border-subtle",
    "--bs-primary-text-emphasis",
    "--bs-secondary",
    "--bs-secondary-bg",
    "--bs-secondary-bg-rgb",
    "--bs-secondary-bg-subtle",
    "--bs-secondary-border-subtle",
    "--bs-secondary-color",
    "--bs-secondary-color-rgb",
    "--bs-secondary-text-emphasis",
    "--bs-spinner-animation-speed",
    "--bs-spinner-border-width",
    "--bs-spinner-height",
    "--bs-spinner-width",
    "--bs-tertiary-bg",
    "--bs-tertiary-bg-rgb",
    "--bs-tertiary-color",
    "--bs-tertiary-color-rgb",
    "--bs-alert-border-color",
    "--bs-alert-margin-bottom",
    "--bs-alert-padding-x",
    "--bs-alert-padding-y",
    "--bs-badge-border-radius",
    "--bs-badge-font-size",
    "--bs-badge-font-weight",
    "--bs-badge-padding-x",
    "--bs-badge-padding-y",
    "--bs-btn-border-radius",
    "--bs-btn-disabled-opacity",
    "--bs-btn-font-size",
    "--bs-btn-font-weight",
    "--bs-card-border-color",
    "--bs-card-cap-bg",
    "--bs-card-cap-padding-x",
    "--bs-card-cap-padding-y",
    "--bs-card-spacer-x",
    "--bs-card-spacer-y",
    "--bs-dropdown-bg",
    "--bs-dropdown-border-color",
    "--bs-dropdown-box-shadow",
    "--bs-dropdown-divider-bg",
    "--bs-dropdown-item-padding-x",
    "--bs-dropdown-item-padding-y",
    "--bs-dropdown-link-active-bg",
    "--bs-dropdown-link-disabled-color",
    "--bs-dropdown-link-hover-bg",
    "--bs-dropdown-padding-x",
    "--bs-dropdown-padding-y",
    "--bs-form-check-disabled-opacity",
    "--bs-form-text-font-size",
    "--bs-form-text-margin-top",
    "--bs-input-border-color",
    "--bs-input-border-radius",
    "--bs-input-border-width",
    "--bs-input-disabled-bg",
    "--bs-input-focus-border-color",
    "--bs-input-focus-width",
    "--bs-input-font-size",
    "--bs-input-font-weight",
    "--bs-input-padding-x",
    "--bs-input-padding-y",
    "--bs-input-placeholder-color",
    "--bs-input-group-addon-color",
    "--bs-input-group-addon-bg",
    "--bs-input-group-addon-border-color",
    "--bs-list-group-action-active-bg",
    "--bs-list-group-action-hover-bg",
    "--bs-list-group-active-bg",
    "--bs-list-group-active-border-color",
    "--bs-list-group-active-color",
    "--bs-list-group-bg",
    "--bs-list-group-border-color",
    "--bs-list-group-border-radius",
    "--bs-list-group-border-width",
    "--bs-list-group-color",
    "--bs-list-group-disabled-bg",
    "--bs-list-group-disabled-color",
    "--bs-list-group-item-padding-x",
    "--bs-list-group-item-padding-y",
    "--bs-nav-link-color",
    "--bs-nav-link-disabled-color",
    "--bs-navbar-active-color",
    "--bs-navbar-disabled-color",
    "--bs-navbar-nav-link-padding-x",
    "--bs-navbar-padding-x",
    "--bs-navbar-padding-y",
    "--bs-pagination-active-bg",
    "--bs-pagination-active-border-color",
    "--bs-pagination-active-color",
    "--bs-pagination-bg",
    "--bs-pagination-border-color",
    "--bs-pagination-border-width",
    "--bs-pagination-color",
    "--bs-pagination-disabled-bg",
    "--bs-pagination-disabled-border-color",
    "--bs-pagination-disabled-color",
    "--bs-pagination-focus-bg",
    "--bs-pagination-focus-box-shadow",
    "--bs-pagination-focus-color",
    "--bs-pagination-font-size",
    "--bs-pagination-hover-bg",
    "--bs-pagination-hover-border-color",
    "--bs-pagination-hover-color",
    "--bs-pagination-padding-x",
    "--bs-pagination-padding-y",
    "--bs-progress-bar-bg",
    "--bs-progress-bar-color",
    "--bs-progress-font-size",
    "--bs-progress-height",
    "--bs-table-active-bg",
    "--bs-table-bg",
    "--bs-table-border-color",
    "--bs-table-color",
    "--bs-table-hover-bg",
    "--bs-table-striped-bg",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixCoverage {
    pub prefix: String,
    pub total: usize,
    pub mapped: usize,
}

impl PrefixCoverage {
    pub const fn unmapped(&self) -> usize {
        self.total - self.mapped
    }

    pub fn coverage_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.mapped as f32 / self.total as f32) * 100.0
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapTokenInventory {
    pub all_tokens: Vec<String>,
    pub mapped_tokens: Vec<String>,
    pub unmapped_tokens: Vec<String>,
    pub unmapped_target_scope_tokens: Vec<String>,
    pub coverage_by_prefix: Vec<PrefixCoverage>,
}

pub fn collect_bootstrap_token_inventory(css: &str) -> BootstrapTokenInventory {
    let all_tokens = collect_bootstrap_token_names(css);
    let mapped_set: HashSet<&str> = MAPPED_BOOTSTRAP_TOKEN_NAMES.iter().copied().collect();

    let mapped_tokens = all_tokens
        .iter()
        .filter(|token| mapped_set.contains(token.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    let unmapped_tokens = all_tokens
        .iter()
        .filter(|token| !mapped_set.contains(token.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    let unmapped_target_scope_tokens = unmapped_tokens
        .iter()
        .filter(|token| is_phase1_target_scope_token(token))
        .cloned()
        .collect::<Vec<_>>();

    let mut coverage_by_prefix = BTreeMap::<String, (usize, usize)>::new();
    for token in &all_tokens {
        let prefix = coverage_prefix(token);
        let entry = coverage_by_prefix.entry(prefix).or_default();
        entry.0 += 1;
        if mapped_set.contains(token.as_str()) {
            entry.1 += 1;
        }
    }

    let coverage_by_prefix = coverage_by_prefix
        .into_iter()
        .map(|(prefix, (total, mapped))| PrefixCoverage {
            prefix,
            total,
            mapped,
        })
        .collect();

    BootstrapTokenInventory {
        all_tokens,
        mapped_tokens,
        unmapped_tokens,
        unmapped_target_scope_tokens,
        coverage_by_prefix,
    }
}

pub fn render_phase1_bootstrap_inventory_report(css: &str) -> String {
    let inventory = collect_bootstrap_token_inventory(css);
    render_phase1_bootstrap_inventory_report_from_inventory(&inventory)
}

pub fn render_phase1_bootstrap_inventory_report_from_inventory(
    inventory: &BootstrapTokenInventory,
) -> String {
    let mut out = String::new();

    let _ = writeln!(
        out,
        "# runnerd-bs Bootstrap token inventory (Phase 1 baseline)"
    );
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "- total `--bs-*` tokens discovered: **{}**",
        inventory.all_tokens.len()
    );
    let _ = writeln!(
        out,
        "- tokens currently mapped in runnerd-bs: **{}**",
        inventory.mapped_tokens.len()
    );
    let _ = writeln!(
        out,
        "- unmapped tokens: **{}**",
        inventory.unmapped_tokens.len()
    );
    let _ = writeln!(
        out,
        "- unmapped tokens inside Phase 1 target scope: **{}**",
        inventory.unmapped_target_scope_tokens.len()
    );
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "## Phase 1 target scope prefixes\n{}",
        PHASE1_TARGET_PREFIXES
            .iter()
            .map(|prefix| format!("`{prefix}`"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "## Coverage by target prefix");
    let _ = writeln!(out, "| Prefix | Total | Mapped | Unmapped | Coverage |",);
    let _ = writeln!(out, "| --- | ---: | ---: | ---: | ---: |");
    for prefix in PHASE1_TARGET_PREFIXES {
        let coverage = inventory
            .coverage_by_prefix
            .iter()
            .find(|row| row.prefix == prefix)
            .cloned()
            .unwrap_or(PrefixCoverage {
                prefix: prefix.to_owned(),
                total: 0,
                mapped: 0,
            });

        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {:.1}% |",
            coverage.prefix,
            coverage.total,
            coverage.mapped,
            coverage.unmapped(),
            coverage.coverage_percent()
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "## Unmapped tokens in Phase 1 target scope ({})",
        inventory.unmapped_target_scope_tokens.len()
    );
    for token in &inventory.unmapped_target_scope_tokens {
        let _ = writeln!(out, "- `{token}`");
    }
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "## All discovered `--bs-*` tokens ({})",
        inventory.all_tokens.len()
    );
    for token in &inventory.all_tokens {
        let _ = writeln!(out, "- `{token}`");
    }

    out
}

fn collect_bootstrap_token_names(css: &str) -> Vec<String> {
    let mut tokens = BTreeSet::new();
    let bytes = css.as_bytes();

    let mut cursor = 0;
    while let Some(found) = css[cursor..].find(TOKEN_PREFIX) {
        let start = cursor + found;
        let mut end = start + TOKEN_PREFIX.len();

        while end < bytes.len() {
            let byte = bytes[end];
            if byte.is_ascii_alphanumeric() || byte == b'-' {
                end += 1;
            } else {
                break;
            }
        }

        if end > start + TOKEN_PREFIX.len() {
            tokens.insert(css[start..end].to_owned());
        }

        cursor = end;
    }

    tokens.into_iter().collect()
}

fn is_phase1_target_scope_token(token: &str) -> bool {
    let prefix = coverage_prefix(token);
    PHASE1_TARGET_PREFIXES.contains(&prefix.as_str())
}

fn coverage_prefix(token: &str) -> String {
    let Some(name) = token.strip_prefix(TOKEN_PREFIX) else {
        return token.to_owned();
    };

    for prefix in COVERAGE_PREFIX_MATCHERS {
        if name == prefix || name.starts_with(&format!("{prefix}-")) {
            return prefix.to_owned();
        }
    }

    name.split('-').next().unwrap_or(name).to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOOTSTRAP_CSS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../bootstrap/dist/css/bootstrap.css"
    ));

    #[test]
    fn finds_bootstrap_token_names_from_css() {
        let tokens = collect_bootstrap_token_names(BOOTSTRAP_CSS);
        assert!(tokens.len() > 400);
        assert!(tokens.contains(&"--bs-btn-padding-x".to_owned()));
        assert!(tokens.contains(&"--bs-card-border-radius".to_owned()));
        assert!(tokens.contains(&"--bs-dropdown-link-hover-bg".to_owned()));
    }

    #[test]
    fn builds_phase1_inventory_report_with_expected_sections() {
        let report = render_phase1_bootstrap_inventory_report(BOOTSTRAP_CSS);

        assert!(report.contains("# runnerd-bs Bootstrap token inventory (Phase 1 baseline)"));
        assert!(report.contains("## Coverage by target prefix"));
        assert!(report.contains("## Unmapped tokens in Phase 1 target scope"));
        assert!(report.contains("## All discovered `--bs-*` tokens"));
    }

    #[test]
    fn mapped_and_unmapped_sets_are_both_non_empty() {
        let inventory = collect_bootstrap_token_inventory(BOOTSTRAP_CSS);

        assert!(!inventory.mapped_tokens.is_empty());
        assert!(!inventory.unmapped_tokens.is_empty());
        assert!(!inventory.unmapped_target_scope_tokens.is_empty());
    }
}
