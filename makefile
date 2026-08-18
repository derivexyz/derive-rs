TOML_FILE := Cargo.toml
MASTER_BRANCH := master
DEV_BRANCH := dev

VERSION := $(shell sed -n 's/^version *= *"\(.*\)"/\1/p' $(TOML_FILE))

.PHONY: dev-version release tag lint fmt build test run codegen all


#
dev-version:
	@set -e; \
	git fetch origin $(MASTER_BRANCH); \
	CURRENT_VERSION=$$(sed -n 's/^version *= *"\(.*\)"/\1/p' $(TOML_FILE)); \
	MASTER_VERSION=$$(git show origin/$(MASTER_BRANCH):$(TOML_FILE) \
		| sed -n 's/^version *= *"\(.*\)"/\1/p'); \
	echo "dev version:    $$CURRENT_VERSION"; \
	echo "master version: $$MASTER_VERSION"; \
	if [ "$$CURRENT_VERSION" != "$$MASTER_VERSION" ]; then \
		echo "dev has already been versioned; nothing to do."; \
		exit 0; \
	fi; \
	NEW_VERSION=$$(echo "$$CURRENT_VERSION" \
		| awk -F. '{printf "%d.%d.%d", $$1, $$2, $$3+1}'); \
	echo "Bumping $$CURRENT_VERSION -> $$NEW_VERSION"; \
	sed -i.bak \
		's/^version *= *".*"/version = "'"$$NEW_VERSION"'"/' \
		$(TOML_FILE); \
	rm -f $(TOML_FILE).bak; \
	cargo check; \
	git add $(TOML_FILE) Cargo.lock; \
	git commit -m "Bump version to v$$NEW_VERSION"; \
	git push origin HEAD:$(DEV_BRANCH)


release:
	@set -e; \
	VERSION=$$(sed -n 's/^version *= *"\(.*\)"/\1/p' $(TOML_FILE)); \
	echo "Releasing v$$VERSION"; \
	git tag -a "v$$VERSION" -m "Release v$$VERSION"; \
	git push origin "v$$VERSION"; \
	gh release create "v$$VERSION" \
		--title "v$$VERSION" \
		--notes "Release v$$VERSION"


lint:
	cargo clippy --benches --examples --tests -- -D warnings

fmt:
	cargo clippy --workspace --all-features --all-targets --tests --fix --allow-dirty -- -D warnings
	cargo fmt --all

build:
	cargo build --all-features

test:
	cargo test -- --nocapture --test-threads 1

run:
	cargo run --all-features

codegen:
	curl https://v3.docs.derive.xyz/openapi.json | yq '.' > schemas/openapi.json
	curl https://v3.docs.derive.xyz/websocket.asyncapi.json | yq '.' > schemas/ws_asyncapi_rpc.json
	curl https://v3.docs.derive.xyz/subscriptions.asyncapi.json | yq '.' > schemas/ws_asyncapi_subscriptions.json
	redocly lint schemas/openapi.json --lint-config=error
	cargo build

all: codegen build fmt lint test