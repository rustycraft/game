use std::io::IsTerminal;
use tracing::error;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

mod engine;

const GAME_NAME: &str = "RustyCraft";
const LOG_LEVEL_ENV: &str = "LOG_LEVEL";

fn main() {
    let log_level = std::env::var(LOG_LEVEL_ENV).unwrap_or_else(|_| "info".to_string());

    let console_layer = fmt::layer()
        .with_ansi(std::io::stdout().is_terminal())
        .with_filter(EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(console_layer)
        .init();

    let engine = engine::Engine::new(GAME_NAME.to_string());

    // todo: add levels and start level
    //  add level actions as btn and etc :)

    engine.run().unwrap_or_else(|e| {
        error!("event loop error: {}", e);
        std::process::exit(1);
    });
}