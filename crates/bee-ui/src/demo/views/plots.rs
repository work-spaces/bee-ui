use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use crate::prelude::*;

use super::shared;

const SCATTER_PLOT_VIEWPORT_ID: &str = "runnerd_bs.demo.plots.scatter.viewport";
const PHASE_TWO_PLOT_VIEWPORT_ID: &str = "runnerd_bs.demo.plots.phase_two.viewport";
const PHASE_THREE_PLOT_VIEWPORT_ID: &str = "runnerd_bs.demo.plots.phase_three.viewport";
const PHASE_FOUR_PLOT_VIEWPORT_ID: &str = "runnerd_bs.demo.plots.phase_four.viewport";
const PHASE_FOUR_STREAM_STATE_ID: &str = "runnerd_bs.demo.plots.phase_four.stream_state";

const PHASE_THREE_POINTS_PER_SERIES: usize = 200_000;

const PHASE_FOUR_STREAM_SERIES_ID: &str = "phase_four.live_signal";
const PHASE_FOUR_STREAM_SERIES_LABEL: &str = "Live signal";
const PHASE_FOUR_INITIAL_POINTS: usize = 4_000;
const PHASE_FOUR_DEFAULT_SAMPLE_RATE_HZ: f64 = 100.0;
const PHASE_FOUR_MIN_RETENTION_POINTS: usize = 1_000;
const PHASE_FOUR_DEFAULT_RETENTION_POINTS: usize = 60_000;
const PHASE_FOUR_MAX_RETENTION_POINTS: usize = 200_000;
const PHASE_FOUR_MAX_BATCH_POINTS: usize = 2_000;
const PHASE_FOUR_REPAINT_INTERVAL: Duration = Duration::from_millis(16);
const PHASE_FOUR_SAMPLE_RATE_OPTIONS: [f64; 3] = [10.0, 100.0, 1_000.0];

pub(super) fn render_view_plots(ui: &mut UiRoot<'_>) {
    ui.card(Card::new().title("Intro — plot composition").outline(), |ui| {
        ui.text(Text::new(
            "PlotArea and ChartCard visualize runnerd SLO trends: latency, error rate, and throughput across recent minutes.",
        ));
    });

    ui.card(
        Card::new()
            .title("Scatter plot — correlated measurements")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Each independent X/Y measurement carries its own marker shape, size, and color.",
            ));

            ui.raw(|egui_ui| {
                let viewport_id = egui::Id::new(SCATTER_PLOT_VIEWPORT_ID);
                let mut viewport = egui_ui
                    .ctx()
                    .data_mut(|data| data.get_temp::<ViewportState>(viewport_id))
                    .unwrap_or_default();
                let dataset = scatter_demo_dataset();
                let props = ScatterPlotProps::new("runnerd_bs_demo_scatter_plot")
                    .title("Sensor response by sample")
                    .axis_labels("input", "response")
                    .height(260.0)
                    .legend(true);

                ScatterPlot::show_with_viewport(egui_ui, &props, dataset, &mut viewport);
                egui_ui.push_id("scatter_control_panel", |scoped_ui| {
                    ScatterControlPanel::show(scoped_ui, &mut viewport, dataset);
                });

                egui_ui
                    .ctx()
                    .data_mut(|data| data.insert_temp(viewport_id, viewport));
            });
        },
    );

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

    ui.card(
        Card::new()
            .title("Phase 4 — real-time streaming + pin/unpin live mode")
            .outline(),
        |ui| {
            ui.text(Text::new(
                "Live terminal-style stream with follow-newest controls, Go Live affordance, pause/resume, and sample-rate stress presets.",
            ));

            ui.raw(|egui_ui| {
                let ctx = egui_ui.ctx().clone();
                let viewport_id = egui::Id::new(PHASE_FOUR_PLOT_VIEWPORT_ID);
                let stream_state_id = egui::Id::new(PHASE_FOUR_STREAM_STATE_ID);

                let mut viewport = ctx
                    .data_mut(|data| data.get_temp::<ViewportState>(viewport_id))
                    .unwrap_or_else(|| ViewportState {
                        x_mode: AxisZoomMode::Manual,
                        y_mode: AxisZoomMode::FitVisible,
                        follow: FollowMode::PinnedToLatest,
                        x_window_size: 20.0,
                        x_min: 0.0,
                        x_max: 20.0,
                        ..ViewportState::default()
                    });

                let mut stream_state = ctx
                    .data_mut(|data| data.get_temp::<LiveStreamingDemoState>(stream_state_id))
                    .unwrap_or_default();

                egui_ui.horizontal_wrapped(|controls_ui| {
                    let button_label = if stream_state.running {
                        "Pause stream"
                    } else {
                        "Resume stream"
                    };
                    if controls_ui.button(button_label).clicked() {
                        if stream_state.running {
                            stream_state.pause();
                        } else {
                            stream_state.resume();
                        }
                    }

                    controls_ui.separator();
                    controls_ui.label("Rate");

                    let mut selected_rate = stream_state.sample_rate_hz;
                    egui::ComboBox::from_id_salt("runnerd_bs_demo_phase_four_rate")
                        .selected_text(format!("{selected_rate:.0} pts/s"))
                        .show_ui(controls_ui, |combo_ui| {
                            for rate in PHASE_FOUR_SAMPLE_RATE_OPTIONS {
                                combo_ui.selectable_value(
                                    &mut selected_rate,
                                    rate,
                                    format!("{rate:.0} pts/s"),
                                );
                            }
                        });

                    if (selected_rate - stream_state.sample_rate_hz).abs() > f64::EPSILON {
                        stream_state.set_sample_rate_hz(selected_rate);
                    }

                    controls_ui.separator();
                    controls_ui.checkbox(&mut stream_state.noise_enabled, "Noise");
                    controls_ui.checkbox(&mut stream_state.spikes_enabled, "Spikes");

                    controls_ui.separator();
                    controls_ui.label("Retention");
                    let mut retention_points = stream_state.retention_points as u32;
                    if controls_ui
                        .add(
                            egui::Slider::new(
                                &mut retention_points,
                                PHASE_FOUR_MIN_RETENTION_POINTS as u32
                                    ..=PHASE_FOUR_MAX_RETENTION_POINTS as u32,
                            )
                            .text("points")
                            .logarithmic(true),
                        )
                        .changed()
                    {
                        stream_state.set_retention_points(retention_points as usize);
                    }
                });

                let appended = stream_state.tick(Instant::now());
                let data_source: &dyn PlotDataSource = &stream_state.source;
                let dataset = data_source.snapshot();

                let (retained_points, newest_x) = live_dataset_stats(&dataset);
                let newest_x_text = newest_x
                    .map(|x| format!("{x:.2}s"))
                    .unwrap_or_else(|| "n/a".to_owned());
                egui_ui.small(format!(
                    "Status: {} • retained: {} points • total produced: {} • newest t: {}",
                    if stream_state.running {
                        "streaming"
                    } else {
                        "paused"
                    },
                    retained_points,
                    stream_state.total_generated,
                    newest_x_text,
                ));

                let plot = TimeSeriesPlotProps::new("runnerd_bs_demo_phase_four_live_plot")
                    .title("Live stream with follow-newest controls")
                    .axis_labels("time (s)", "signal")
                    .height(300.0)
                    .legend(true)
                    .fill_width(true)
                    .min_margin_x(0.5);

                let response = TimeSeriesPlot::show(egui_ui, &plot, &dataset, &mut viewport);

                if viewport.follow == FollowMode::Free && response.has_new_data {
                    egui_ui.small("Live data is arriving while unpinned. Use Go Live to jump back to the latest window.");
                }

                if stream_state.running {
                    if appended > 0 {
                        ctx.request_repaint();
                    } else {
                        ctx.request_repaint_after(PHASE_FOUR_REPAINT_INTERVAL);
                    }
                }

                ctx.data_mut(|data| {
                    data.insert_temp(viewport_id, viewport);
                    data.insert_temp(stream_state_id, stream_state);
                });
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

fn scatter_demo_dataset() -> &'static ScatterDataset {
    static SCATTER_DEMO_DATASET: OnceLock<ScatterDataset> = OnceLock::new();

    SCATTER_DEMO_DATASET.get_or_init(|| {
        let measurements = (0..48)
            .map(|index| {
                let x = f64::from(index) / 6.0;
                let variation =
                    (f64::from(index) * 1.7).sin() * 1.4 + (f64::from(index) * 0.37).cos() * 0.7;
                let (shape, color) = match index % 3 {
                    0 => (ScatterShape::Circle, egui::Color32::from_rgb(80, 120, 255)),
                    1 => (ScatterShape::Square, egui::Color32::from_rgb(40, 180, 99)),
                    _ => (ScatterShape::Diamond, egui::Color32::from_rgb(220, 53, 69)),
                };
                ScatterPoint::new(x, 2.1 * x + variation + 3.0)
                    .shape(shape)
                    .size(3.0 + (index % 4) as f32)
                    .color(color)
            })
            .collect();
        let control = (0..24)
            .map(|index| {
                let x = f64::from(index) / 3.0;
                let variation = (f64::from(index) * 1.2).sin() * 1.1;
                ScatterPoint::new(x, 1.5 * x + variation + 1.0)
                    .shape(ScatterShape::Cross)
                    .size(4.0)
                    .color(egui::Color32::from_rgb(13, 202, 240))
            })
            .collect();

        ScatterDataset::new(vec![
            ScatterSeries::new("scatter.measurements", "Measurements").with_points(measurements),
            ScatterSeries::new("scatter.control", "Control group").with_points(control),
            ScatterSeries::new("scatter.outlier", "Outliers").with_points(vec![
                ScatterPoint::new(2.0, 20.0)
                    .shape(ScatterShape::Plus)
                    .size(7.0)
                    .color(egui::Color32::from_rgb(255, 193, 7)),
                ScatterPoint::new(6.8, 4.0)
                    .shape(ScatterShape::Diamond)
                    .size(6.0)
                    .color(egui::Color32::from_rgb(255, 193, 7)),
            ]),
        ])
    })
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

#[derive(Clone, Debug)]
struct LiveStreamingDemoState {
    source: SharedPlotSource,
    generator: MathGenerator,
    running: bool,
    sample_rate_hz: f64,
    retention_points: usize,
    noise_enabled: bool,
    spikes_enabled: bool,
    sample_carry: f64,
    last_update: Instant,
    total_generated: u64,
}

impl Default for LiveStreamingDemoState {
    fn default() -> Self {
        let mut generator = MathGenerator::seeded(GeneratorProfile::MultiSine, 0x44)
            .sample_rate_hz(PHASE_FOUR_DEFAULT_SAMPLE_RATE_HZ)
            .noise_amplitude(0.08)
            .drift(0.0008);

        let warmup_points = generator.generate_next(PHASE_FOUR_INITIAL_POINTS);
        let source = SharedPlotSource::new(TimeSeriesDataset::new(vec![
            TimeSeries::new(PHASE_FOUR_STREAM_SERIES_ID, PHASE_FOUR_STREAM_SERIES_LABEL)
                .with_points(warmup_points),
        ]));

        Self {
            source,
            generator,
            running: true,
            sample_rate_hz: PHASE_FOUR_DEFAULT_SAMPLE_RATE_HZ,
            retention_points: PHASE_FOUR_DEFAULT_RETENTION_POINTS,
            noise_enabled: true,
            spikes_enabled: false,
            sample_carry: 0.0,
            last_update: Instant::now(),
            total_generated: PHASE_FOUR_INITIAL_POINTS as u64,
        }
    }
}

impl LiveStreamingDemoState {
    fn pause(&mut self) {
        self.running = false;
        self.sample_carry = 0.0;
        self.last_update = Instant::now();
    }

    fn resume(&mut self) {
        self.running = true;
        self.sample_carry = 0.0;
        self.last_update = Instant::now();
    }

    fn set_sample_rate_hz(&mut self, sample_rate_hz: f64) {
        self.sample_rate_hz = nearest_sample_rate(sample_rate_hz);
        self.sample_carry = 0.0;
        self.last_update = Instant::now();
    }

    fn set_retention_points(&mut self, retention_points: usize) {
        self.retention_points = clamp_retention_points(retention_points);
        self.enforce_retention_limit();
    }

    fn tick(&mut self, now: Instant) -> usize {
        if !self.running {
            self.last_update = now;
            return 0;
        }

        self.configure_generator();

        let elapsed = now
            .saturating_duration_since(self.last_update)
            .as_secs_f64();
        self.last_update = now;

        self.sample_carry += elapsed * self.sample_rate_hz;
        let requested_points = self.sample_carry.floor() as usize;
        if requested_points == 0 {
            return 0;
        }

        let emitted_points = requested_points.min(PHASE_FOUR_MAX_BATCH_POINTS);
        if requested_points > PHASE_FOUR_MAX_BATCH_POINTS {
            self.sample_carry = 0.0;
        } else {
            self.sample_carry -= emitted_points as f64;
        }

        let points = self.generator.generate_next(emitted_points);

        {
            let mut dataset = self.source.write();
            let series = ensure_live_series(&mut dataset);
            series.points.extend(points);

            if series.points.len() > self.retention_points {
                let overflow = series.points.len() - self.retention_points;
                series.points.drain(0..overflow);
            }
        }

        self.total_generated = self.total_generated.saturating_add(emitted_points as u64);
        emitted_points
    }

    fn enforce_retention_limit(&mut self) {
        let mut dataset = self.source.write();
        let series = ensure_live_series(&mut dataset);
        if series.points.len() > self.retention_points {
            let overflow = series.points.len() - self.retention_points;
            series.points.drain(0..overflow);
        }
    }

    fn configure_generator(&mut self) {
        self.generator.profile = if self.spikes_enabled {
            GeneratorProfile::BurstyWithSpikes
        } else {
            GeneratorProfile::MultiSine
        };
        self.generator.sample_rate_hz = self.sample_rate_hz;
        self.generator.noise_amplitude = if self.noise_enabled { 0.08 } else { 0.0 };
        self.generator.drift = if self.spikes_enabled { 0.0002 } else { 0.0008 };
    }
}

fn ensure_live_series(dataset: &mut TimeSeriesDataset) -> &mut TimeSeries {
    if let Some(index) = dataset
        .series
        .iter()
        .position(|series| series.id.as_str() == PHASE_FOUR_STREAM_SERIES_ID)
    {
        return &mut dataset.series[index];
    }

    dataset.series.push(TimeSeries::new(
        PHASE_FOUR_STREAM_SERIES_ID,
        PHASE_FOUR_STREAM_SERIES_LABEL,
    ));
    dataset
        .series
        .last_mut()
        .expect("just pushed a live stream series")
}

fn live_dataset_stats(dataset: &TimeSeriesDataset) -> (usize, Option<f64>) {
    let series = dataset
        .series
        .iter()
        .find(|series| series.id.as_str() == PHASE_FOUR_STREAM_SERIES_ID)
        .or_else(|| dataset.series.first());

    let point_count = series.map(|series| series.points.len()).unwrap_or_default();
    let newest_x = series
        .and_then(|series| series.points.last())
        .map(|point| point.x);

    (point_count, newest_x)
}

fn nearest_sample_rate(sample_rate_hz: f64) -> f64 {
    PHASE_FOUR_SAMPLE_RATE_OPTIONS
        .iter()
        .copied()
        .min_by(|left, right| {
            (left - sample_rate_hz)
                .abs()
                .partial_cmp(&(right - sample_rate_hz).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(PHASE_FOUR_DEFAULT_SAMPLE_RATE_HZ)
}

fn clamp_retention_points(retention_points: usize) -> usize {
    retention_points.clamp(
        PHASE_FOUR_MIN_RETENTION_POINTS,
        PHASE_FOUR_MAX_RETENTION_POINTS,
    )
}
