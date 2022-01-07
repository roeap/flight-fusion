# flight-fusion

[![CI](https://github.com/roeap/flight-fusion/actions/workflows/rust.yml/badge.svg)](https://github.com/roeap/flight-fusion/actions/workflows/rust.yml)

This repository contains some experiments on what a modern data platform could look like.
A strong emphasis lies on how observability can be achieved throughout all actions
occurring on a data platform.

## Documentation

Currently the documentation is not published, but can be viewed in this repository.

```sh
mkdocs serve
```

## Development

To run unit tests for the rust crates execute

```sh
cargo test
```

To build and install the python bindings run

```sh
make python-develop
```

### Build Docker

```sh
docker build -f docker/Dockerfile
```

```sh
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

### Generating python protos

To generate proto definitions for use in python client.

```
make proto
```

## Known issues

Right now the current release of the `quote` crate breaks the build. Until the dependent crates have addressed
the underlying issue, the version of the quote crate has to be manually pinned.

```sh
cargo update --package quote --precise 1.0.10
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
