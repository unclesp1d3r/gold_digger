fn main() {
    // Build script for gold_digger
    //
    // This build script handles any compile-time configuration needed for Gold Digger.
    // The project uses a single rustls-based TLS implementation via the ssl feature.

    // Re-run if features change
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SSL");
}
