# Prometheus gRPC Bridge

A Prometheus metrics endpoint bridging data from a gRPC service,
for when you cannot host gRPC and HTTP at the same time.

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

[Exposition formats]: https://github.com/prometheus/docs/blob/0ac960bbc57d9a229848f785934455c0f6344a9c/content/docs/instrumenting/exposition_formats.md
[`prometheus.proto`]: protos/prometheus.proto
