use std::{env, io::IsTerminal};
use tracing_subscriber::{EnvFilter, Registry, prelude::*};

// Define the LogStyle enum
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LogStyle {
    Pretty,
    Text,
    Json,
}

impl LogStyle {
    fn from_env() -> Self {
        match env::var("GRAFBASE_LOG_STYLE").ok().as_deref() {
            Some("pretty") => LogStyle::Pretty,
            Some("text") => LogStyle::Text,
            Some("json") => LogStyle::Json,
            _ => {
                // Default logic
                let log_level = env::var("GRAFBASE_LOG").unwrap_or_else(|_| "info".to_string());
                let is_terminal = std::io::stdout().is_terminal();
                if is_terminal && (log_level.contains("debug") || log_level.contains("trace")) {
                    LogStyle::Pretty
                } else {
                    LogStyle::Text
                }
            }
        }
    }
}

pub(super) fn init() {
    let env_filter = EnvFilter::builder()
        .with_env_var("GRAFBASE_LOG")
        .try_from_env()
        .unwrap_or_else(|_| EnvFilter::from("info")); // Default to "info" level

    let log_style = LogStyle::from_env();
    let is_terminal = std::io::stdout().is_terminal();
    let registry = Registry::default();

    match log_style {
        LogStyle::Pretty => registry
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty()
                    .with_ansi(is_terminal)
                    .with_target(false),
            )
            .with(env_filter)
            .init(),
        LogStyle::Text => registry
            .with(
                tracing_subscriber::fmt::layer()
                    .with_ansi(is_terminal)
                    .with_target(false),
            )
            .with(env_filter)
            .init(),
        LogStyle::Json => registry
            .with(tracing_subscriber::fmt::layer().json()) // Use JSON formatting
            .with(env_filter) // Apply the filter
            .init(), // Set this subscriber as the global default
    };
}
