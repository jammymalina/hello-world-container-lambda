FROM rust:1.56-bullseye as build
RUN set -x \
    && apt-get update \
    && apt-get install --no-install-recommends --no-install-suggests -y \
    build-essential \
    libc-dev-arm64-cross gcc-aarch64-linux-gnu \
    && rustup target add aarch64-unknown-linux-gnu
ARG binary
WORKDIR /usr/src/api-service
COPY . .
RUN cargo build --bin ${binary} --release --target aarch64-unknown-linux-gnu --target-dir /usr/bin/api-service

FROM gcr.io/distroless/cc-debian10:latest-arm64
ARG binary
ARG log_level
ARG stage
ENV RUST_LOG=${log_level}
ENV STAGE=${stage}
COPY --from=build /usr/bin/api-service/aarch64-unknown-linux-gnu/release/${binary} /asset-output/bootstrap
ENTRYPOINT [ "/asset-output/bootstrap" ]
