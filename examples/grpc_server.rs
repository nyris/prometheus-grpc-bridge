//! This example hosts a gRPC server implementing the simple Prometheus metrics
//! protocol defined in `../protos/prometheus.proto`.

use pb::prometheus_metrics_server::{PrometheusMetrics, PrometheusMetricsServer};
use pb::{PrometheusMetricsRequest, PrometheusMetricsResponse};
use prometheus::{default_registry, Encoder, IntGauge, Opts, TextEncoder};
use tonic::codec::CompressionEncoding;
use tonic::{Request, Response, Status};
use tracing::info;

lazy_static::lazy_static! {
    static ref APP_INSTANCES_METRIC: IntGauge = {
        let metric = IntGauge::with_opts(Opts::new("application_instance", "Number of instances of this application")
            .const_label("app", "grpc-server-example")
            .const_label("app_version", env!("CARGO_PKG_VERSION"))
        ).unwrap();
        default_registry().register(Box::new(metric.clone())).unwrap();
        metric.inc();
        metric
    };

    static ref METRICS_CALLS: IntGauge = {
        let metric = IntGauge::with_opts(Opts::new("metrics_calls", "Number of calls to the metrics service")
            .const_label("app", "grpc-server-example")
            .const_label("app_version", env!("CARGO_PKG_VERSION"))
        ).unwrap();
        default_registry().register(Box::new(metric.clone())).unwrap();
        metric
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the static metric.
    let _ = APP_INSTANCES_METRIC.get();

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(PrometheusService::FILE_DESCRIPTOR_SET)
        .build()?;

    let server = tonic::transport::Server::builder()
        .add_service(reflection_server)
        .add_service(PrometheusService::new_server());
    server.serve("0.0.0.0:11000".parse()?).await?;
    Ok(())
}

#[derive(Debug)]
pub struct PrometheusService {}

/// Imported protocol buffer and gRPC service definitions.
mod pb {
    tonic::include_proto!("prometheus");
}

impl PrometheusService {
    pub const FILE_DESCRIPTOR_SET: &'static [u8] =
        tonic::include_file_descriptor_set!("prometheus_descriptor");

    pub fn new_server() -> PrometheusMetricsServer<Self> {
        PrometheusMetricsServer::new(Self {})
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip)
    }
}

#[tonic::async_trait]
impl PrometheusMetrics for PrometheusService {
    async fn metrics(
        &self,
        _request: Request<PrometheusMetricsRequest>,
    ) -> Result<Response<PrometheusMetricsResponse>, Status> {
        info!("Prometheus metrics request received");
        METRICS_CALLS.inc();

        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();

        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        let output = String::from_utf8(buffer.clone()).unwrap();
        Ok(Response::new(PrometheusMetricsResponse { text: output }))
    }
}
