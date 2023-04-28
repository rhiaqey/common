.PHONY: dev
dev: build

.PHONY: build
build:
	cargo build

.PHONY: prod
prod:
	cargo build --release

.PHONY: test
test:
	cargo test
