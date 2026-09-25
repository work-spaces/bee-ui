"""
Build and run bee-ui cargo projects.
"""

load("//@star/prelude/rules/deps.star", "deps_new")
load("//@star/prelude/rules/rules.star", "rules_as_dep", "rules_as_rule", "rules_new")
load("//@star/prelude/rules/run.star", "run_add", "run_add_exec")

def cargo_add(name: str, working_directory: str = ".", rule_deps = []):
    rules = rules_new(
        name,
        [
            "fonts_icons",
            "run",
            "check",
            "build",
            "build_release",
            "clippy",
            "test",
            "format",
            "bootstrap_token_inventory",
        ],
    )

    run_add_exec(
        rules_as_rule(rules, "fonts_icons"),
        command = "./scripts/font-icon-preprocessor.exec.star",
        args = [
            "--icon-folder=crates/bee-ui/assets/icons",
            "--font-folder=crates/bee-ui/assets/fonts",
            "--icon-output=crates/bee-ui/assets/icons.rs",
            "--font-output=crates/bee-ui/assets/fonts.rs",
        ],
        deps = deps_new(
            rules = rule_deps,
            files = [
                "scripts/font-icon-preprocessor.exec.star",
                "crates/bee-ui/assets/icons/**",
                "crates/bee-ui/assets/fonts/**",
            ],
        ),
        target_files = [
            "crates/bee-ui/assets/icons.rs",
            "crates/bee-ui/assets/fonts.rs",
        ],
        working_directory = working_directory,
        help = "Generate bee-ui icon/font asset modules",
    )

    cargo_deps = deps_new(
        rules = [rules_as_dep(rules, "fonts_icons")],
        files = ["crates/**"],
    )

    run_add_exec(
        rules_as_rule(rules, "run"),
        command = "cargo",
        args = ["run", "--package", "bee-app"],
        working_directory = working_directory,
        deps = cargo_deps,
        help = "Run bee-ui demo application",
    )

    run_add_exec(
        rules_as_rule(rules, "check"),
        command = "cargo",
        args = ["check", "--workspace"],
        deps = cargo_deps,
        working_directory = working_directory,
        help = "cargo check for the bee-ui workspace",
    )

    run_add_exec(
        rules_as_rule(rules, "build"),
        command = "cargo",
        args = ["build", "--workspace"],
        deps = cargo_deps,
        working_directory = working_directory,
        help = "cargo build (dev profile) for the bee-ui workspace",
    )

    run_add_exec(
        rules_as_rule(rules, "build_release"),
        command = "cargo",
        args = ["build", "--workspace", "--release"],
        deps = cargo_deps,
        working_directory = working_directory,
        help = "cargo build (release profile) for the bee-ui workspace",
    )

    run_add_exec(
        rules_as_rule(rules, "clippy"),
        command = "cargo",
        args = ["clippy", "--workspace", "--all-targets"],
        deps = cargo_deps,
        working_directory = working_directory,
        help = "cargo clippy for the bee-ui workspace --",
        log_level = "Passthrough",
    )

    run_add_exec(
        rules_as_rule(rules, "test"),
        command = "cargo",
        args = ["test", "--workspace"],
        deps = cargo_deps,
        working_directory = working_directory,
        help = "cargo test for the bee-ui workspace",
    )

    run_add_exec(
        rules_as_rule(rules, "format"),
        command = "cargo",
        args = ["fmt", "--all", "--check"],
        deps = cargo_deps,
        working_directory = working_directory,
        help = "cargo fmt --check for the bee-ui workspace",
    )

    run_add_exec(
        rules_as_rule(rules, "bootstrap_token_inventory"),
        command = "cargo",
        args = ["run", "--package", "bee-ui", "--bin", "bootstrap_token_inventory"],
        working_directory = working_directory,
        deps = cargo_deps,
        help = "Generate the bee-ui Bootstrap token inventory report for Phase 1 parity",
    )

    run_add(
        name,
        deps = [
            rules_as_dep(rules, "check"),
        ],
    )

cargo_add("cargo")
