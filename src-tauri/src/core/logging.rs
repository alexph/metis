use std::sync::Once;

static LOGGING_INIT: Once = Once::new();

pub fn init_logging() {
    LOGGING_INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_target(false)
            .compact()
            .init();
    });
}
