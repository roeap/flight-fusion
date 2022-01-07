FROM rust:1.57.0 as build-env
WORKDIR /app
COPY ./rust /app/rust
COPY ./proto/ /app/proto/
COPY Cargo.docker.toml /app/Cargo.toml
# TODO remove once dependency pins are fixed
RUN cargo update --package quote --precise 1.0.10
RUN cargo build --package flight-fusion --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/flight-fusion /
CMD ["./flight-fusion"]
