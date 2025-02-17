mod file_handling;
mod telemetry;
mod app;

use file_handling::file_handler;
use opentelemetry::{global, trace::{Tracer, TracerProvider}};
use tracing::info;
use crate::app::TemplateApp;

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
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    )
    
    // do_some_work();
    // telemetry::shutdown_telemetry(providers);
    // Ok(())
}

fn do_some_work() {
    let tracer = global::tracer_provider().tracer(telemetry::APP_NAME);
    tracer.in_span("do_some_work", |_cx| {
        let result = file_handler::unzip_file("./data/export_28658334.zip", None);
        if let Err(ref e) = result {
            info!("Failed to unzip file: {:?}", e);
        } else {
            info!("Unzipped file successfully");
        }
    });
}
