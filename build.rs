use std::env;

fn main() {
    // Check for enabled features using Cargo-provided environment variables
    let ssl_enabled = env::var_os("CARGO_FEATURE_SSL").is_some();
    let ssl_rustls_enabled = env::var_os("CARGO_FEATURE_SSL_RUSTLS").is_some();

    // Detect musl target using CARGO_CFG_TARGET_ENV
    let is_musl = env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("musl");

    // Get the target triple for error messages
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    // Check if this is a musl target
    if is_musl {
        // For musl targets, we need to ensure ssl-rustls is used instead of native-tls
        // This prevents OpenSSL dependency issues on musl systems
        println!("cargo:warning=Detected musl target: {}. Forcing ssl-rustls feature for compatibility.", target);
        println!("cargo:rustc-cfg=target_musl");

        // Set a custom cfg flag that can be used in the code
        println!("cargo:rustc-cfg=musl_target");

        // If native-tls is being used, this will cause a build error
        // The build will fail with a clear error message
        if ssl_enabled && !ssl_rustls_enabled {
            panic!(
                "ERROR: musl target '{}' detected but native-tls (ssl feature) is enabled.\n\
                musl targets require ssl-rustls feature for compatibility.\n\
                Please use --no-default-features --features ssl-rustls instead.\n\
                \n\
                Recommended build command:\n\
                cargo build --release --no-default-features --features \"json,csv,ssl-rustls,additional_mysql_types,verbose\"",
                target
            );
        }
    }

    // Validate feature combinations
    if ssl_enabled && ssl_rustls_enabled {
        panic!(
            "ERROR: Both 'ssl' and 'ssl-rustls' features are enabled.\n\
            These features are mutually exclusive. Please enable only one:\n\
            - For native TLS: --features ssl\n\
            - For pure Rust TLS: --features ssl-rustls"
        );
    }

    // Re-run if target changes
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ENV");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SSL");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SSL_RUSTLS");
}
