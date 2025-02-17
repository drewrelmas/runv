mod file_handling;
mod telemetry;
mod ui;
mod app;

use file_handling::file_handler;
use app::RunVApp;

#[tokio::main]
async fn main() -> eframe::Result {
    let providers = telemetry::init_telemetry();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native(
        telemetry::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(RunVApp::new(cc)))),
    )
}