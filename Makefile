# Dxc Makefile
#
# This Makefile provides convenient shortcuts for common Cargo commands.
# Usage:
#   make help          # Show this help message
#   make check         # Check code for errors (cargo check)
#   make clippy        # Run clippy linter on all targets and features
#   make fmt           # Format code using in-place  (for local development)
#   make fmt-check     # Check if code is formatted (CI-friendly)
#   make test          # Run all tests
#   make doc           # Generate documentation
#   make clean         # Remove all generated artifacts
#   make patch         # Bump patch version: 0.1.0 → 0.1.1
#   make minor         # Bump minor version: 0.1.0 → 0.2.0
#   make release       # Bump major version: 0.1.0 → 1.0.0

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

.PHONY: check
check: ## Check code for errors without building
	cargo check

.PHONY: clippy
clippy: ## Run clippy linter on all targets and features
	cargo clippy --workspace --all-targets --all-features

.PHONY: fmt
fmt: ## Format code using in-place  (for local development)
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Check if code is formatted (CI-friendly)
	cargo fmt --all -- --check

.PHONY: test
test: ## Run all tests
	cargo test --workspace --all-features

.PHONY: doc
doc: ## Generate documentation for private items, excluding dependencies
	cargo doc --no-deps --document-private-items --workspace

.PHONY: clean
clean: ## Remove all generated artifacts (target directory)
	cargo clean

.PHONY: patch
patch: ## Bump patch version: 0.1.0 → 0.1.1 (auto commit/tag/changelog, no push/publish)
	cargo release patch \
		--workspace \
		--no-publish \
		--no-push \
		--no-verify \
		--no-confirm

.PHONY: minor
minor: ## Bump minor version: 0.1.0 → 0.2.0 (auto commit/tag/changelog, no push/publish)
	cargo release minor \
		--workspace \
		--no-publish \
		--no-push \
		--no-verify \
		--no-confirm

.PHONY: major
major: ## Bump major version: 0.1.0 → 1.0.0 (auto commit/tag/changelog, no push/publish)
	cargo release major \
		--workspace \
		--no-publish \
		--no-push \
		--no-verify \
		--no-confirm

.PHONY: release
release: ## Run interactive release (full control, with confirmation)
	cargo release --workspace \
		--no-publish \
		--no-push \
		--verify

# -----------------------------------------------------------------------------
# Convenience Targets
# -----------------------------------------------------------------------------

.PHONY: all
all: check clippy fmt test doc ## Check, clippy, fmt, test, and generate documentation

.PHONY: ci
ci: fmt check clippy test ## Run a full CI-like pipeline locally
	@echo "All checks passed successfully."