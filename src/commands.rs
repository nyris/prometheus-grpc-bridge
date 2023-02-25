use clap::{Arg, Command};

pub fn build_command() -> Command {
    let command = Command::new("Metrics Client")
        .version("0.1.0")
        .author("Markus Mayer <m.mayer@nyris.io>")
        .about("Read Prometheus Metrics")
        .arg(
            Arg::new("endpoint")
                .short('e')
                .long("endpoint")
                .env("GRPC_SERVER_CONNECT_ADDRESS")
                .value_name("ENDPOINT")
                .default_value("localhost:11000")
                .help("The gRPC endpoint to connect to")
                .num_args(1)
                .help_heading("Endpoint"),
        )
        .arg(
            Arg::new("scheme")
                .short('s')
                .long("scheme")
                .env("GRPC_SERVER_CONNECT_SCHEME")
                .value_name("SCHEME")
                .default_value("http")
                .help("The gRPC endpoint scheme to use, e.g. http or https")
                .num_args(1)
                .help_heading("Endpoint"),
        );
    command
}
