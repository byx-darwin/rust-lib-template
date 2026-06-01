.DEFAULT_GOAL := help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Compile the project
	@cargo build

test: ## Run tests with nextest
	@cargo nextest run --all-features

fmt: ## Check code formatting with nightly rustfmt
	@cargo +nightly fmt -- --check

clippy: ## Lint with pedantic clippy rules
	@cargo clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

lint: fmt clippy ## Run fmt and clippy

check-agent-sync: ## Verify CLAUDE.md exists
	@test -f CLAUDE.md || { \
		echo "CLAUDE.md is required for project-level agent instructions."; \
		exit 1; \
	}

release: ## Tag and publish a release with cargo-release and git-cliff
	@cargo release tag --execute
	@git cliff -o CHANGELOG.md
	@git commit -a -n -m "Update CHANGELOG.md" || true
	@git push origin master
	@cargo release push --execute

update-submodule: ## Update git submodules recursively
	@git submodule update --init --recursive --remote

.PHONY: help build test fmt clippy lint check-agent-sync release update-submodule
