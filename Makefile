.DEFAULT_GOAL := help

MATURIN_VERSION := $(shell awk -F '[ ="]+' '$$1 == "requires" { print $$4 }' pyproject.toml)
PY_PACKAGE_VERSION := $(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)
PROTO_DEST := "python/flight-fusion/flight_fusion/proto"

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: init
init: ## Initialize development envoronment
	$(info --- initialize developmemnt environment ---)
	poetry install --no-root

.PHONY: gen-test-data
gen-test-data:
	python scripts/generate_test_data.py

.PHONY: generate-openmeta
generate-openmeta:
	docker run --rm -v $(PWD):/local openapitools/openapi-generator-cli generate \
		-i /local/rust/open-metadata/swagger.json \
		-g rust -o /local/out/open-metadata \
		--package-name openmetadata \
		--additional-properties=useSingleRequestParameter=true \
		--skip-validate-spec

.PHONY: generate-types
generate-types:
	node scripts/generate-openmeta-types.js

.PHONY: build-docker-fusion
build-docker-fusion:
	$(info Build flight fusion container)
	docker build -f rust/flight-fusion/Dockerfile -t flight-fusion .

.PHONY: python-develop
python-develop: ## Run check on Python
	$(info Dev build for python bindings)
	cd python/flight-fusion-server && maturin develop --extras=devel $(MATURIN_EXTRA_ARGS)

.PHONY: python-check
python-check: ## Run check on Python
	$(info Check Python black)
	black --check .
	$(info Check Python pyright)
	pyright python/

.PHONY: python-build
python-build: ## Build Python binding of flight fusion
	$(info --- Build Python binding ---)
	cd python/flight-fusion && maturin build --release --no-sdist --strip

.PHONY: python-proto
python-proto:
	$(info Generate python protobuffers)
	mkdir tmp-proto
	python -m grpc_tools.protoc -I proto --python_betterproto_out=tmp-proto proto/common.proto proto/message.proto proto/signals.proto proto/flight.proto
	mv -f ./tmp-proto/flight_fusion/ipc/v1alpha1/* ./python/flight-fusion/flight_fusion/ipc/v1alpha1/
	rm -rf ./tmp-proto
	black --line-length 100 python/flight-fusion/flight_fusion/
	isort python/flight-fusion/flight_fusion/

.PHONY: python-test
python-test: ## Run check on Rust
	pytest python/flight-fusion/tests
	pytest python/dagster-fusion/tests

.PHONY: rust-check
rust-check: ## Run check on Rust
	$(info --- Check Rust clippy ---)
	cargo clippy
	$(info --- Check Rust format ---)
	cargo fmt -- --check

.PHONY: rust-test-integration
rust-test-integration:
	cargo test --package flight-fusion-client --tests --features integration

.PHONY: run
run: ## run most recent build of flight fusion
	./target/debug/flight-fusion
