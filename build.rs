use std::env;

fn main() {
    // Check for enabled features using Cargo-provided environment variables
    let ssl_enabled = env::var_os("CARGO_FEATURE_SSL").is_some();
    let ssl_rustls_enabled = env::var_os("CARGO_FEATURE_SSL_RUSTLS").is_some();

    // Validate feature combinations
    if ssl_enabled && ssl_rustls_enabled {
        panic!(
            "ERROR: Both 'ssl' and 'ssl-rustls' features are enabled.\n\
            These features are mutually exclusive. Please enable only one:\n\
            - For native TLS: --features ssl\n\
            - For pure Rust TLS: --features ssl-rustls"
        );
    }

    // Re-run if features change
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SSL");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SSL_RUSTLS");
}
