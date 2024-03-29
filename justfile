# Initialize development envoronment
init:
    poetry install --no-root

build-py-object-store:
    maturin develop -m python/fusion-object-store/Cargo.toml --extras=devel $(MATURIN_EXTRA_ARGS)

# build python bindings in development mode
build-py-server:
    maturin develop -m python/flight-fusion-server/Cargo.toml --extras=devel $(MATURIN_EXTRA_ARGS)

release-py-server:
    maturin build -m python/flight-fusion-server/Cargo.toml --release --no-sdist --strip

# compile proto files
generate-proto:
    @echo 'generating proto'
    buf generate proto/flight_fusion
    buf generate proto/inference
    buf generate proto/mlflow

    @echo 'moving some files around'
    rsync -a tmp-proto/flight_fusion/ipc/v1alpha1/ python/flight-fusion/flight_fusion/ipc/v1alpha1/
    rsync -a tmp-proto/inference/ python/flight-fusion/flight_fusion/ipc/inference/
    rsync -a tmp-proto/mlflow/ python/flight-fusion/flight_fusion/ipc/mlflow/
    rsync -a tmp-proto/mlflow/ python/mlflow-fusion/mlflow_fusion/ipc/

    @echo 'tidy up some code'
    black .
    isort --profile=black .

    @echo 'cleaning up'
    rm -r tmp-proto
    rm rust/flight-fusion-ipc/src/gen/mlflow.tonic.rs
    rm rust/flight-fusion-ipc/src/gen/mlflow.rs
    rm rust/flight-fusion-ipc/src/gen/inference.model_repository.rs
    rm rust/flight-fusion-ipc/src/gen/inference.model_repository.tonic.rs
    rm rust/flight-fusion-ipc/src/gen/inference.rs
    rm rust/flight-fusion-ipc/src/gen/inference.tonic.rs

generate-test-data:
    python scripts/generate_test_data.py

generate-helm:
    mlfusion-helm schema apply

check-rust:
    cargo fmt -- --check
    cargo clippy

check-python:
    black --check .
    pyright python/

# run python tests
test-python:
    pytest python/

build-docker:
    docker build -f docker/fusion.Dockerfile -t flight-fusion .

# run flight fusion server
run-fusion:
    docker-compose --profile fusion up

# run example app
run-app:
    python -m app

# run the documentation
run-docs:
    mkdocs serve

run-mlflow:
    docker-compose --profile mlflow up
