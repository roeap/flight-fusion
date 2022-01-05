# flight-fusion

## Documentation

Currently the documentation is not published, but can be viewed in this repository.

```sh
mkdocs serve
```

## Development

To build and install the python bindings

```sh
make python-develop
```

## Build Docker

```sh
docker build -f docker/Dockerfile
```

## Generating python protos

To generate proto definitions for use in python client.

```
make proto
```

Afterwards imports in the generated `message_pb2.py` and `message_pb2.pyi` have to be fixed manually.

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
