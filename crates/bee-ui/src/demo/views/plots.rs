use std::sync::OnceLock;

use crate::prelude::*;

use super::shared;

const PHASE_TWO_PLOT_VIEWPORT_ID: &str = "runnerd_bs.demo.plots.phase_two.viewport";
const PHASE_THREE_PLOT_VIEWPORT_ID: &str = "runnerd_bs.demo.plots.phase_three.viewport";
const PHASE_THREE_POINTS_PER_SERIES: usize = 200_000;

pub(super) fn render_view_plots(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — plot composition").outline(), |ui| {
        ui.text(Text::new(
            "PlotArea and ChartCard visualize runnerd SLO trends: latency, error rate, and throughput across recent minutes.",
        ));
    });

    ui.card(
        Card::new()
            .title("Variant gallery — PlotArea fill-width + legend")
            .outline(),
        |ui| {
            ui.plot(
                PlotArea::new("runnerd_bs_demo_plots_fill_width")
                    .title("Service trends (last 35 minutes)")
                    .axis_labels("minute", "value")
                    .height(220.0)
                    .fill_width(true)
                    .legend(true),
                |plot_ui| {
                    for series in shared::METRIC_SERIES {
                        plot_ui.line(line_for_series(series));
                    }
                },
            );
        },
    );

    ui.card(
        Card::new()
            .title("Variant gallery — PlotArea fixed size")
            .outline(),
        |ui| {
            ui.plot(
                PlotArea::new("runnerd_bs_demo_plots_fixed_size")
                    .title("api-gateway latency focus")
                    .x_axis_label("minute")
                    .y_axis_label("ms")
                    .width(320.0)
                    .height(160.0)
                    .fill_width(false)
                    .legend(false),
                |plot_ui| {
                    plot_ui.line(line_for_series(shared::METRIC_SERIES[0]));
                },
            );
        },
    );

    ui.card(
        Card::new()
            .title("Composed example — ChartCard dashboards")
            .outline(),
        |ui| {
            ui.row(Row::new().gutter(Spacing::new(12.0, 12.0)), |ui| {
                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.chart_card(
                        ChartCard::new("Throughput", "runnerd_bs_demo_chart_throughput")
                            .variant(Variant::Success)
                            .footer("filled ChartCard")
                            .plot_height(150.0),
                        |plot_ui| {
                            plot_ui.line(line_for_series(shared::METRIC_SERIES[2]));
                        },
                    );
                });

                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.chart_card(
                        ChartCard::new("Error rate", "runnerd_bs_demo_chart_errors")
                            .variant(Variant::Danger)
                            .outline()
                            .footer("outlined ChartCard")
                            .plot_height(150.0),
                        |plot_ui| {
                            plot_ui.line(line_for_series(shared::METRIC_SERIES[1]));
                        },
                    );
                });

                ui.col(Col::new().xs(12).lg(4), |ui| {
                    ui.chart_card(
                        ChartCard::new("P95 latency", "runnerd_bs_demo_chart_latency")
                            .variant(Variant::Info)
                            .footer("fill-width chart in card composition")
                            .plot_height(150.0),
                        |plot_ui| {
                            plot_ui.line(line_for_series(shared::METRIC_SERIES[0]));
                        },
                    );
                });
            });
        },
    );

    ui.card(
        Card::new()
            .title("Phase 2 — control panel + bounded scrolling")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Use manual X/Y modes to pan/zoom. X panning is clamped to data extent with a small overscroll margin.",
            ));

            ui.raw(|egui_ui| {
                let viewport_id = egui::Id::new(PHASE_TWO_PLOT_VIEWPORT_ID);
                let mut viewport = egui_ui
                    .ctx()
                    .data_mut(|data| data.get_temp::<ViewportState>(viewport_id))
                    .unwrap_or_else(|| ViewportState {
                        x_mode: AxisZoomMode::Manual,
                        y_mode: AxisZoomMode::FitVisible,
                        follow: FollowMode::Free,
                        x_window_size: 10.0,
                        ..ViewportState::default()
                    });

                let dataset = phase_two_dataset();
                let plot = TimeSeriesPlotProps::new("runnerd_bs_demo_phase_two_controlled_plot")
                    .title("Latency trend (manual viewport controls)")
                    .axis_labels("minute", "latency ms")
                    .height(240.0)
                    .legend(true)
                    .fill_width(true)
                    .min_margin_x(0.5);

                TimeSeriesPlot::show(egui_ui, &plot, &dataset, &mut viewport);

                egui_ui
                    .ctx()
                    .data_mut(|data| data.insert_temp(viewport_id, viewport));
            });
        },
    );

    ui.card(
        Card::new()
            .title("Phase 3 — large synthetic dataset + adaptive downsampling")
            .outline(),
        |ui| {
            let dataset = phase_three_dataset();
            let total_points: usize = dataset.series.iter().map(|series| series.points.len()).sum();

            ui.text(Text::new(format!(
                "Synthetic workload with {total_points} points across {} series. The renderer applies viewport-first filtering and adaptive min/max decimation for smooth interaction.",
                dataset.series.len(),
            )));

            ui.raw(|egui_ui| {
                let viewport_id = egui::Id::new(PHASE_THREE_PLOT_VIEWPORT_ID);
                let mut viewport = egui_ui
                    .ctx()
                    .data_mut(|data| data.get_temp::<ViewportState>(viewport_id))
                    .unwrap_or_else(|| ViewportState {
                        x_mode: AxisZoomMode::Manual,
                        y_mode: AxisZoomMode::FitVisible,
                        follow: FollowMode::Free,
                        x_window_size: 25.0,
                        x_min: 0.0,
                        x_max: 25.0,
                        ..ViewportState::default()
                    });

                let point_budget = viewport_point_budget(
                    egui_ui.available_width(),
                    DEFAULT_DECIMATION_POINTS_PER_PIXEL,
                );
                egui_ui.small(format!(
                    "Adaptive point budget: ~{point_budget} points/series in-view ({} raw points per series).",
                    PHASE_THREE_POINTS_PER_SERIES,
                ));

                let plot = TimeSeriesPlotProps::new("runnerd_bs_demo_phase_three_large_dataset")
                    .title("Synthetic throughput/latency/noise signals")
                    .axis_labels("time (s)", "value")
                    .height(280.0)
                    .legend(true)
                    .fill_width(true)
                    .min_margin_x(1.0);

                TimeSeriesPlot::show(egui_ui, &plot, dataset, &mut viewport);

                egui_ui
                    .ctx()
                    .data_mut(|data| data.insert_temp(viewport_id, viewport));
            });
        },
    );
}

fn line_for_series(series: shared::MetricSeries) -> egui_plot::Line<'static> {
    let points: Vec<[f64; 2]> = series
        .points
        .iter()
        .map(|point| [f64::from(point.minute), point.value])
        .collect();

    let color = match series.variant {
        Variant::Primary => egui::Color32::from_rgb(80, 120, 255),
        Variant::Secondary => egui::Color32::from_rgb(130, 130, 130),
        Variant::Success => egui::Color32::from_rgb(40, 180, 99),
        Variant::Danger => egui::Color32::from_rgb(220, 53, 69),
        Variant::Warning => egui::Color32::from_rgb(255, 193, 7),
        Variant::Info => egui::Color32::from_rgb(13, 202, 240),
        Variant::Light => egui::Color32::from_rgb(210, 210, 210),
        Variant::Dark => egui::Color32::from_rgb(55, 55, 55),
    };

    egui_plot::Line::new(series.label, points).color(color)
}

fn phase_two_dataset() -> TimeSeriesDataset {
    let latency_points: Vec<PlotPoint> = shared::METRIC_SERIES[0]
        .points
        .iter()
        .map(|point| PlotPoint::new(f64::from(point.minute), point.value))
        .collect();

    let baseline_points: Vec<PlotPoint> = shared::METRIC_SERIES[0]
        .points
        .iter()
        .map(|point| {
            let x = f64::from(point.minute);
            let y = 96.0 + (x / 7.0).sin() * 3.0;
            PlotPoint::new(x, y)
        })
        .collect();

    let hidden_outlier = TimeSeries::new("hidden_outlier", "Hidden outlier")
        .visible(false)
        .with_points(vec![PlotPoint::new(8.0, 10.0), PlotPoint::new(26.0, 500.0)]);

    TimeSeriesDataset::new(vec![
        TimeSeries::new("latency", "P95 latency").with_points(latency_points),
        TimeSeries::new("baseline", "Target baseline").with_points(baseline_points),
        hidden_outlier,
    ])
}

fn phase_three_dataset() -> &'static TimeSeriesDataset {
    static PHASE_THREE_DATASET: OnceLock<TimeSeriesDataset> = OnceLock::new();

    PHASE_THREE_DATASET.get_or_init(|| {
        let baseline = MathGenerator::seeded(GeneratorProfile::MultiSine, 0x10)
            .sample_rate_hz(180.0)
            .noise_amplitude(0.03)
            .drift(0.001)
            .generate_series(
                "phase_three.baseline",
                "Baseline multi-sine",
                PHASE_THREE_POINTS_PER_SERIES,
            );

        let chirp = MathGenerator::seeded(GeneratorProfile::Chirp, 0x20)
            .sample_rate_hz(180.0)
            .noise_amplitude(0.05)
            .drift(0.0005)
            .generate_series("phase_three.chirp", "Chirp", PHASE_THREE_POINTS_PER_SERIES);

        let bursty = MathGenerator::seeded(GeneratorProfile::BurstyWithSpikes, 0x30)
            .sample_rate_hz(180.0)
            .noise_amplitude(0.07)
            .drift(0.0)
            .generate_series(
                "phase_three.bursty",
                "Bursty + spikes",
                PHASE_THREE_POINTS_PER_SERIES,
            );

        TimeSeriesDataset::new(vec![baseline, chirp, bursty])
    })
}
