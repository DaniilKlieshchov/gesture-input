use tracing::Level;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, Registry, filter::LevelFilter};


pub fn setup_logger() {
    let std_layer = tracing_subscriber::fmt::layer().pretty().with_writer(std::io::stdout);

    let file_appender = tracing_appender::rolling::never("./logs", "app.log");
    let file_layer = tracing_subscriber::fmt::layer().with_writer(file_appender);

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::INFO))
        .with(file_layer)
        .with(std_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set default subscriber");
}