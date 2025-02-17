mod file_handling;
mod telemetry;

use file_handling::file_handler;
use opentelemetry::{global, trace::{Tracer, TracerProvider}};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let providers = telemetry::init_telemetry();
    do_some_work();
    telemetry::shutdown_telemetry(providers);
    Ok(())
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
