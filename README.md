# flight-fusion

<p align="center">
<a href="https://github.com/roeap/flight-fusion/actions/workflows/rust.yml"><img alt="Actions Status" src="https://github.com/roeap/flight-fusion/actions/workflows/rust.yml/badge.svg"></a>
<a href="https://github.com/roeap/flight-fusion/actions/workflows/rust.yml"><img alt="Rustc Version 1.57+" src="https://img.shields.io/badge/rustc-1.57+-lightgray.svg"></a>
<a href="https://github.com/psf/black"><img alt="Code style: black" src="https://img.shields.io/badge/code%20style-black-000000.svg"></a>
</p>

This repository contains some experiments on what a modern data platform could look like.
A strong emphasis lies on how observability can be achieved throughout all actions
occurring on a data platform.

## Documentation

Currently the documentation is not published, but can be viewed in this repository.

```sh
mkdocs serve
```

## Development

### Prerequisites

- rust>=1.57
- poetry (for python development)

### Rust

To run unit tests for the rust crates execute

```sh
cargo test
```

To build crates and services

```sh
cargo build
```

TODO explain mock framework

## Python Development

To set up the python environment run

```sh
poetry install
```

The python bindings fofr rust dependent crates have to be build and installed separately.
Make sure you have the environment created by poetry activated and run.

```sh
make python-develop
```

## Build Docker

```sh
docker build -f docker/Dockerfile
```

```sh
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

### Generating python protos

To generate proto definitions for use in python client.

```
make python-proto
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
