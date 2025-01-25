mod file_handling;
mod telemetry;

fn main() {
    let logger = telemetry::telemetry::init_logs();
    let meter = telemetry::telemetry::init_metrics();
    let tracer = telemetry::telemetry::init_tracing();
}
