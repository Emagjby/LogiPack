SHELL := /bin/bash

.PHONY: help test fmt clippy dev-api dev-web build-web

help:
	@echo "Targets:"
	@echo "  test      - cargo test --workspace"
	@echo "  fmt       - cargo fmt --all"
	@echo "  clippy    - cargo clippy --workspace --all-targets --all-features"
	@echo "  dev-api   - run hub API"
	@echo "  dev-web   - run SvelteKit dev server"
	@echo "  build-web - build SvelteKit"

test:
	cargo test --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets --all-features

dev-api:
	cargo run -p hub-api

dev-web:
	cd logipack-hub/hub-web && bun run dev

build-web:
	cd logipack-hub/hub-web && bun run build
