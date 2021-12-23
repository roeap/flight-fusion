# OpenMetadata Client for Rust

This crate contains a client implementation for communicating with the APIs defined
for the OpenMetadata service.

Technologically, the client implementation is heavily inspired by the pipelines architecture
from azure sdk for rust. In fact the very core is more or less the same with the Azure specific parts removed.

The overall structure of the clients is designed to mimic the "official"
[structure](https://docs.open-metadata.org/openmetadata/apis/api-organization) of the API, providing thematic
clients for data assets (`DataAssetClient`), Services (`ServicesClient`), and users (`UsersClient`).

## Example

```rs
use open_metadata::prelude::*;


```

## Usage

```toml
[dependencies]
open-metadata = { version = "0.1", git = "https://github.com/roeap/flight-fusion" }
```
