FROM rust:1.56.1 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --package flight-fusion --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/flight-fusion /
CMD ["./flight-fusion"]
