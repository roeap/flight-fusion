FROM rust:1.62 as build-env

RUN apt-get update && apt-get install -y cmake

WORKDIR /app
COPY ./rust /app/rust
COPY ./proto/ /app/proto/
COPY Cargo.docker.toml /app/Cargo.toml
RUN cargo build --package flight-fusion --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/flight-fusion /opt/fusion/

WORKDIR /opt/fusion/

CMD ["./flight-fusion"]
