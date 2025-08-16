use std::{env, fs::File, path::PathBuf};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::{Shell as CompletionShell, generate};
use mysql::Pool;
use mysql::prelude::Queryable;

use gold_digger::cli::{Cli, Commands, OutputFormat, Shell};
use gold_digger::rows_to_strings;

/// Main entry point for the gold_digger CLI tool.
///
/// Parses CLI arguments and environment variables, executes a database query, and writes the output in the specified format.
fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle subcommands first
    if let Some(command) = cli.command {
        match command {
            Commands::Completion { shell } => {
                generate_completion(shell);
                return Ok(());
            },
        }
    }

    // Handle --dump-config flag
    if cli.dump_config {
        dump_configuration(&cli)?;
        return Ok(());
    }

    // Resolve configuration with precedence: CLI flags > environment variables
    let database_url = resolve_database_url(&cli)?;
    let database_query = resolve_database_query(&cli)?;
    let output_file = resolve_output_file(&cli)?;

    let pool = Pool::new(database_url.as_str())?;
    let mut conn = pool.get_conn()?;

    if cli.verbose > 0 && !cli.quiet {
        println!("Connecting to database...");
    }

    let result: Vec<mysql::Row> = conn.query(database_query)?;

    if cli.verbose > 0 && !cli.quiet {
        println!("Outputting {} records to {}.", result.len(), output_file.display());
    }

    if result.is_empty() {
        if cli.allow_empty {
            if cli.verbose > 0 && !cli.quiet {
                println!("No records found in database, but --allow-empty is set.");
            }
            // Create empty output file
            let output = File::create(&output_file)?;
            let empty_rows: Vec<Vec<String>> = vec![];
            write_output(empty_rows, output, output_file.as_path(), &cli)?;
        } else {
            if cli.verbose > 0 && !cli.quiet {
                println!("No records found in database.");
            }
            anyhow::bail!("No records found in database.");
        }
    } else {
        let rows = rows_to_strings(result)?;
        let output = File::create(&output_file)?;
        write_output(rows, output, output_file.as_path(), &cli)?;
    }

    Ok(())
}

/// Resolves the database URL from CLI arguments or environment variables
fn resolve_database_url(cli: &Cli) -> Result<String> {
    if let Some(url) = &cli.db_url {
        Ok(url.clone())
    } else if let Ok(url) = env::var("DATABASE_URL") {
        Ok(url)
    } else {
        anyhow::bail!(
            "Missing database URL. Provide --db-url or set DATABASE_URL environment variable"
        );
    }
}

/// Resolves the database query from CLI arguments or environment variables
fn resolve_database_query(cli: &Cli) -> Result<String> {
    if let Some(query) = &cli.query {
        Ok(query.clone())
    } else if let Some(query_file) = &cli.query_file {
        std::fs::read_to_string(query_file).map_err(|e| {
            anyhow::anyhow!("Failed to read query file {}: {}", query_file.display(), e)
        })
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
        anyhow::bail!(
            "Missing output file. Provide --output or set OUTPUT_FILE environment variable"
        );
    }
}

/// Writes output in the specified format
fn write_output(
    rows: Vec<Vec<String>>,
    output: File,
    output_file: &std::path::Path,
    cli: &Cli,
) -> Result<()> {
    let format = if let Some(format) = &cli.format {
        format.clone()
    } else {
        OutputFormat::from_extension(output_file)
    };

    match format {
        #[cfg(feature = "csv")]
        OutputFormat::Csv => gold_digger::csv::write(rows, output)?,
        #[cfg(feature = "json")]
        OutputFormat::Json => {
            if cli.pretty {
                // TODO: Implement pretty JSON formatting in json module
                gold_digger::json::write(rows, output)?
            } else {
                gold_digger::json::write(rows, output)?
            }
        },
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
        Shell::PowerShell => {
            generate(CompletionShell::PowerShell, &mut cmd, bin_name, &mut std::io::stdout())
        },
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
