use std::{env, ffi::OsStr, path::Path};

use anyhow::{Context, Result};
use mysql::Row;

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
/// This function safely handles all MySQL data types including NULL values without panicking.
/// It uses safe iteration over row values instead of indexed access to prevent runtime panics.
///
/// # Arguments
///
/// * `rows` - A vector of MySQL rows.
///
/// # Returns
///
/// A Result containing a vector of string vectors, or an error.
///
/// # Safety
///
/// This function replaces the dangerous pattern of using `row[column.name_str().as_ref()]`
/// which can panic on NULL values or type mismatches. Instead, it uses safe iteration
/// over `row.as_ref()` to handle all value types gracefully.
pub fn rows_to_strings(rows: Vec<Row>) -> anyhow::Result<Vec<Vec<String>>> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut result_rows: Vec<Vec<String>> = Vec::new();

    // Extract headers from the first row
    let header_row: Vec<String> = rows[0]
        .columns_ref()
        .iter()
        .map(|column| column.name_str().to_string())
        .collect();
    result_rows.push(header_row);

    // Process each row using safe iteration
    for row in rows {
        let data_row: Vec<String> = (0..row.len())
            .map(|i| match row.as_ref(i) {
                Some(value) => mysql_value_to_string(value),
                None => String::new(),
            })
            .collect();
        result_rows.push(data_row);
    }

    Ok(result_rows)
}

/// Safely converts a MySQL Value to a String representation.
///
/// This function handles all MySQL value types including NULL values,
/// binary data, and numeric types without panicking.
///
/// # Arguments
///
/// * `value` - A reference to a MySQL Value.
///
/// # Returns
///
/// A String representation of the value. NULL values become empty strings.
fn mysql_value_to_string(value: &mysql::Value) -> String {
    match value {
        mysql::Value::NULL => String::new(),
        mysql::Value::Bytes(bytes) => {
            // Try to convert bytes to UTF-8 string, fallback to debug representation
            String::from_utf8(bytes.clone()).unwrap_or_else(|_| format!("{:?}", bytes))
        },
        mysql::Value::Int(i) => i.to_string(),
        mysql::Value::UInt(u) => u.to_string(),
        mysql::Value::Float(f) => f.to_string(),
        mysql::Value::Double(d) => d.to_string(),
        mysql::Value::Date(year, month, day, hour, minute, second, microsecond) => {
            if *hour == 0 && *minute == 0 && *second == 0 && *microsecond == 0 {
                format!("{:04}-{:02}-{:02}", year, month, day)
            } else {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                    year, month, day, hour, minute, second, microsecond
                )
            }
        },
        mysql::Value::Time(negative, days, hours, minutes, seconds, microseconds) => {
            let sign = if *negative { "-" } else { "" };
            if *days > 0 {
                format!("{}{:02}:{:02}:{:02}.{:06}", sign, days * 24 + *hours as u32, minutes, seconds, microseconds)
            } else {
                format!("{}{:02}:{:02}:{:02}.{:06}", sign, hours, minutes, seconds, microseconds)
            }
        },
    }
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

    #[test]
    fn test_mysql_value_to_string_null() {
        let result = mysql_value_to_string(&mysql::Value::NULL);
        assert_eq!(result, "");
    }

    #[test]
    fn test_mysql_value_to_string_integers() {
        assert_eq!(mysql_value_to_string(&mysql::Value::Int(42)), "42");
        assert_eq!(mysql_value_to_string(&mysql::Value::Int(-42)), "-42");
        assert_eq!(mysql_value_to_string(&mysql::Value::UInt(123)), "123");
    }

    #[test]
    fn test_mysql_value_to_string_floats() {
        assert_eq!(mysql_value_to_string(&mysql::Value::Float(3.5)), "3.5");
        assert_eq!(mysql_value_to_string(&mysql::Value::Double(2.5)), "2.5");
    }

    #[test]
    fn test_mysql_value_to_string_bytes() {
        let bytes = b"hello world".to_vec();
        let result = mysql_value_to_string(&mysql::Value::Bytes(bytes));
        assert_eq!(result, "hello world");

        // Test invalid UTF-8 bytes
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        let result = mysql_value_to_string(&mysql::Value::Bytes(invalid_bytes.clone()));
        assert!(result.contains("255")); // Should contain debug representation
    }

    #[test]
    fn test_mysql_value_to_string_date() {
        let result = mysql_value_to_string(&mysql::Value::Date(2023, 12, 25, 0, 0, 0, 0));
        assert_eq!(result, "2023-12-25");

        let result = mysql_value_to_string(&mysql::Value::Date(2023, 12, 25, 14, 30, 45, 123456));
        assert_eq!(result, "2023-12-25 14:30:45.123456");
    }

    #[test]
    fn test_mysql_value_to_string_time() {
        let result = mysql_value_to_string(&mysql::Value::Time(false, 0, 14, 30, 45, 123456));
        assert_eq!(result, "14:30:45.123456");

        let result = mysql_value_to_string(&mysql::Value::Time(true, 1, 2, 30, 45, 0));
        assert_eq!(result, "-26:30:45.000000");
    }

    #[test]
    fn test_rows_to_strings_empty() {
        let result = rows_to_strings(vec![]).unwrap();
        assert_eq!(result.len(), 0);
    }
}
