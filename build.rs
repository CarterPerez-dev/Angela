// This build script performs compile-time CPU feature detection and optimization setup.
// It configures the build environment for maximum performance based on the target platform.

use std::env;
use std::process::Command;

fn main() {
    // I'm detecting the target architecture for platform-specific optimizations
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let profile = env::var("PROFILE").unwrap();
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    
    // Setting up CPU feature detection for x86_64
    if target.contains("x86_64") {
        detect_x86_features();
    }
    
    // Configuring link-time optimizations based on profile
    if profile == "release" || profile == "production" {
        configure_lto();
    }
    
    // I'm enabling platform-specific optimizations
    match target.as_str() {
        "x86_64-unknown-linux-gnu" => configure_linux_optimizations(),
        "x86_64-apple-darwin" | "aarch64-apple-darwin" => configure_macos_optimizations(),
        "x86_64-pc-windows-msvc" => configure_windows_optimizations(),
        _ => {}
    }
    
    // Detecting available SIMD capabilities
    detect_simd_support();
    
    // Setting up memory allocator hints
    configure_allocator_hints();
    
    // If we're building for the same architecture, enable native CPU optimizations
    if host == target && env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() != "musl" {
        println!("cargo:rustc-cfg=native_cpu");
    }
}

fn detect_x86_features() {
    // I'm using compile-time CPU feature detection to enable optimizations
    if is_x86_feature_available("avx2") {
        println!("cargo:rustc-cfg=avx2_enabled");
    }
    
    if is_x86_feature_available("avx512f") {
        println!("cargo:rustc-cfg=avx512_enabled");
    }
    
    if is_x86_feature_available("sse4.2") {
        println!("cargo:rustc-cfg=sse42_enabled");
    }
    
    // Detecting CPU cache line size for alignment optimizations
    let cache_line_size = detect_cache_line_size();
    println!("cargo:rustc-env=CACHE_LINE_SIZE={}", cache_line_size);
}

fn is_x86_feature_available(feature: &str) -> bool {
    // In a real implementation, I'd use CPUID or similar
    // For now, I'm checking if the feature is in RUSTFLAGS
    if let Ok(rustflags) = env::var("RUSTFLAGS") {
        rustflags.contains(&format!("+{}", feature))
    } else {
        false
    }
}

fn detect_cache_line_size() -> usize {
    // I'm detecting the CPU cache line size for optimal memory alignment
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("getconf")
            .arg("LEVEL1_DCACHE_LINESIZE")
            .output()
        {
            if let Ok(size) = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<usize>()
            {
                return size;
            }
        }
    }
    
    // Default to 64 bytes (common for x86_64)
    64
}

fn configure_lto() {
    // I'm setting up link-time optimization flags
    println!("cargo:rustc-link-arg=-Wl,--icf=all");
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    
    // Enable parallel LTO if available
    if let Ok(num_jobs) = env::var("CARGO_BUILD_JOBS") {
        println!("cargo:rustc-link-arg=-Wl,--thinlto-jobs={}", num_jobs);
    }
}

fn configure_linux_optimizations() {
    // Linux-specific optimizations
    println!("cargo:rustc-link-arg=-Wl,-z,now");
    println!("cargo:rustc-link-arg=-Wl,-z,relro");
    println!("cargo:rustc-link-arg=-Wl,--as-needed");
    
    // I'm checking if we can use mold linker for faster builds
    if is_mold_available() {
        println!("cargo:rustc-link-arg=-fuse-ld=mold");
    } else if is_lld_available() {
        println!("cargo:rustc-link-arg=-fuse-ld=lld");
    }
}

fn configure_macos_optimizations() {
    // macOS-specific optimizations
    println!("cargo:rustc-link-arg=-Wl,-dead_strip");
    println!("cargo:rustc-link-arg=-Wl,-no_compact_unwind");
}

fn configure_windows_optimizations() {
    // Windows-specific optimizations
    println!("cargo:rustc-link-arg=/OPT:REF");
    println!("cargo:rustc-link-arg=/OPT:ICF");
    println!("cargo:rustc-link-arg=/LTCG");
}

fn detect_simd_support() {
    // I'm enabling compile-time flags for SIMD optimization paths
    println!("cargo:rustc-cfg=simd_support");
    
    // Detect specific SIMD instruction sets
    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::x86_64::*;
        
        // This would need runtime detection in practice
        // For build time, we'll rely on target features
        if cfg!(target_feature = "avx2") {
            println!("cargo:rustc-cfg=has_avx2");
        }
        
        if cfg!(target_feature = "sse4.2") {
            println!("cargo:rustc-cfg=has_sse42");
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        println!("cargo:rustc-cfg=has_neon");
    }
}

fn configure_allocator_hints() {
    // I'm setting up hints for our custom allocator
    println!("cargo:rustc-env=ANGELAX_PAGE_SIZE=4096");
    println!("cargo:rustc-env=ANGELAX_ARENA_SIZE=1048576"); // 1MB arenas
    
    // Huge pages support on Linux
    #[cfg(target_os = "linux")]
    {
        if huge_pages_available() {
            println!("cargo:rustc-cfg=huge_pages_supported");
            println!("cargo:rustc-env=ANGELAX_USE_HUGE_PAGES=1");
        }
    }
}

fn is_mold_available() -> bool {
    Command::new("mold").arg("--version").output().is_ok()
}

fn is_lld_available() -> bool {
    Command::new("ld.lld").arg("--version").output().is_ok()
}

#[cfg(target_os = "linux")]
fn huge_pages_available() -> bool {
    std::path::Path::new("/sys/kernel/mm/transparent_hugepage/enabled").exists()
}

#[cfg(not(target_os = "linux"))]
fn huge_pages_available() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_line_detection() {
        let size = detect_cache_line_size();
        // Cache lines are typically 32, 64, or 128 bytes
        assert!(size == 32 || size == 64 || size == 128);
    }
    
    #[test]
    fn test_feature_detection() {
        // This would need more sophisticated testing in practice
        env::set_var("RUSTFLAGS", "-C target-feature=+avx2");
        assert!(is_x86_feature_available("avx2"));
        env::remove_var("RUSTFLAGS");
    }
}
