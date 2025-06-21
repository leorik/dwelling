use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use std::env;
use std::time::Duration;
use thiserror::Error;
use tracing::warn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Error, Debug)]
enum ObservabilityInitError {
    #[error("Export protocol {0} is not supported")]
    UnsupportedExportProtocol(String),
    #[error("No OTLP endpoint provided")]
    NoEndpointProvided,
}

pub fn init() {
    let tracing_endpoint = env::var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
        .or(env::var("OTEL_EXPORTER_OTLP_ENDPOINT"))
        .ok();

    let telemetry_layer = if let Some(tracing_endpoint) = tracing_endpoint {
        let protocol = env::var("OTEL_EXPORTER_OTLP_PROTOCOL").unwrap_or("grpc".to_owned());

        match protocol.as_str() {
            "grpc" => {
                let mut exporter_builder = opentelemetry_otlp::SpanExporter::builder()
                    .with_tonic()
                    .with_timeout(Duration::from_secs(3));

                if tracing_endpoint.starts_with("https") {
                    exporter_builder = exporter_builder
                        .with_tls_config(ClientTlsConfig::default().with_enabled_roots());
                }

                let exporter = exporter_builder
                    .build()
                    .expect("Cannot build OTLP exporter");

                let tracer = opentelemetry_sdk::trace::SdkTracerProvider::builder()
                    .with_batch_exporter(exporter)
                    .build()
                    .tracer("otlp-tracer");

                Ok(tracing_opentelemetry::layer().with_tracer(tracer))
            }
            _ => Err(ObservabilityInitError::UnsupportedExportProtocol(protocol)),
        }
    } else {
        Err(ObservabilityInitError::NoEndpointProvided)
    };

    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "error,dwelling=info".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    match telemetry_layer {
        Ok(telemetry_layer) => {
            subscriber.with(telemetry_layer).init();
        }
        Err(e) => {
            subscriber.init();
            warn!("Tracing disabled: {}", e);
        }
    }
}
