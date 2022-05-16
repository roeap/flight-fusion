# Initialize development envoronment
init:
    poetry install --no-root

# build python bindings in development mode
python-develop:
    maturin develop -m python/flight-fusion-server/Cargo.toml --extras=devel $(MATURIN_EXTRA_ARGS)

python-build:
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

    @echo 'tidy up some code'
    black .
    isort .

    @echo 'cleaning up'
    rm -r tmp-proto
    rm -r ./typescript/vscode-fusion/src/generated/scalapb
    rm -r ./typescript/vscode-fusion/src/generated/mlflow

generate-test-data:
    python scripts/generate_test_data.py

check-rust:
    cargo fmt -- --check
    cargo clippy

check-python:
    black --check .
    pyright python/

# run python tests
test-python:
    pytest python/

docker-build:
    docker build -f rust/flight-fusion/Dockerfile -t flight-fusion .
