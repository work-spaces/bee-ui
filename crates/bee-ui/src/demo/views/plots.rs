use crate::prelude::*;

use super::shared;

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
