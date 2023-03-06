# Prometheus gRPC Bridge

A Prometheus metrics endpoint bridging data from a gRPC service,
for when you cannot host gRPC and HTTP at the same time.

## HTTP /metrics endpoint

The server supports both HTTP/1.1 and HTTP/2 transparently without
requiring a secure connection to perform ALPN.

For a complete example, see [`docker-compose.yml`]. You can start
both the bridge and an example server by running:

```shell
docker compose up
```

To instead host the service locally, run e.g.

```shell
cargo run -- --serve 0.0.0.0:8080
```

Afterwards, you can fetch metrics using e.g. `curl` (HTTP/1.1) or `nghttp` (HTTP/2 prior knowledge):

```shell
curl http://localhost:8080/metrics
nghttp http://localhost:8080/metrics
````

The server's log output can be controlled using the [`RUST_LOG`] environment variable.
To switch between plain and JSON log output, provide the `--log simple` or
`--log json` argument respectively:

```shell
cargo run -- --serve 0.0.0.0:8080 --log json
```

Note that logging of debug level and below is disabled in release builds.

## Protocol

The gRPC protocol is kept simple and assumes the metrics can already
be provided in Prometheus text format as described in the [Exposition formats]
section. See the [`prometheus.proto`] file for the protocol:

```protobuf
syntax = "proto3";

package prometheus;

service PrometheusMetrics {
  rpc Metrics(PrometheusMetricsRequest) returns (PrometheusMetricsResponse) {}
}

message PrometheusMetricsRequest {
}

message PrometheusMetricsResponse {
  // The metrics as a string in Prometheus text format.
  string text = 1;
}
```

### Example server

This project comes with a Rust example server based on Tonic.
To run the server defined in [`examples/grpc_server.rs`], execute

```shell
cargo run --example grpc_server
```

It provides a simple instance metric and a gauge that increments
with every request. To quickly poll the server using the default settings, run:

```curl
cargo run
```

Similarly, you can inspect the gRPC call using `grpcurl`:

```shell
grpcurl -plaintext localhost:11000 prometheus.PrometheusMetrics/Metrics
```

[Exposition formats]: https://github.com/prometheus/docs/blob/0ac960bbc57d9a229848f785934455c0f6344a9c/content/docs/instrumenting/exposition_formats.md
[`prometheus.proto`]: protos/prometheus.proto
[`RUST_LOG`]: https://docs.rs/env_logger/0.10.0/env_logger/#enabling-logging
[`examples/grpc_server.rs`]: examples/grpc_server.rs
[`docker-compose.yml`]: docker-compose.yml

## License

If not mentioned otherwise in a file, the content provided is
licensed under the EUPL v1.2, a copy of which can be found in 
[LICENSE.md](LICENSE.md).
