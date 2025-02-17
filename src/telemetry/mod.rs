use once_cell::sync::Lazy;
use opentelemetry::global;
use opentelemetry_stdout::{SpanExporter, LogExporter, MetricExporter};

// tracing
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;

// logging
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_appender_tracing::layer;
use tracing_subscriber::{prelude::*, EnvFilter};

// metrics
use opentelemetry_sdk::metrics::SdkMeterProvider;

pub const APP_NAME: &str = "runv";

static RESOURCE: Lazy<Resource> = Lazy::new(|| {
    Resource::builder()
        .with_service_name(APP_NAME)
        .build()
});

pub fn init_telemetry() -> (SdkTracerProvider, SdkMeterProvider, SdkLoggerProvider) {
    let tracer_provider = init_tracing();
    let meter_provider = init_metrics();
    let logger_provider = init_logging();
    (tracer_provider, meter_provider, logger_provider)
}

pub fn shutdown_telemetry(providers: (SdkTracerProvider, SdkMeterProvider, SdkLoggerProvider)) {
    let _ = providers.0.shutdown();
    let _ = providers.1.shutdown();
    let _ = providers.2.shutdown();
}

fn init_tracing() -> SdkTracerProvider {
    let provider = SdkTracerProvider::builder()
        .with_resource(RESOURCE.clone())
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider.clone());
    provider
}

fn init_logging() -> SdkLoggerProvider {
    let provider = SdkLoggerProvider::builder()
        .with_resource(RESOURCE.clone())
        .with_simple_exporter(LogExporter::default())
        .build();
    let filter = EnvFilter::new("info");
    let layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filter);
    tracing_subscriber::registry().with(layer).init();
    provider
}

fn init_metrics() -> SdkMeterProvider {
    let provider = SdkMeterProvider::builder()
        .with_periodic_exporter(MetricExporter::default())
        .with_resource(RESOURCE.clone())
        .build();
    global::set_meter_provider(provider.clone());
    provider
}