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
	cargo +nightly fmt --all
build:
	cargo build --all-features
test:
	cargo test -- --nocapture
run:
	cargo run --all-features

codegen:
	curl https://v3.docs.derive.xyz/openapi.json | yq '.' > schemas/openapi.json
	curl https://v3.docs.derive.xyz/websocket.asyncapi.json | yq '.' > schemas/ws_asyncapi_rpc.json
	curl https://v3.docs.derive.xyz/subscriptions.asyncapi.json | yq '.' > schemas/ws_asyncapi_subscriptions.json
	redocly lint schemas/openapi.json --lint-config=error
	cargo build

all: codegen build fmt lint test
