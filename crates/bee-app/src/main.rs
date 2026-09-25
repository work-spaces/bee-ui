use eframe::egui;

struct BeeUiDemo;

impl eframe::App for BeeUiDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        bee_ui::demo::show_bootstrap_demo(ctx);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("runnerd-bs Demo")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "bee-ui Demo",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(BeeUiDemo))
        }),
    )
}
