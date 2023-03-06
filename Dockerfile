FROM rust:1.67.1-bullseye as builder

RUN apt-get update && apt-get install -y --no-install-recommends protobuf-compiler && rm -rf /var/lib/apt/lists/*

# https://github.com/rust-secure-code/cargo-auditable
RUN cargo install cargo-auditable

WORKDIR /usr/src/prometheus-grpc-bridge
COPY protos protos
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
COPY build.rs .

COPY .git .git

RUN cargo auditable install --path /usr/src/prometheus-grpc-bridge --root /install

COPY examples examples
RUN cargo auditable install --example grpc_server --path /usr/src/prometheus-grpc-bridge --root /install

FROM debian:bullseye-slim

LABEL org.opencontainers.artifact.description = "A server to bridge Prometheus metrics from gRPC to HTTP"
LABEL org.opencontainers.image.vendor = "nyris GmbH"
LABEL org.opencontainers.image.title = "prometheus-grpc-bridge"
LABEL org.opencontainers.image.description = "A server to bridge Prometheus metrics from gRPC to HTTP"
LABEL org.opencontainers.image.base.name = "registry.example.com/nyris/prometheus-grpc-bridge:0.1.0"
LABEL org.opencontainers.image.url="https://github.com/nyris/prometheus-grpc-bridge"
LABEL org.opencontainers.image.documentation="https://github.com/nyris/prometheus-grpc-bridge"
LABEL org.opencontainers.image.source = "https://github.com/nyris/prometheus-grpc-bridge.git"
LABEL org.label-schema.schema-version = "1.0"
LABEL org.label-schema.docker.cmd = "docker run --rm -it -p 80:80 nyris/prometheus-grpc-bridge"

COPY --from=builder /install/bin/prometheus-grpc-bridge /usr/local/bin/prometheus-grpc-bridge
COPY --from=builder /install/bin/grpc_server /usr/local/bin/prometheus-grpc-bridge-example

RUN useradd -ms /bin/bash prometheus-grpc-bridge
USER prometheus-grpc-bridge

WORKDIR /app
COPY README.md .
COPY CHANGELOG.md .

ENV HTTP_SERVER_LOG_STYLE=json
ENV HTTP_SERVER_BIND_ADDRESS=0.0.0.0:80

EXPOSE 80

ENTRYPOINT ["prometheus-grpc-bridge"]
