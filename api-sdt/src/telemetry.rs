use opentelemetry::global;
use tracing_subscriber::{EnvFilter, prelude::*};
use crate::settings::Settings;

pub fn init_telemetry(settings: Settings) {

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // Define Tracer
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(settings.jaeger.endpoint)
        .with_service_name("api-sdt")
        .install_simple()
        .expect("Não foi possível instalar o Jaeger Tracer");

    // Layer to add our configured tracer.
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Define subscriber with a tracing layer to use our tracer
    let subscriber = tracing_subscriber::fmt::layer().json();

    // Layer to filter traces based on level - trace, debug, info, warn, error.
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));

    // Trace executed code
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(telemetry)
        .init();

}