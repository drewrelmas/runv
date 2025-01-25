use opentelemetry_sdk::metrics::{SdkMeterProvider, PeriodicReader};
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::TracerProvider;

use opentelemetry_sdk::logs::LoggerProvider;

pub fn init_tracing() -> TracerProvider {
    TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build()
}

pub fn init_logs() -> LoggerProvider {
    LoggerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::LogExporter::default())
        .build()
}

pub fn init_metrics() -> SdkMeterProvider {
    let exporter = opentelemetry_stdout::MetricExporter::default();
    let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();
    SdkMeterProvider::builder().with_reader(reader).build()
}