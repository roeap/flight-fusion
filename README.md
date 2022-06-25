# flight-fusion

<p align="center">
<a href="https://github.com/roeap/flight-fusion/actions/workflows/python.yml"><img alt="Actions Status" src="https://github.com/roeap/flight-fusion/actions/workflows/python.yml/badge.svg"></a>
<a href="https://github.com/roeap/flight-fusion/actions/workflows/rust.yml"><img alt="Actions Status" src="https://github.com/roeap/flight-fusion/actions/workflows/rust.yml/badge.svg"></a>
<a href="https://github.com/roeap/flight-fusion/actions/workflows/rust.yml"><img alt="Rustc Version 1.58+" src="https://img.shields.io/badge/rustc-1.58+-lightgray.svg"></a>
<a href="https://github.com/psf/black"><img alt="Code style: black" src="https://img.shields.io/badge/code%20style-black-000000.svg"></a>
<a href="https://codecov.io/gh/roeap/flight-fusion"><img src="https://codecov.io/gh/roeap/flight-fusion/branch/main/graph/badge.svg?token=QI8UWIJ8KY"/></a>
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

- rust 1.58+
- python 3.8+
- poetry (for python development)
- [just](https://github.com/casey/just) (for running workspace commands)

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

The python bindings for rust dependent crates have to be build and installed separately.
Make sure you have the environment created by poetry activated and run.

```sh
make python-develop
```

## Dagster

To see the Dagster integrations in play, create two folders in the project root: `.dagster` and `.fusion`,
then start an instance of the fusion service in a separate shell.

```sh
fusion server start --host 127.0.0.1 --port 50051 --log-level info
```

An example Dagster repository using most of the features from the `dagster-fusion` package is
provided within `scripts/dagster_example.py`. To inspect it and play with the configurations,
run a local instance of dagster.

```sh
DAGSTER_HOME=$(pwd)/.dagster dagit -f scripts/dagster_example.py
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
