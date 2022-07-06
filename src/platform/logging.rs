use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::prelude::*;

pub fn init_logging() {
    // RUST_LOG=level to control application logging level
    let application_logger = tracing_subscriber::fmt::layer()
        .with_filter(filter_fn(|metadata| metadata.target() == "application"))
        .with_filter(tracing_subscriber::EnvFilter::from_default_env());

    let engine_logger = tracing_subscriber::fmt::layer()
        .with_filter(filter_fn(|metadata| metadata.target() == "engine"))
        .with_filter(tracing_subscriber::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(application_logger)
        .with(engine_logger)
        .init();
}
