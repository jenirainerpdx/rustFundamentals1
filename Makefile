SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:' Makefile | sed 's/:.*#/#/' | column -t -s '#'

clean: ## Clean the project using cargo. Also runs format.
	@rustup component add rustfmt 2> /dev/null
	cargo clean
	cargo fmt -- --check

build: ## Build the project using cargo
	cargo build

test: ## Run tests using cargo
	cargo test

fmt: ## Format the code
	@rustup component add rustfmt 2> /dev/null
	cargo fmt -- --check

lint: ## Check code style and check for errors and warnings. Also checks formatting.
	@rustup component add clippy 2> /dev/null
	cargo fmt -- --check
	cargo clippy -- -D warnings

doc: ## Generate documentation
	@rustup component add rust-docs 2> /dev/null
	cargo doc

make run: ## Run the project
	cargo run
	