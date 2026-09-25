use runnerd_bs::prelude::*;

#[test]
fn semantic_api_supports_nested_bootstrap_like_composition() {
    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.container(Container::fluid(), |ui| {
            ui.row(Row::new().gutter_x(16.0).gutter_y(12.0), |ui| {
                ui.col(Col::new().md(8).sm(12), |ui| {
                    ui.card(Card::new().title("System Status"), |ui| {
                        ui.label(Label::new("Runner").muted(true));
                        ui.badge(Badge::success("Healthy").pill(true));
                        ui.button(Button::new("Restart").variant(Variant::Warning));
                        ui.button(
                            Button::new("Deploy")
                                .icon(SvgIcon::Flag)
                                .icon_after()
                                .variant(Variant::Primary),
                        );
                    });
                });

                ui.col(ColProps::new().md(4).sm(12), |ui| {
                    ui.card(CardProps::new().header("Metrics"), |ui| {
                        ui.label(LabelProps::new("Latency").size(LabelSize::Small));
                        ui.badge(BadgeProps::new("Live").variant(Variant::Info));
                    });
                });
            });
        });

        ui.container(ContainerProps::fixed().padding(Spacing::all(8.0)), |ui| {
            ui.row(RowProps::new().gutter_x(8.0), |ui| {
                ui.col(Col::new().xs(12), |ui| {
                    ui.label(Label::new("Footer"));
                });
            });
        });
    });
}

#[test]
fn semantic_api_supports_raw_egui_escape_hatches() {
    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.with_theme_variant(egui::Theme::Dark, |ui| {
            ui.with_style_override(
                |style| {
                    style.spacing.item_spacing = egui::vec2(2.0, 2.0);
                },
                |ui| {
                    ui.raw(|egui_ui| {
                        egui_ui.label("Raw egui label");
                    });

                    ui.badge(Badge::info("Interop"));
                },
            );
        });
    });
}

#[test]
fn semantic_api_supports_card_plot_wrappers() {
    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.card(Card::new().title("Latency"), |ui| {
            ui.plot(
                PlotArea::new("latency_plot")
                    .title("P95 Latency")
                    .axis_labels("Samples", "Milliseconds")
                    .height(180.0)
                    .legend(true),
                |_plot_ui| {
                    // compile-smoke: callsite intentionally empty.
                },
            );
        });

        ui.chart_card(
            ChartCard::new("CPU Usage", "cpu_plot")
                .variant(Variant::Info)
                .plot_height(200.0)
                .footer("Rolling 5m"),
            |_plot_ui| {
                // compile-smoke: callsite intentionally empty.
            },
        );
    });
}

#[test]
fn semantic_api_supports_mvp_dashboard_composition_smoke() {
    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.container(Container::fluid(), |ui| {
            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                ui.col(Col::new().xs(12).lg(8), |ui| {
                    ui.card(
                        Card::new()
                            .title("Build Health")
                            .variant(Variant::Success)
                            .footer("Updated just now"),
                        |ui| {
                            ui.label(Label::new("CI Queue").small().muted(true));
                            ui.badge(Badge::success("Passing").as_pill());
                            ui.plot(
                                PlotArea::new("build_health_plot").height(160.0),
                                |_plot_ui| {},
                            );
                        },
                    );
                });

                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.card(Card::new().title("Actions"), |ui| {
                        ui.button(Button::warning("Restart Workers"));
                        ui.button(Button::secondary("Drain Queue").disabled());
                    });
                });
            });
        });
    });
}

#[test]
fn semantic_row_smoke_allows_non_col_children_via_implicit_columns() {
    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.row(Row::new().gutter_x(8.0), |ui| {
            ui.badge(Badge::warning("No explicit col"));
            ui.label(Label::new("Still composes"));
        });
    });
}

#[test]
fn semantic_api_supports_phase_zero_state_scopes() {
    let ctx = egui::Context::default();
    let mut modal_open = true;
    let mut active_index = 0_usize;

    show_root(&ctx, |ui| {
        ui.with_disclosure_state(&mut modal_open, |ui| {
            ui.card(Card::new().title("Disclosure content"), |ui| {
                ui.label(Label::new("Only rendered while open"));
            });
        });

        ui.with_active_index_state(&mut active_index, |ui| {
            ui.badge(Badge::info("Active index scope"));
        });

        ui.with_disclosure_state(
            DisclosureState::uncontrolled("runnerd-bs.smoke.uncontrolled.disclosure", true),
            |ui| {
                ui.label(Label::new("Uncontrolled disclosure scope"));
            },
        );

        ui.with_active_index_state(
            ActiveIndexState::uncontrolled("runnerd-bs.smoke.uncontrolled.index", 1),
            |ui| {
                ui.badge(Badge::secondary("Uncontrolled index scope"));
            },
        );
    });
}

#[test]
fn semantic_api_supports_phase_one_component_family() {
    fn noop(_: &egui::Context) {}

    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.alert(Alert::warning("Bootstrap-style semantic alert"));
        ui.progress(
            Progress::new(65.0)
                .variant(Variant::Success)
                .with_percentage(),
        );
        ui.close_button(CloseButton::new().hover_text("Dismiss").on_click(noop));

        ui.pagination(
            Pagination::new()
                .item(PageItem::new("Previous"))
                .item(PageItem::new("1").current())
                .item(PageItem::new("2"))
                .item(PageItem::new("Next")),
        );

        ui.spinner(Spinner::new().label("Loading runners"));
        ui.placeholder(Placeholder::new().height(24.0));

        ui.button_group(
            ButtonGroup::new()
                .button(Button::primary("Start"))
                .button(Button::secondary("Stop")),
        );

        ui.list_group(
            ListGroup::new()
                .item(ListGroupItem::new("Overview").current())
                .item(ListGroupItem::new("Metrics"))
                .item(ListGroupItem::new("Logs").disabled(true)),
        );
    });
}

#[test]
fn semantic_api_supports_phase_two_navigation_components() {
    fn noop(_: &egui::Context) {}

    let ctx = egui::Context::default();
    let mut dropdown_open = true;

    show_root(&ctx, |ui| {
        ui.nav(
            Nav::new()
                .item(NavItem::new("Overview").current())
                .item(NavItem::new("Runners").on_click(noop))
                .item(NavItem::new("Settings").disabled(true))
                .variant(Variant::Info),
        );

        ui.navbar(
            Navbar::new()
                .brand("runnerd")
                .on_brand_click(noop)
                .item(NavItem::new("Home").current())
                .item(NavItem::new("Queues"))
                .variant(Variant::Dark),
        );

        ui.dropdown(
            &mut dropdown_open,
            Dropdown::new("Actions")
                .item(DropdownItem::new("Restart workers").on_click(noop))
                .item(DropdownItem::new("Drain queue"))
                .item(DropdownItem::new("Delete queue").disabled(true))
                .variant(Variant::Secondary),
        );

        ui.dropdown(
            DisclosureState::uncontrolled("runnerd-bs.smoke.phase-two.dropdown", false),
            Dropdown::new("More").item(DropdownItem::new("Diagnostics")),
        );
    });
}

#[test]
fn semantic_api_supports_phase_three_forms_components() {
    fn noop(_: &egui::Context) {}

    let ctx = egui::Context::default();
    let mut display_name = String::new();
    let mut role_index = 0_usize;
    let mut notify = false;
    let mut threshold = 0.5_f32;
    let mut email = String::new();

    show_root(&ctx, |ui| {
        ui.form_label(FormLabel::new("Display name").is_required());
        ui.form_control(
            &mut display_name,
            FormControl::new()
                .placeholder("Runner name")
                .invalid()
                .on_change(noop),
        );
        ui.form_text(FormText::new("Shown in runner dashboards."));

        ui.form_label(FormLabel::new("Role"));
        ui.form_select(
            &mut role_index,
            FormSelect::new()
                .option("Reader")
                .option("Operator")
                .option(FormSelectOption::new("Admin").disabled(true))
                .valid()
                .on_change(noop),
        );

        ui.form_check(
            &mut notify,
            FormCheck::checkbox("Send deployment notifications")
                .variant(Variant::Info)
                .on_change(noop),
        );

        ui.form_range(
            &mut threshold,
            FormRange::new(0.0, 1.0)
                .step(0.05)
                .label("Alert threshold")
                .valid()
                .on_change(noop),
        );

        ui.floating_label(
            &mut email,
            FloatingLabel::new("Contact email")
                .placeholder("ops@example.com")
                .variant(Variant::Secondary),
        );

        ui.validation_feedback(
            ValidationFeedback::invalid("Display name is required").variant(Variant::Danger),
        );
    });
}
