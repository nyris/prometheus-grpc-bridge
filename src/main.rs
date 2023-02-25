mod commands;

use crate::commands::build_command;
use crate::pb::{PrometheusMetricsRequest, PrometheusMetricsResponse};
use dotenvy::dotenv;
use pb::prometheus_metrics_client::PrometheusMetricsClient;
use std::net::SocketAddr;
use tonic::codec::CompressionEncoding;
use tonic::transport::{Channel, Uri};
use warp::{Filter, Rejection, Reply};

mod pb {
    tonic::include_proto!("prometheus");
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let matches = build_command().get_matches();

    let grpc_address: &String = matches.get_one("grpc_address").unwrap();
    let grpc_scheme: &String = matches.get_one("grpc_scheme").unwrap();
    let grpc_endpoint = format!("{}://{}", grpc_scheme, grpc_address);
    let grpc_endpoint: Uri = grpc_endpoint.parse().unwrap();
    let channel = match Channel::builder(grpc_endpoint.clone()).connect().await {
        Ok(channel) => channel,
        Err(e) => {
            eprintln!("Failed to connect to the gRPC metric endpoint at {grpc_endpoint}");
            eprintln!("Got error {e:?}");
            return;
        }
    };

    // If the serve argument is provided, host the service, otherwise print directly.
    let serve_address: Option<&SocketAddr> = matches.get_one("serve_address");
    if let Some(&address) = serve_address {
        let metrics = warp::path!("metrics")
            .and(warp::any().map(move || channel.clone()))
            .and_then(metrics);
        warp::serve(metrics).run(address).await;
    } else {
        fetch_and_print(channel).await;
    }
}

async fn fetch_and_print(channel: Channel) {
    let response = get_metrics_from_server(channel).await;
    println!("{}", response.text);
}

async fn metrics(channel: Channel) -> Result<impl Reply, Rejection> {
    let response = get_metrics_from_server(channel).await;
    Ok(response.text)
}

async fn get_metrics_from_server(channel: Channel) -> PrometheusMetricsResponse {
    let mut client = PrometheusMetricsClient::new(channel)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let request = tonic::Request::new(PrometheusMetricsRequest::default());
    let response = client.metrics(request).await.unwrap().into_inner();
    response
}
