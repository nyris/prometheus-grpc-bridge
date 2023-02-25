use anyhow;
use std::env;
use std::path::PathBuf;
use vergen::{vergen, Config, ShaKind, TimestampKind};

fn main() -> anyhow::Result<()> {
    build_versions()?;
    build_proto()?;
    Ok(())
}

fn build_proto() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .build_server(true) // for the samples
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("prometheus_descriptor.bin"))
        .compile(&["./protos/prometheus.proto"], &["./protos"])?;
    Ok(())
}

fn build_versions() -> anyhow::Result<()> {
    let mut config = Config::default();
    *config.build_mut().timestamp_mut() = true;
    *config.build_mut().kind_mut() = TimestampKind::DateOnly;
    *config.git_mut().sha_mut() = true;
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen(config)?;
    Ok(())
}
