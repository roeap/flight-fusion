# flight-fusion

```sh
docker build -f docker/Dockerfile
```

## Generating python protos

To generate proto definitions for use in python client.

```
make proto
```

Afterwards imports in the generated `message_pb2.py` and `message_pb2.pyi` have to be fixed manually.
