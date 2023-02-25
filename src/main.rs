mod commands;

use crate::commands::build_command;
use crate::pb::PrometheusMetricsRequest;
use dotenvy::dotenv;
use pb::prometheus_metrics_client::PrometheusMetricsClient;
use tonic::codec::CompressionEncoding;
use tonic::transport::{Channel, Uri};

mod pb {
    tonic::include_proto!("prometheus");
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let matches = build_command().get_matches();

    let endpoint: &String = matches.get_one("endpoint").unwrap();
    let scheme: &String = matches.get_one("scheme").unwrap();

    let endpoint = format!("{}://{}", scheme, endpoint);
    let endpoint: Uri = endpoint.parse().unwrap();

    let channel = match Channel::builder(endpoint.clone()).connect().await {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to connect to the metric endpoint at {}", endpoint);
            eprintln!("Got error {:?}", e);
            return;
        }
    };

    let mut client = PrometheusMetricsClient::new(channel)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let request = tonic::Request::new(PrometheusMetricsRequest::default());
    let response = client.metrics(request).await.unwrap().into_inner();

    println!("{}", response.text);
}
