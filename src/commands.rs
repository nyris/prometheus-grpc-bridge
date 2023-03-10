use crate::logging::LoggingStyle;
use clap::{Arg, Command};
use std::net::{AddrParseError, SocketAddr, ToSocketAddrs};

const APP_VERSION: &'static str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA_SHORT"),
    " ",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn build_command() -> Command {
    let command = Command::new("Prometheus gRPC Bridge")
        .version(APP_VERSION)
        .author("Markus Mayer <m.mayer@nyris.io>")
        .about("Read Prometheus Metrics")
        .arg(
            Arg::new("serve_address")
                .long("serve")
                .env("HTTP_SERVER_BIND_ADDRESS")
                .value_name("ADDRESS")
                .default_missing_value("0.0.0.0:80")
                .help("The HTTP address to bind to")
                .help_heading("Server")
                .value_parser(parse_socketaddr),
        )
        .arg(
            Arg::new("grpc_address")
                .short('a')
                .long("grpc-address")
                .env("GRPC_SERVER_CONNECT_ADDRESS")
                .value_name("ADDRESS")
                .default_value("localhost:11000")
                .help("The gRPC address to connect to")
                .num_args(1)
                .help_heading("Endpoint")
                .value_parser(parse_address),
        )
        .arg(
            Arg::new("grpc_scheme")
                .short('s')
                .long("grpc-scheme")
                .env("GRPC_SERVER_CONNECT_SCHEME")
                .value_name("SCHEME")
                .default_value("http")
                .help("The gRPC scheme to use, e.g. http or https")
                .num_args(1)
                .help_heading("Endpoint"),
        )
        .arg(
            Arg::new("logging_style")
                .short('l')
                .long("log")
                .env("HTTP_SERVER_LOG_STYLE")
                .value_name("STYLE")
                .default_value("simple")
                .help("The logging style to use (simple, json)")
                .num_args(1)
                .value_parser(logging_style)
                .help_heading("Logging"),
        );
    command
}

/// Ensures that the provided address is a valid socket address,
/// e.g. `0.0.0.0:80`.
fn parse_socketaddr(value: &str) -> Result<SocketAddr, String> {
    Ok(value.parse().map_err(|e: AddrParseError| e.to_string())?)
}

/// Ensures that the provided address is a valid HTTP host and port,
/// e.g. `localhost:11000`.
///
/// This method differs from [`parse_socketaddr`] in that it allows
/// host names, not just IP addresses, which it will attempt to look up.
fn parse_address(value: &str) -> Result<String, String> {
    let _ = value.to_socket_addrs().map_err(|e| e.to_string())?;
    Ok(String::from(value))
}

/// Validates and parses the logging style.
fn logging_style(s: &str) -> Result<LoggingStyle, String> {
    match s {
        "simple" => Ok(LoggingStyle::Compact),
        "compact" => Ok(LoggingStyle::Compact),
        "json" => Ok(LoggingStyle::Json),
        _ => Err(String::from("Either simple or json must be specified")),
    }
}
