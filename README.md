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
