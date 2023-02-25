use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true) // for the samples
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("prometheus_descriptor.bin"))
        .compile(&["./protos/prometheus.proto"], &["./protos"])?;

    Ok(())
}
