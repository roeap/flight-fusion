FROM rust:1.63 as build-env

RUN apt-get update && apt-get install -y cmake curl unzip

ENV PROTOC_ZIP=protoc-21.4-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v21.4/${PROTOC_ZIP}
RUN unzip -o ${PROTOC_ZIP} -d ./proto
RUN chmod 755 -R ./proto/bin
ENV BASE=/usr
# Copy into path
RUN cp ./proto/bin/protoc ${BASE}/bin/
RUN cp -R ./proto/include/* ${BASE}/include/

WORKDIR /app
COPY ./rust /app/rust
COPY ./proto/ /app/proto/
COPY Cargo.docker.toml /app/Cargo.toml
RUN cargo build --package flight-fusion --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/flight-fusion /opt/fusion/

WORKDIR /opt/fusion/

CMD ["./flight-fusion"]
