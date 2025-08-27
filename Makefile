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
#   make release-ci    # CI-friendly release (no confirmation)

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
patch: ## Bump patch version: 0.1.0 → 0.1.1, tag, update changelog (no push, no publish)
	cargo release patch \
		--workspace \
		--no-publish \
		--no-push \
		--no-verify \
		--no-confirm \
		--pre-release-commit-message="chore: release v{{version}}" \
		--tag-message="v{{version}}" \
		--tag-prefix="v"

.PHONY: minor
minor: ## Bump minor version: 0.1.0 → 0.2.0, tag, update changelog (no push, no publish)
	cargo release minor \
		--workspace \
		--no-publish \
		--no-push \
		--no-verify \
		--no-confirm \
		--pre-release-commit-message="chore: release v{{version}}" \
		--tag-message="v{{version}}" \
		--tag-prefix="v"

.PHONY: major
major: ## Bump major version: 0.1.0 → 1.0.0, tag, update changelog (no push, no publish)
	cargo release major \
		--workspace \
		--no-publish \
		--no-push \
		--no-verify \
		--no-confirm \
		--pre-release-commit-message="chore: release v{{version}}" \
		--tag-message="v{{version}}" \
		--tag-prefix="v"

.PHONY: release
release: ## Run interactive release
	cargo release --workspace

.PHONY: release-ci
release-ci: ## CI-friendly release (no confirmation)
	cargo release --workspace --no-confirm

# -----------------------------------------------------------------------------
# Convenience Targets
# -----------------------------------------------------------------------------

.PHONY: all
all: check clippy fmt test doc ## Check, clippy, fmt, test, and generate documentation

.PHONY: ci
ci: fmt check clippy test ## Run a full CI-like pipeline locally
	@echo "All checks passed successfully."