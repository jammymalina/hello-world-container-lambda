FROM rust:1.55 as build
ARG binary
WORKDIR /usr/src/api-service
COPY . .
RUN cargo install --bin ${binary} --path .

FROM gcr.io/distroless/cc-debian10
ARG binary
COPY --from=build /usr/local/cargo/bin/${binary} /usr/local/bin/${binary}
CMD [ ${binary} ]
