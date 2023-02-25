FROM rust:1.67.1-bullseye as builder

RUN echo "deb http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-14 main" > /etc/apt/sources.list.d/clang.list
RUN echo "deb-src http://apt.llvm.org/bullseye/ llvm-toolchain-bullseye-14 main" >> /etc/apt/sources.list.d/clang.list
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -
RUN apt-get update && apt-get install -y --no-install-recommends g++-11 clang-14 && rm -rf /var/lib/apt/lists/*

RUN ln -s "$(which clang-14)" /usr/bin/clang

# https://github.com/rust-secure-code/cargo-auditable
RUN cargo install cargo-auditable cargo-audit

# We require the library during build to bake in its version.
COPY kernel_io-sys/vendor/kernel_io/lib/libkernel_io.so /usr/local/lib
RUN ldconfig

WORKDIR /usr/src/convert
COPY convert convert
COPY kernel_io kernel_io
COPY kernel_io-sys kernel_io-sys
COPY Cargo.toml .
COPY Cargo.lock .

COPY .git .git

ARG KERNEL_IO_LICENSE
ENV COMPILE_KERNEL_IO_LICENSE=$KERNEL_IO_LICENSE

RUN cargo auditable install --path /usr/src/convert/convert

FROM debian:bullseye-slim
# FROM ubuntu:jammy

LABEL org.opencontainers.image.vendor = "nyris GmbH"
LABEL org.opencontainers.image.title = "cad-convert-cli"
LABEL org.opencontainers.image.description = "A CLI to convert CAD files to glTF/glb"
LABEL org.opencontainers.artifact.description = "A CLI to convert CAD files to glTF/glb"
LABEL org.opencontainers.image.url="https://git.nyris.io/vision/threed_kernel_io"
LABEL org.opencontainers.image.documentation="https://git.nyris.io/vision/threed_kernel_io"
LABEL org.opencontainers.image.source = "[git@git.nyris.io:10022]:vision/threed_kernel_io.git"
LABEL org.label-schema.schema-version = "1.0"
LABEL org.label-schema.docker.cmd = "docker run --rm -it -v /host/input:/input:ro -v /host/output:/output:rw eu.gcr.io/everybag-1273/cad-convert-cli --input-dir /input --output-dir /output"

# Install setcap (see below)
# RUN apt-get update && apt-get install -y --no-install-recommends libcap2-bin && rm -rf /var/lib/apt/lists/*

COPY kernel_io-sys/vendor/kernel_io/lib/libkernel_io.so /usr/local/lib
RUN ldconfig

COPY --from=builder /usr/local/cargo/bin/convert /usr/local/bin/convert

# Give the application permissions to change the process niceness,
# https://serverfault.com/questions/96742/give-pthread-setschedparam-permissions-to-non-root-user-linux
#
# Note that this requires `docker run --cap-add=sys_nice ...`
# RUN /sbin/setcap 'cap_sys_nice=eip' /usr/local/bin/convert

RUN useradd -ms /bin/bash cad-convert
USER cad-convert

WORKDIR /app
COPY CHANGELOG.md .
COPY "kernel_io-sys/vendor/kernel_io/Disclaimer.txt"  3D_Kernel_IO_Disclaimer.txt
ENTRYPOINT ["convert"]
CMD ["--help"]
