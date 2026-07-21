TOML_FILE := Cargo.toml

# Extract version from TOML
VERSION := $(shell sed -n 's/^version *= *"\(.*\)"/\1/p' $(TOML_FILE))

# Default: bump patch
PATCH_VERSION := $(shell \
	echo $(VERSION) | awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}' \
)

# Allow override
NEW_VERSION ?= $(PATCH_VERSION)

.PHONY: version tag release


version:
	@echo "Current version: $(VERSION)"
	# Update version in Cargo.toml
	@sed -i.bak 's/^version *= *".*"/version = "$(NEW_VERSION)"/' $(TOML_FILE)
	@rm -f $(TOML_FILE).bak
	@echo "Release version: $(NEW_VERSION)"
	cargo check
tag:
	@git tag -a v$(NEW_VERSION) -m "Release v$(NEW_VERSION)"
	@git push origin v$(NEW_VERSION)

package:
	@echo packaging crate
	git add $(TOML_FILE) Cargo.lock
	@git commit -m "Bump version to v$(NEW_VERSION)"
	@git push
	echo added git
	@cargo package

release: version package tag
	@echo "Creating GitHub release v$(NEW_VERSION)"
	@gh release create v$(NEW_VERSION) \
		--title "v$(NEW_VERSION)" \
		--notes "Release v$(NEW_VERSION)"
	# @echo "Creating crate release v$(NEW_VERSION)"
	# @cargo publish

lint: 
	cargo clippy --benches --examples --tests -- -D warnings 
fmt:
	cargo clippy --workspace --all-features --all-targets --tests --fix --allow-dirty -- -D warnings
	cargo fmt --all
build:
	cargo build --all-features
test:
	cargo test -- --nocapture
run:
	cargo run --all-features

codegen:

	curl https://v3.docs.derive.xyz/openapi.json | yq '.' > openapi.json

	openapi-generator-cli version
# 	python build_script/patch_spec.py

	# REST API generation
	openapi-generator-cli generate \
	  -i openapi.json \
	  -g rust \
	  -o ./generated \
	  --type-mappings decimal=bigdecimal::BigDecimal \
	  --additional-properties=supportAsync=true,useSingleRequestParameter=true,avoidBoxedModels=true,generateAliasAsModel=false,preserveOriginalNames=true,supportNullable=true,library=reqwest-trait,topLevelApiClient=true,reqwestDefaultFeatures=rustls-tls-webpki-roots \
 	  --skip-validate-spec	
	rm -rf ./src/models/*
	cp ./generated/src/models/* ./src/models/
	# cp -r ./generated/src/apis ./src
	rm -rf ./generated

	# WebSocket OpenAPI generation (ticker)
	# redocly bundle ws_openapi.json -o bundled.yaml --ext yaml
	# openapi-generator-cli generate \
	  # -i bundled.yaml \
	  # -g rust \
	  # -o ./generated \
	  # --type-mappings decimal=bigdecimal::BigDecimal \
	  # --additional-properties=supportAsync=true,useSingleRequestParameter=true,avoidBoxedModels=true,generateAliasAsModel=false,preserveOriginalNames=true,supportNullable=true,library=reqwest-trait,topLevelApiClient=true,reqwestDefaultFeatures=rustls-tls-webpki-roots 
# #  	  --skip-validate-spec

	# cp ./generated/src/models/interval.rs src/models/interval.rs
	# cp ./generated/src/models/group.rs src/models/group.rs
	# cp ./generated/src/models/depth.rs src/models/depth.rs
	# cp ./generated/src/models/orderbook_*.rs src/models/
	# cp ./generated/src/models/*balance* src/models/
	# cp ./generated/src/models/*notification* src/models/
	cp build_script/models/ticker_slim_schema.rs src/models/ticker_slim_schema.rs
	# rm -rf bundled.yaml generated

	# python build_script/post_processing.py


all: codegen fmt lint build test
