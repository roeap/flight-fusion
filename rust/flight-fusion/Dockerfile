FROM rust:1.58.0 as build-env

WORKDIR /app
COPY ./rust /app/rust
COPY ./proto/ /app/proto/
COPY Cargo.docker.toml /app/Cargo.toml
RUN cargo build --package flight-fusion --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/flight-fusion /opt/fusion/
COPY ./config/ /opt/fusion/config/

WORKDIR /opt/fusion/

CMD ["./flight-fusion"]
