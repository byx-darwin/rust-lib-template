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

install-tools: ## Install development toolchain (pre-commit, cargo-deny, cargo-audit, typos, gitleaks)
	@pip install pre-commit 2>/dev/null || echo "Install pre-commit manually: https://pre-commit.com/#install"
	@cargo install cargo-deny --locked 2>/dev/null || echo "cargo-deny already installed or install manually"
	@cargo install cargo-audit --locked 2>/dev/null || echo "cargo-audit already installed or install manually"
	@cargo install typos-cli 2>/dev/null || echo "typos already installed or install manually"
	@which gitleaks >/dev/null 2>&1 || echo "Install gitleaks: https://github.com/gitleaks/gitleaks#installing"
	@pre-commit install
	@echo "Development tools installed. Run 'pre-commit run --all-files' to verify."

bench: ## Run benchmarks with Criterion
	@cargo bench --workspace

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

.PHONY: help build test fmt clippy lint install-tools bench check-agent-sync release update-submodule
