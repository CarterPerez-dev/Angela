# Angelax Build System

This document describes the optimized build configuration for the Angelax web framework.

## Overview

Angelax uses an aggressively optimized build system designed for maximum performance. Our configuration includes:

- **CPU-specific optimizations**: Native instruction sets (AVX2, SSE4.2, etc.)
- **Link-time optimization (LTO)**: Fat LTO for release builds
- **Custom allocator support**: Build-time configuration for our custom allocator
- **Profile-guided optimization**: Support for PGO builds
- **Platform-specific tuning**: Optimized for Linux, macOS, and Windows

## Build Profiles

### Development (`cargo build`)
- Fast compilation
- Debug symbols enabled
- Incremental compilation
- Thin LTO for better performance even in dev

### Release (`cargo build --release`)
- Full optimizations (opt-level = 3)
- Fat LTO enabled
- Single codegen unit
- Strip symbols
- Panic = abort

### Production (`cargo build --profile production`)
- Everything from release profile
- Native CPU targeting
- Additional LLVM optimizations
- Designed for deployment on known hardware

### Min-Size (`cargo build --profile min-size`)
- Optimized for binary size (opt-level = z)
- Ideal for edge deployments
- Full LTO and stripping

## Quick Start

```bash
# Setup development environment
make setup

# Development build
make build

# Production build with native optimizations
make production

# Run tests
make test

# Run benchmarks
make bench

# See all available commands
make help
```

## Advanced Optimizations

### Profile-Guided Optimization (PGO)

```bash
# Step 1: Build with profiling
make pgo-generate

# Step 2: Run your workload to generate profile data
./target/release/angelax-cli benchmark

# Step 3: Build with profile data
make pgo-build
```

### CPU Feature Detection

The build system automatically detects and enables:
- AVX2/AVX512 instructions
- SSE4.2 for string operations
- Cache line size for optimal alignment
- Huge pages support on Linux

### Linker Optimizations

We automatically use the fastest available linker:
1. **mold** (Linux) - Fastest option
2. **lld** (cross-platform) - Fast alternative
3. System default - Fallback option

## Platform-Specific Notes

### Linux
- Huge pages support enabled when available
- Security hardening flags enabled
- Uses mold linker for fastest builds

### macOS
- Universal binary support (x86_64 and ARM64)
- Dead code stripping enabled
- Optimized for both Intel and Apple Silicon

### Windows
- MSVC optimizations enabled
- Incremental linking disabled in release
- Profile-guided optimization support

## Benchmarking

We maintain comprehensive benchmarks in `benchmarks/`:

```bash
# Run all benchmarks
cargo bench --profile bench

# Run specific benchmark suite
cargo bench --bench http_parsing --profile bench

# Compare against other frameworks
make perf-compare
```

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Ensure you have the latest Rust stable: `rustup update`
2. Clean the build: `cargo clean`
3. Check your RUSTFLAGS: `echo $RUSTFLAGS`

### Performance Issues

If builds are slow:

1. Enable parallel compilation: `export CARGO_BUILD_JOBS=0`
2. Use a faster linker: Install mold or lld
3. Disable LTO for development: Comment out LTO in Cargo.toml

### Memory Issues

For systems with limited memory:

1. Reduce codegen units: Set `codegen-units = 16` in release profile
2. Disable parallel compilation: `export CARGO_BUILD_JOBS=2`
3. Use thin LTO instead of fat LTO

## Contributing

When adding new crates or build configurations:

1. Update the workspace members in root `Cargo.toml`
2. Ensure new crates follow the optimization settings
3. Add appropriate build.rs scripts for compile-time optimizations
4. Update CI configuration if needed

## Build Metrics

Typical build performance on modern hardware:

- **Dev build**: < 30 seconds (incremental: < 5 seconds)
- **Release build**: 2-3 minutes
- **Production build**: 3-5 minutes
- **Binary size**: ~5MB (stripped, min-size profile)

## Security

Our build configuration includes:

- Position Independent Executables (PIE)
- Stack protection
- FORTIFY_SOURCE (where applicable)
- Control Flow Guard (Windows)
