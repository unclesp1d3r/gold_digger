use std::{env, ffi::OsStr, path::Path};

use anyhow::{Context, Result};
use mysql::{Row, from_value_opt};

/// CLI interface module.
pub mod cli;
/// CSV output module.
pub mod csv;
/// Exit code helper module.
pub mod exit;
/// JSON output module.
pub mod json;
/// Tab-delimited output module.
pub mod tab;
/// TLS configuration module.
pub mod tls;

/// Trait for writing data in different formats
pub trait FormatWriter {
    fn write_header(&mut self, columns: &[String]) -> Result<()>;
    fn write_row(&mut self, row: &[String]) -> Result<()>;
    fn finalize(self) -> Result<()>;
}

// TODO: Implement RowStream with correct QueryResult type signature
// pub struct RowStream<'a> {
//     result: mysql::QueryResult<'a>,
//     columns: Vec<Column>,
// }

/// Converts MySQL rows to a vector of string vectors, with the first row as headers.
///
/// # Arguments
///
/// * `rows` - A vector of MySQL rows.
///
/// # Returns
///
/// A Result containing a vector of string vectors, or an error.
pub fn rows_to_strings(rows: Vec<Row>) -> anyhow::Result<Vec<Vec<String>>> {
    let mut result_rows: Vec<Vec<String>> = Vec::new();
    for row in rows.into_iter() {
        if result_rows.is_empty() {
            let header_row: Vec<String> = row
                .columns_ref()
                .to_vec()
                .iter()
                .map(|column| column.name_str().to_string())
                .collect::<Vec<String>>();
            result_rows.push(header_row);
        }

        let data_row: Vec<String> = row
            .columns_ref()
            .to_vec()
            .iter()
            .map(|column| {
                let val = &row[column.name_str().as_ref()];
                match val {
                    mysql::Value::NULL => "".to_string(),
                    val => from_value_opt::<String>(val.clone()).unwrap_or_else(|_| format!("{:?}", val)),
                }
            })
            .collect::<Vec<String>>();
        result_rows.push(data_row);
    }

    Ok(result_rows)
}

/// Extracts the file extension from a filename, if present.
///
/// # Arguments
///
/// * `filename` - The filename as a string slice.
///
/// # Returns
///
/// An Option containing the extension as a string slice, or None if not found.
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

/// Gets a required environment variable with contextual error information.
///
/// # Arguments
///
/// * `var_name` - The name of the environment variable to retrieve.
///
/// # Returns
///
/// A Result containing the environment variable value as a String, or an error with context.
pub fn get_required_env(var_name: &str) -> Result<String> {
    env::var(var_name).with_context(|| format!("Missing required environment variable: {}", var_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_required_env_missing() {
        let result = get_required_env("NONEXISTENT_ENV_VAR");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Missing required environment variable: NONEXISTENT_ENV_VAR")
        );
    }

    #[test]
    fn test_get_required_env_present() {
        unsafe {
            std::env::set_var("TEST_ENV_VAR", "test_value");
        }
        let result = get_required_env("TEST_ENV_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");
        unsafe {
            std::env::remove_var("TEST_ENV_VAR");
        }
    }
}
