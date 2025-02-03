mod file_handling;
mod telemetry;

use file_handling::file_handler::unzip_file;
use opentelemetry::{global, trace::{Tracer, TracerProvider}};
use tracing::info;

fn main() {
    telemetry::telemetry::init_telemetry();
    do_some_work();
    telemetry::telemetry::shutdown_telemetry();
}

fn do_some_work() {
    let tracer = global::tracer_provider().tracer(telemetry::telemetry::APP_NAME);
    tracer.in_span("do_some_work", |_cx| {
        let result = unzip_file("./data/export_28658334.zip", None);
        if let Err(ref e) = result {
            info!("Failed to unzip file: {:?}", e);
        } else {
            info!("Unzipped file successfully");
        }
    });
}
