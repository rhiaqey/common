.PHONY: dev
dev: build

.PHONY: build
build:
	cargo build --all-features

.PHONY: prod
prod:
	cargo build --release --all-features

.PHONY: test
test:
	cargo test

.PHONY: format
format:
	cargo fmt
