use bee_ui::plot::{clamp_x_window_to_limits, compute_data_bounds, compute_x_navigation_limits};
use bee_ui::prelude::*;

#[test]
fn semantic_api_supports_nested_bootstrap_like_composition() {
    let ctx = egui::Context::default();

    show_root(&ctx, |ui| {
        ui.container(Container::fluid(), |ui| {
            ui.row(Row::new().gutter_x(16.0).gutter_y(12.0), |ui| {
                ui.col(Col::new().md(8).sm(12), |ui| {
                    ui.card(Card::new().title("System Status"), |ui| {
                        ui.label(Label::new("Runner-").muted(true));
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
fn semantic_api_supports_phase_three_generator_and_decimation_helpers() {
    let generator = MathGenerator::seeded(GeneratorProfile::BurstyWithSpikes, 1337)
        .sample_rate_hz(240.0)
        .noise_amplitude(0.2)
        .drift(0.0004);

    let series = generator.generate_series("phase3", "Phase 3", 20_000);
    assert_eq!(series.points.len(), 20_000);

    let visible = select_visible_points(&series.points, 5.0, 25.0);
    assert!(!visible.is_empty());

    let decimated = downsample_min_max(&visible, 512);
    assert!(decimated.len() <= 512);

    let line = prepare_line_points(
        &series.points,
        5.0,
        25.0,
        viewport_point_budget(640.0, DEFAULT_DECIMATION_POINTS_PER_PIXEL),
    );
    assert!(!line.is_empty());
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

#[test]
fn semantic_api_supports_phase_one_time_series_foundations() {
    let mut dataset = TimeSeriesDataset::default();

    let mut latency = TimeSeries::new(SeriesId::new("latency"), "P95 Latency");
    latency.push_point(PlotPoint::new(0.0, 12.0));
    latency.push_point(PlotPoint::new(1.0, 18.0));
    latency.push_point(PlotPoint::new(2.0, 14.0));

    let throughput = TimeSeries::new("throughput", "Throughput")
        .visible(false)
        .with_points(vec![PlotPoint::new(0.0, 100.0), PlotPoint::new(2.0, 300.0)]);

    dataset.push_series(latency);
    dataset.push_series(throughput);

    let source = SharedPlotSource::new(dataset);
    let snapshot = source.snapshot();

    let bounds = compute_data_bounds(&snapshot).expect("expected data bounds for snapshot");
    assert_eq!(bounds.x_min, 0.0);
    assert_eq!(bounds.x_max, 2.0);

    let limits = compute_x_navigation_limits(bounds.x_min, bounds.x_max, 1.0, 0.01)
        .expect("expected x navigation limits");
    let (x_min, x_max) = clamp_x_window_to_limits(-100.0, -99.0, limits);

    assert!(x_min >= limits.min_x);
    assert!(x_max <= limits.max_x);

    let data_source: &dyn PlotDataSource = &source;
    let _ = data_source.snapshot();
}

#[test]
fn semantic_api_supports_phase_two_time_series_control_panel() {
    let ctx = egui::Context::default();

    let dataset = TimeSeriesDataset::new(vec![TimeSeries::new("latency", "Latency").with_points(
        vec![
            PlotPoint::new(0.0, 100.0),
            PlotPoint::new(1.0, 102.0),
            PlotPoint::new(2.0, 98.0),
            PlotPoint::new(3.0, 105.0),
        ],
    )]);

    let mut viewport = ViewportState {
        x_mode: AxisZoomMode::Manual,
        y_mode: AxisZoomMode::FitVisible,
        follow: FollowMode::Free,
        x_window_size: 2.0,
        ..ViewportState::default()
    };

    show_root(&ctx, |ui| {
        ui.time_series_plot(
            TimeSeriesPlotProps::new("phase_two_time_series_plot")
                .title("Phase 2")
                .axis_labels("t", "v")
                .show_control_panel(true),
            &dataset,
            &mut viewport,
        );
    });
}

#[test]
fn semantic_api_supports_phase_four_source_backed_time_series_plot() {
    let ctx = egui::Context::default();

    let source = SharedPlotSource::new(TimeSeriesDataset::new(vec![
        TimeSeries::new("live", "Live").with_points(vec![
            PlotPoint::new(0.0, 1.0),
            PlotPoint::new(1.0, 1.2),
            PlotPoint::new(2.0, 0.9),
        ]),
    ]));

    {
        let mut write_guard = source.write();
        write_guard.series[0].push_point(PlotPoint::new(3.0, 1.4));
    }

    let mut viewport = ViewportState {
        x_mode: AxisZoomMode::Manual,
        y_mode: AxisZoomMode::FitVisible,
        follow: FollowMode::PinnedToLatest,
        x_window_size: 2.0,
        ..ViewportState::default()
    };

    show_root(&ctx, |ui| {
        ui.time_series_plot_from_source(
            TimeSeriesPlotProps::new("phase_four_time_series_plot")
                .title("Phase 4")
                .axis_labels("t", "value")
                .show_control_panel(true),
            &source,
            &mut viewport,
        );
    });
}

#[test]
fn semantic_api_supports_phase_five_plot_polish_and_shared_aliases() {
    let ctx = egui::Context::default();

    let source: SharedTimeSeriesDataset =
        SharedTimeSeriesDataset::new(TimeSeriesDataset::new(vec![
            TimeSeries::new("live", "Live")
                .with_points(vec![PlotPoint::new(0.0, 0.2), PlotPoint::new(1.0, 0.6)]),
            TimeSeries::new("hidden", "Hidden")
                .visible(false)
                .with_points(vec![
                    PlotPoint::new(0.0, -100.0),
                    PlotPoint::new(1.0, 100.0),
                ]),
        ]));

    {
        let mut write_guard = source.write();
        write_guard.series[0].push_point(PlotPoint::new(2.0, 0.8));
    }

    let data_source: &dyn PlotDataSource = &source;
    let snapshot = data_source.snapshot();
    assert_eq!(snapshot.series[0].points.len(), 3);

    let mut viewport = ViewportState {
        x_mode: AxisZoomMode::Manual,
        y_mode: AxisZoomMode::FitVisible,
        follow: FollowMode::PinnedToLatest,
        x_window_size: 1.0,
        ..ViewportState::default()
    };

    let _props = PlotControlPanelProps {
        min_margin_x: 0.25,
        has_new_data: true,
    };
    let _response = PlotControlPanelResponse::default();

    show_root(&ctx, |ui| {
        ui.time_series_plot_from_source(
            TimeSeriesPlotProps::new("phase_five_time_series_plot")
                .title("Phase 5")
                .axis_labels("t", "value")
                .min_margin_x(0.25)
                .include_hidden_series(false)
                .show_control_panel(true),
            &source,
            &mut viewport,
        );
    });
}
