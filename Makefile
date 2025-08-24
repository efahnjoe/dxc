# Dxc Makefile
#
# This Makefile provides convenient shortcuts for common Cargo commands.
# Usage:
#   make help          # Show this help message
#   make build         # Build debug version
#   make release       # Build optimized release version
#   make check         # Check code for errors (cargo check)
#   make clippy        # Run clippy linter on all targets and features
#   make fmt           # Format code with rustfmt
#   make test          # Run all tests
#   make doc           # Generate documentation
#   make clean         # Remove all generated artifacts

# -----------------------------------------------------------------------------
# Variables (customize as needed)
# -----------------------------------------------------------------------------

# Optional: Define features if commonly used
# FEATURES = postgres redis

# -----------------------------------------------------------------------------
# Main Targets
# -----------------------------------------------------------------------------

.PHONY: help
help: ## Show this help message
	@echo "Available targets:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build debug version
	cargo build

.PHONY: release
release: ## Build optimized release version
	cargo build --release

.PHONY: check
check: ## Check code for errors without building
	cargo check

.PHONY: clippy
clippy: ## Run clippy linter on all targets and features
	cargo clippy --all-targets --all-features

.PHONY: fmt
fmt: ## Format code using rustfmt
	cargo fmt

.PHONY: test
test: ## Run all tests
	cargo test

.PHONY: doc
doc: ## Generate documentation for private items, excluding dependencies
	cargo doc --no-deps --document-private-items --workspace

.PHONY: clean
clean: ## Remove all generated artifacts (target directory)
	cargo clean

# -----------------------------------------------------------------------------
# Convenience Targets
# -----------------------------------------------------------------------------

.PHONY: all
all: build test doc ## Build, test, and generate documentation

.PHONY: ci
ci: fmt check clippy test ## Run a full CI-like pipeline locally
	@echo "✅ All checks passed successfully."