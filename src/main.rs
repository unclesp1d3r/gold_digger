use std::{env, fs::File, path::PathBuf};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::{Shell as CompletionShell, generate};
use mysql::Pool;
use mysql::prelude::Queryable;

use gold_digger::cli::{Cli, Commands, OutputFormat, Shell};
use gold_digger::exit::{exit_no_rows, exit_success, exit_with_error};
use gold_digger::rows_to_strings;

#[cfg(feature = "ssl")]
use gold_digger::tls::{TlsConfig, create_tls_connection};

/// Main entry point for the gold_digger CLI tool.
///
/// Parses CLI arguments and environment variables, executes a database query, and writes the output in the specified format.
fn main() {
    let cli = Cli::parse();

    // Handle subcommands first
    if let Some(command) = cli.command {
        match command {
            Commands::Completion { shell } => {
                generate_completion(shell);
                return;
            },
        }
    }

    // Handle --dump-config flag
    if cli.dump_config {
        if let Err(e) = dump_configuration(&cli) {
            exit_with_error(e, Some("Configuration dump failed"));
        }
        return;
    }

    // Resolve configuration with precedence: CLI flags > environment variables
    let database_url = match resolve_database_url(&cli) {
        Ok(url) => url,
        Err(e) => exit_with_error(e, Some("Database URL resolution failed")),
    };
    let database_query = match resolve_database_query(&cli) {
        Ok(query) => query,
        Err(e) => exit_with_error(e, Some("Database query resolution failed")),
    };
    let output_file = match resolve_output_file(&cli) {
        Ok(file) => file,
        Err(e) => exit_with_error(e, Some("Output file resolution failed")),
    };

    let pool = match create_database_connection(&database_url) {
        Ok(pool) => pool,
        Err(e) => exit_with_error(anyhow::anyhow!("Database connection pool creation failed: {}", e), None),
    };
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => exit_with_error(anyhow::anyhow!("Database connection failed: {}", e), None),
    };

    if cli.verbose > 0 && !cli.quiet {
        println!("Connecting to database...");
    }

    let result: Vec<mysql::Row> = match conn.query(database_query) {
        Ok(result) => result,
        Err(e) => exit_with_error(anyhow::anyhow!("Query execution failed: {}", e), None),
    };

    if cli.verbose > 0 && !cli.quiet {
        println!("Outputting {} records to {}.", result.len(), output_file.display());
    }

    if result.is_empty() {
        if cli.allow_empty {
            if cli.verbose > 0 && !cli.quiet {
                println!("No records found in database, but --allow-empty is set.");
            }
            // Create empty output file
            let output = match File::create(&output_file) {
                Ok(output) => output,
                Err(e) => exit_with_error(anyhow::anyhow!("Failed to create output file: {}", e), None),
            };
            let empty_rows: Vec<Vec<String>> = vec![];
            if let Err(e) = write_output(empty_rows, output, output_file.as_path(), &cli) {
                exit_with_error(e, Some("Output writing failed"));
            }
        } else {
            if cli.verbose > 0 && !cli.quiet {
                println!("No records found in database.");
            }
            exit_no_rows(Some("No records found in database"));
        }
    } else {
        let rows = match rows_to_strings(result) {
            Ok(rows) => rows,
            Err(e) => exit_with_error(e, Some("Row conversion failed")),
        };
        let output = match File::create(&output_file) {
            Ok(output) => output,
            Err(e) => exit_with_error(anyhow::anyhow!("Failed to create output file: {}", e), None),
        };
        if let Err(e) = write_output(rows, output, output_file.as_path(), &cli) {
            exit_with_error(e, Some("Output writing failed"));
        }
    }

    exit_success(None);
}

/// Creates a database connection pool with optional TLS configuration
fn create_database_connection(database_url: &str) -> Result<Pool> {
    #[cfg(feature = "ssl")]
    {
        // Parse TLS configuration from URL (placeholder for future enhancement)
        let tls_config = parse_tls_config_from_url(database_url)?;

        // Use TLS-aware connection creation
        create_tls_connection(database_url, tls_config)
    }

    #[cfg(not(feature = "ssl"))]
    {
        // Fallback to direct Pool creation when SSL feature is disabled
        Pool::new(database_url).map_err(|e| anyhow::anyhow!("Database connection failed: {}", e))
    }
}

/// Parses TLS configuration from database URL
/// Currently returns None as the mysql crate doesn't support URL-based SSL configuration
/// This function provides a foundation for future TLS URL parameter support
#[cfg(feature = "ssl")]
fn parse_tls_config_from_url(_database_url: &str) -> Result<Option<TlsConfig>> {
    // The mysql crate doesn't support URL-based SSL configuration like ssl-mode, ssl-ca, etc.
    // For now, we return None to use default TLS behavior when the ssl feature is enabled
    // Future enhancement: Parse URL parameters and create appropriate TlsConfig

    // Example of what this could look like in the future:
    // if database_url.contains("ssl-mode=required") {
    //     return Ok(Some(TlsConfig::new()));
    // }
    // if database_url.contains("ssl-ca=") {
    //     // Extract CA path and create config
    // }

    Ok(None)
}

/// Resolves the database URL from CLI arguments or environment variables
fn resolve_database_url(cli: &Cli) -> Result<String> {
    if let Some(url) = &cli.db_url {
        Ok(url.clone())
    } else if let Ok(url) = env::var("DATABASE_URL") {
        Ok(url)
    } else {
        anyhow::bail!("Missing database URL. Provide --db-url or set DATABASE_URL environment variable");
    }
}

/// Resolves the database query from CLI arguments or environment variables
fn resolve_database_query(cli: &Cli) -> Result<String> {
    if let Some(query) = &cli.query {
        Ok(query.clone())
    } else if let Some(query_file) = &cli.query_file {
        std::fs::read_to_string(query_file)
            .map_err(|e| anyhow::anyhow!("Failed to read query file {}: {}", query_file.display(), e))
    } else if let Ok(query) = env::var("DATABASE_QUERY") {
        Ok(query)
    } else {
        anyhow::bail!(
            "Missing database query. Provide --query, --query-file, or set DATABASE_QUERY environment variable"
        );
    }
}

/// Resolves the output file path from CLI arguments or environment variables
fn resolve_output_file(cli: &Cli) -> Result<PathBuf> {
    if let Some(output) = &cli.output {
        Ok(output.clone())
    } else if let Ok(output) = env::var("OUTPUT_FILE") {
        Ok(PathBuf::from(output))
    } else {
        anyhow::bail!("Missing output file. Provide --output or set OUTPUT_FILE environment variable");
    }
}

/// Writes output in the specified format
fn write_output(rows: Vec<Vec<String>>, output: File, output_file: &std::path::Path, cli: &Cli) -> Result<()> {
    let format = if let Some(format) = &cli.format {
        format.clone()
    } else {
        OutputFormat::from_extension(output_file)
    };

    match format {
        #[cfg(feature = "csv")]
        OutputFormat::Csv => gold_digger::csv::write(rows, output)?,
        #[cfg(feature = "json")]
        OutputFormat::Json => gold_digger::json::write_with_pretty(rows, output, cli.pretty)?,
        OutputFormat::Tsv => gold_digger::tab::write(rows, output)?,
        #[cfg(not(feature = "csv"))]
        OutputFormat::Csv => anyhow::bail!("CSV support not compiled in"),
        #[cfg(not(feature = "json"))]
        OutputFormat::Json => anyhow::bail!("JSON support not compiled in"),
    }

    Ok(())
}

/// Generates shell completion scripts
fn generate_completion(shell: Shell) {
    let mut cmd = Cli::command();
    let bin_name = "gold_digger";

    match shell {
        Shell::Bash => generate(CompletionShell::Bash, &mut cmd, bin_name, &mut std::io::stdout()),
        Shell::Zsh => generate(CompletionShell::Zsh, &mut cmd, bin_name, &mut std::io::stdout()),
        Shell::Fish => generate(CompletionShell::Fish, &mut cmd, bin_name, &mut std::io::stdout()),
        Shell::PowerShell => generate(CompletionShell::PowerShell, &mut cmd, bin_name, &mut std::io::stdout()),
    }
}

/// Dumps current configuration as JSON
fn dump_configuration(cli: &Cli) -> Result<()> {
    use serde_json::json;

    let config = json!({
        "database_url": if cli.db_url.is_some() {
            "***REDACTED***".to_string()
        } else {
            env::var("DATABASE_URL").map(|_| "***REDACTED***".to_string()).unwrap_or("null".to_string())
        },
        "query": cli.query.as_ref().unwrap_or(&env::var("DATABASE_QUERY").unwrap_or_default()),
        "query_file": cli.query_file.as_ref().map(|p| p.display().to_string()),
        "output": cli.output.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| env::var("OUTPUT_FILE").unwrap_or_default()),
        "format": cli.format.as_ref().map(|f| f.as_str()),
        "verbose": cli.verbose,
        "quiet": cli.quiet,
        "pretty": cli.pretty,
        "allow_empty": cli.allow_empty
    });

    println!("{}", serde_json::to_string_pretty(&config)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_database_connection_invalid_url() {
        // Test with invalid URL to ensure error handling works
        let result = create_database_connection("invalid://url");
        assert!(result.is_err());
    }

    #[cfg(feature = "ssl")]
    #[test]
    fn test_parse_tls_config_from_url() {
        // Test the TLS config parsing function
        let result = parse_tls_config_from_url("mysql://user:pass@localhost:3306/db");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Currently returns None as documented
    }

    #[cfg(feature = "ssl")]
    #[test]
    fn test_create_database_connection_with_ssl_feature() {
        // Test that the function exists and handles errors properly when ssl feature is enabled
        let result = create_database_connection("mysql://invalid:invalid@nonexistent:3306/test");
        // Should fail due to invalid connection details, but not panic
        assert!(result.is_err());
    }

    #[cfg(not(feature = "ssl"))]
    #[test]
    fn test_create_database_connection_without_ssl_feature() {
        // Test that the function works without ssl feature
        let result = create_database_connection("mysql://invalid:invalid@nonexistent:3306/test");
        // Should fail due to invalid connection details, but not panic
        assert!(result.is_err());
    }
}
