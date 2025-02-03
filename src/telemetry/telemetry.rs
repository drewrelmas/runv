use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_appender_tracing::layer;
use std::sync::Once;
use std::sync::Mutex;
use tracing_subscriber::prelude::*;

pub const APP_NAME: &str = "runv";

// Since otel currently doesn't have a global logger provider, we need to initialize it once
static INIT: Once = Once::new();
static mut LOGGER_PROVIDER: Option<Mutex<LoggerProvider>> = None;

pub fn init_telemetry() {
    INIT.call_once(|| {
        init_tracing();
        let logger_provider = init_logging();
        unsafe {
            LOGGER_PROVIDER = Some(Mutex::new(logger_provider));
        }
    });
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
    unsafe {
        if let Some(ref provider) = LOGGER_PROVIDER {
            let _ = provider.lock().unwrap().shutdown();
        }
    }
}

fn init_tracing() {
    let provider = TracerProvider::builder()
        .with_resource(Resource::new_with_defaults(vec![KeyValue::new("service.name", APP_NAME)]))
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);
}

fn init_logging() -> LoggerProvider {
    let provider = LoggerProvider::builder()
        .with_resource(Resource::new_with_defaults(vec![KeyValue::new("service.name", APP_NAME)]))
        .with_simple_exporter(opentelemetry_stdout::LogExporter::default())
        .build();
    let layer = layer::OpenTelemetryTracingBridge::new(&provider);
    tracing_subscriber::registry().with(layer).init();
    provider
}