use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn configure_logs() {
    let env_filter = EnvFilter::from_default_env();

    Subscriber::builder()
        .with_env_filter(env_filter)
        .finish()
        .init();
}
