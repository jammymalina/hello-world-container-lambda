FROM rust:1.55 as build
ARG binary
WORKDIR /usr/src/api-service
COPY . .
RUN cargo install --bin ${binary} --path .

FROM gcr.io/distroless/cc-debian10
ARG binary
ARG log_level
ENV RUST_LOG=${log_level}
COPY --from=build /usr/local/cargo/bin/${binary} /asset-output/bootstrap
ENTRYPOINT [ "/asset-output/bootstrap" ]
