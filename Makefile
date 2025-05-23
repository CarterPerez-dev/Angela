# This Makefile provides convenient commands for building Angelax with optimal settings.
# It wraps cargo commands with the right flags and environment variables for maximum performance.

.PHONY: all build release production bench test clean doc fmt lint install help

# I'm detecting the number of CPU cores for parallel builds
NPROCS := $(shell nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
MAKEFLAGS += -j$(NPROCS)

# Setting up color output for better readability
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
NC := \033[0m # No Color

# Default target
all: build

help: ## Show this help message
	@echo "$(BLUE)Angelax Build System$(NC)"
	@echo "===================="
	@echo ""
	@echo "$(YELLOW)Available targets:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'

# Development build
build: ## Build in development mode (fast compilation)
	@echo "$(BLUE)Building Angelax (dev mode)...$(NC)"
	CARGO_BUILD_JOBS=$(NPROCS) cargo build

# Release build with optimizations
release: ## Build in release mode (optimized)
	@echo "$(BLUE)Building Angelax (release mode)...$(NC)"
	CARGO_BUILD_JOBS=$(NPROCS) cargo build --release

# Production build with maximum optimizations
production: ## Build with production profile (maximum optimization)
	@echo "$(BLUE)Building Angelax (production mode)...$(NC)"
	RUSTFLAGS="-C target-cpu=native" CARGO_BUILD_JOBS=$(NPROCS) cargo build --profile production

# Size-optimized build for edge deployments
minsize: ## Build with minimum size profile
	@echo "$(BLUE)Building Angelax (minimum size)...$(NC)"
	CARGO_BUILD_JOBS=$(NPROCS) cargo build --profile min-size

# Run benchmarks
bench: ## Run performance benchmarks
	@echo "$(BLUE)Running benchmarks...$(NC)"
	cargo bench --profile bench

# Run tests
test: ## Run all tests
	@echo "$(BLUE)Running tests...$(NC)"
	RUST_BACKTRACE=1 cargo test --all

# Run tests in release mode
test-release: ## Run tests with optimizations
	@echo "$(BLUE)Running tests (release mode)...$(NC)"
	cargo test --release --all

# Clean build artifacts
clean: ## Clean all build artifacts
	@echo "$(RED)Cleaning build artifacts...$(NC)"
	cargo clean
	rm -rf target/

# Generate documentation
doc: ## Generate and open documentation
	@echo "$(BLUE)Generating documentation...$(NC)"
	cargo doc --all-features --no-deps --open

# Format code
fmt: ## Format all code
	@echo "$(BLUE)Formatting code...$(NC)"
	cargo fmt --all

# Check formatting
fmt-check: ## Check code formatting
	@echo "$(BLUE)Checking code format...$(NC)"
	cargo fmt --all -- --check

# Run linting
lint: ## Run clippy lints
	@echo "$(BLUE)Running lints...$(NC)"
	cargo clippy --all-targets --all-features -- -D warnings

# Fix linting issues
lint-fix: ## Fix linting issues automatically
	@echo "$(BLUE)Fixing lints...$(NC)"
	cargo clippy --all-targets --all-features --fix -- -D warnings

# Install locally
install: ## Install angelax-cli locally
	@echo "$(BLUE)Installing angelax-cli...$(NC)"
	cargo install --path crates/angelax-cli --profile production

# Check for security vulnerabilities
audit: ## Run security audit
	@echo "$(BLUE)Running security audit...$(NC)"
	cargo audit

# Update dependencies
update: ## Update all dependencies
	@echo "$(BLUE)Updating dependencies...$(NC)"
	cargo update

# Profile-guided optimization build (step 1: generate profile data)
pgo-generate: ## Build with PGO profiling generation
	@echo "$(BLUE)Building with PGO generation...$(NC)"
	RUSTFLAGS="-C profile-generate=/tmp/pgo-data" cargo build --profile production

# Profile-guided optimization build (step 2: use profile data)
pgo-build: ## Build using PGO profile data
	@echo "$(BLUE)Merging PGO data...$(NC)"
	llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
	@echo "$(BLUE)Building with PGO optimization...$(NC)"
	RUSTFLAGS="-C profile-use=/tmp/pgo-data/merged.profdata" cargo build --profile production

# Build with link-time optimization analysis
lto-analysis: ## Analyze LTO impact
	@echo "$(BLUE)Building with LTO analysis...$(NC)"
	RUSTFLAGS="-C lto=fat -C linker-plugin-lto -Z print-link-args" cargo +nightly build --release

# Memory usage analysis
mem-analysis: ## Analyze memory usage
	@echo "$(BLUE)Analyzing memory usage...$(NC)"
	cargo +nightly build -Z print-type-sizes --release

# Build for all targets
build-all-targets: ## Build for all supported targets
	@echo "$(BLUE)Building for all targets...$(NC)"
	@for target in x86_64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-msvc; do \
		echo "$(GREEN)Building for $$target...$(NC)"; \
		cargo build --target $$target --release; \
	done

# Quick development cycle
dev: ## Quick build and run
	@echo "$(BLUE)Quick dev build...$(NC)"
	cargo build --package angelax-cli
	./target/debug/angelax-cli

# CI/CD simulation
ci: fmt-check lint test ## Run CI checks locally
	@echo "$(GREEN)All CI checks passed!$(NC)"

# Performance comparison
perf-compare: ## Compare performance across different build profiles
	@echo "$(BLUE)Building all profiles for comparison...$(NC)"
	@cargo build
	@cargo build --release  
	@cargo build --profile production
	@echo "$(GREEN)Build complete. Check target/ for binaries.$(NC)"

# Setup development environment
setup: ## Setup development environment
	@echo "$(BLUE)Setting up development environment...$(NC)"
	@rustup component add rustfmt clippy rust-src rust-analyzer
	@cargo install cargo-audit cargo-outdated cargo-tree cargo-expand
	@echo "$(GREEN)Development environment ready!$(NC)"

# Show build configuration
show-config: ## Display current build configuration
	@echo "$(BLUE)Current Build Configuration:$(NC)"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Target: $$(rustc -vV | grep host | cut -d' ' -f2)"
	@echo "CPU cores: $(NPROCS)"
	@echo "RUSTFLAGS: $${RUSTFLAGS:-none}"
