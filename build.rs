fn main() {
    // Build script for gold_digger
    //
    // Note: Previously this file contained feature conflict checks for ssl vs ssl-rustls,
    // but we've migrated to a rustls-only implementation with a single ssl feature.

    // Re-run if features change
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_SSL");
}
