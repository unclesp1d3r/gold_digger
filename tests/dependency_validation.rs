use std::process::Command;

/// Test to verify that rustls is present when using ssl feature (rustls-only implementation)
#[test]
fn test_rustls_with_ssl_feature() {
    let output = Command::new("cargo")
        .args(["tree", "-f", "{p} {f}", "--no-default-features", "--features", "ssl"])
        .output()
        .expect("Failed to run cargo tree");

    let tree_output = String::from_utf8(output.stdout).unwrap();

    // Verify rustls is present (expected with ssl feature in rustls-only implementation)
    assert!(tree_output.contains("rustls"), "rustls dependency not found in tree with ssl feature: {}", tree_output);

    // Verify rustls-native-certs is present for platform certificate store integration
    assert!(
        tree_output.contains("rustls-native-certs"),
        "rustls-native-certs dependency not found in tree with ssl feature: {}",
        tree_output
    );

    // Verify native-tls is NOT present (rustls-only implementation)
    assert!(!tree_output.contains("native-tls"), "native-tls found in rustls-only implementation: {}", tree_output);
}

/// Test to verify that no TLS dependencies are present without ssl feature
#[test]
fn test_no_tls_without_ssl_feature() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "-f",
            "{p} {f}",
            "--no-default-features",
            "--features",
            "json,csv",
        ])
        .output()
        .expect("Failed to run cargo tree");

    let tree_output = String::from_utf8(output.stdout).unwrap();

    // Verify neither native-tls nor rustls is present without ssl feature
    assert!(!tree_output.contains("native-tls"), "native-tls dependency found without ssl feature: {}", tree_output);

    // Note: rustls might still be present through other dependencies (like testcontainers)
    // but it should not be directly included by our ssl feature
}

/// Test to verify correct feature flag behavior for ssl feature (rustls-only implementation)
#[test]
fn test_ssl_feature_flag_behavior() {
    // Test with ssl feature enabled (rustls-only implementation)
    let output_ssl = Command::new("cargo")
        .args(["tree", "-f", "{p} {f}", "--no-default-features", "--features", "ssl"])
        .output()
        .expect("Failed to run cargo tree with ssl feature");

    let tree_output_ssl = String::from_utf8(output_ssl.stdout).unwrap();

    // Should contain mysql with rustls (rustls-only implementation)
    assert!(
        tree_output_ssl.contains("mysql") && tree_output_ssl.contains("rustls"),
        "mysql with rustls not found with ssl feature: {}",
        tree_output_ssl
    );

    // Should contain rustls-native-certs for platform certificate store integration
    assert!(
        tree_output_ssl.contains("rustls-native-certs"),
        "rustls-native-certs not found with ssl feature: {}",
        tree_output_ssl
    );

    // Should NOT contain native-tls (rustls-only implementation)
    assert!(
        !tree_output_ssl.contains("native-tls"),
        "native-tls found in rustls-only implementation: {}",
        tree_output_ssl
    );
}

/// Test to verify no TLS dependencies when TLS features are disabled
#[test]
fn test_no_tls_dependencies_without_features() {
    let output = Command::new("cargo")
        .args([
            "tree",
            "-f",
            "{p} {f}",
            "--no-default-features",
            "--features",
            "json,csv",
            "--no-dev-deps", // Exclude dev dependencies to avoid testcontainers pulling in rustls
        ])
        .output()
        .expect("Failed to run cargo tree without TLS features");

    let tree_output = String::from_utf8(output.stdout).unwrap();

    // Verify no TLS-related dependencies are present in production dependencies
    assert!(!tree_output.contains("native-tls"), "native-tls dependency found without TLS features: {}", tree_output);

    assert!(!tree_output.contains("rustls"), "rustls dependency found without TLS features: {}", tree_output);
}

/// Helper function to parse cargo tree output and extract dependency names
fn parse_dependency_tree(tree_output: &str) -> Vec<String> {
    tree_output
        .lines()
        .filter_map(|line| {
            // Remove tree drawing characters and extract package name
            let cleaned = line.trim_start_matches(&['├', '│', '└', '─', ' '][..]);

            // Parse lines like "mysql v26.0.1" or "native-tls v0.2.11"
            if let Some(first_space) = cleaned.find(' ') {
                let dep_name = &cleaned[..first_space];
                if !dep_name.is_empty() {
                    Some(dep_name.to_string())
                } else {
                    None
                }
            } else if !cleaned.is_empty() {
                // Handle lines with just the package name
                Some(cleaned.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Test the dependency tree parsing logic
#[test]
fn test_dependency_tree_parsing() {
    let sample_output = r#"mysql v26.0.1
├── native-tls v0.2.11
│   ├── lazy_static v1.4.0
│   └── libc v0.2.147
└── serde v1.0.183"#;

    let dependencies = parse_dependency_tree(sample_output);

    println!("Parsed dependencies: {:?}", dependencies);

    assert!(dependencies.contains(&"mysql".to_string()));
    assert!(dependencies.contains(&"native-tls".to_string()));
    assert!(dependencies.contains(&"lazy_static".to_string()));
    assert!(dependencies.contains(&"libc".to_string()));
    assert!(dependencies.contains(&"serde".to_string()));
}

/// Test to verify feature combinations work correctly (rustls-only implementation)
#[test]
fn test_feature_combinations() {
    // Test ssl + json + csv (common combination)
    let output = Command::new("cargo")
        .args([
            "tree",
            "-f",
            "{p} {f}",
            "--no-default-features",
            "--features",
            "ssl,json,csv",
        ])
        .output()
        .expect("Failed to run cargo tree with ssl,json,csv features");

    let tree_output = String::from_utf8(output.stdout).unwrap();

    // Should have rustls (rustls-only implementation)
    assert!(tree_output.contains("rustls"), "rustls not found with ssl,json,csv features: {}", tree_output);

    // Should NOT have native-tls (rustls-only implementation)
    assert!(!tree_output.contains("native-tls"), "native-tls found in rustls-only implementation: {}", tree_output);

    // Should have serde_json and csv dependencies
    assert!(
        tree_output.contains("serde_json") || tree_output.contains("serde"),
        "JSON dependencies not found with json feature: {}",
        tree_output
    );

    assert!(tree_output.contains("csv"), "CSV dependency not found with csv feature: {}", tree_output);
}

/// Legacy test to verify cargo-deny is available for CI validation
#[test]
fn test_cargo_deny_available() {
    let output = std::process::Command::new("cargo").args(["deny", "--version"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("✓ cargo-deny is available: {}", version.trim());
        },
        _ => {
            // Don't panic in tests - just skip if cargo-deny isn't installed
            println!("⚠ cargo-deny not installed - install with: cargo install cargo-deny");
        },
    }
}
