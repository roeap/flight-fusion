.DEFAULT_GOAL := help

MATURIN_VERSION := $(shell awk -F '[ ="]+' '$$1 == "requires" { print $$4 }' pyproject.toml)
PY_PACKAGE_VERSION := $(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)
PROTO_DEST := "python/flight-fusion/flight_fusion/proto"

.PHONY: init
init: ## Initialize development envoronment
	$(info --- initialize developmemnt environment ---)
	poetry install --no-root

.PHONY: build
build: ## Build Python binding of arrow-azure-fs
	echo $(MATURIN_VERSION)
	$(info --- Build Python binding ---)
	cd python/arrow-azure-fs && maturin build $(MATURIN_EXTRA_ARGS)

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: proto
proto:
	python -m grpc_tools.protoc -I proto --mypy_out $(PROTO_DEST) --python_out $(PROTO_DEST) proto/message.proto proto/actions.proto proto/tickets.proto proto/common.proto

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
	cd python/flight-fusion && maturin develop $(MATURIN_EXTRA_ARGS)

.PHONY: python-check
python-check: ## Run check on Python
	$(info Check Python isort)
	isort --diff --check-only .
	$(info Check Python black)
	black --check

.PHONY: python-build
python-build: ## Build Python binding of flight fusion
	$(info --- Build Python binding ---)
	cd python/flight-fusion && maturin build --release --no-sdist --strip

.PHONY: rust-test-integration
rust-test-integration:
	cargo test --package flight-fusion-client --tests --features integration
