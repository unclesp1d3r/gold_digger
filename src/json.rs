use std::{collections::BTreeMap, io::Write};

use crate::FormatWriter;
use anyhow::Result;

/// JSON writer that implements the FormatWriter trait
pub struct JsonWriter<W: Write> {
    writer: W,
    columns: Vec<String>,
    first_row: bool,
    pretty: bool,
}

impl<W: Write> JsonWriter<W> {
    /// Creates a new JsonWriter with the specified writer and pretty printing option
    pub fn new(writer: W, pretty: bool) -> Self {
        Self {
            writer,
            columns: Vec::new(),
            first_row: true,
            pretty,
        }
    }
}

impl<W: Write> FormatWriter for JsonWriter<W> {
    fn write_header(&mut self, columns: &[String]) -> Result<()> {
        self.columns = columns.to_vec();
        self.first_row = true;
        write!(self.writer, "{{\"data\":[")?;
        Ok(())
    }

    fn write_row(&mut self, row: &[String]) -> Result<()> {
        if !self.first_row {
            write!(self.writer, ",")?;
        }
        self.first_row = false;

        // Create ordered map for deterministic output
        let mut obj = BTreeMap::new();
        for (col, val) in self.columns.iter().zip(row.iter()) {
            // Convert string values to appropriate JSON types when possible
            let json_value = if val.is_empty() {
                serde_json::Value::Null
            } else if let Ok(num) = val.parse::<i64>() {
                serde_json::Value::Number(num.into())
            } else if let Ok(num) = val.parse::<f64>() {
                serde_json::Number::from_f64(num)
                    .map(serde_json::Value::Number)
                    .unwrap_or_else(|| serde_json::Value::String(val.clone()))
            } else if val == "true" || val == "false" {
                serde_json::Value::Bool(val == "true")
            } else {
                serde_json::Value::String(val.clone())
            };
            obj.insert(col.clone(), json_value);
        }

        if self.pretty {
            serde_json::to_writer_pretty(&mut self.writer, &obj)?;
        } else {
            serde_json::to_writer(&mut self.writer, &obj)?;
        }

        Ok(())
    }

    fn finalize(mut self) -> Result<()> {
        write!(self.writer, "]}}")?;
        self.writer.flush()?;
        Ok(())
    }
}

/// Writes rows to a JSON output using the provided writer.
///
/// # Arguments
///
/// * `rows` - An iterator over records, where each record is an iterator over fields.
/// * `output` - A writer to output the JSON data.
///
/// # Returns
///
/// A Result indicating success or failure.
pub fn write<R, F, W>(rows: R, output: W) -> anyhow::Result<()>
where
    R: IntoIterator<Item = F>,
    F: IntoIterator<Item = String>,
    W: Write,
{
    write_with_pretty(rows, output, false)
}

/// Writes rows to a JSON output using the provided writer with pretty printing option.
///
/// # Arguments
///
/// * `rows` - An iterator over records, where each record is an iterator over fields.
/// * `output` - A writer to output the JSON data.
/// * `pretty` - Whether to format the JSON with pretty printing.
///
/// # Returns
///
/// A Result indicating success or failure.
pub fn write_with_pretty<R, F, W>(rows: R, output: W, pretty: bool) -> anyhow::Result<()>
where
    R: IntoIterator<Item = F>,
    F: IntoIterator<Item = String>,
    W: Write,
{
    let mut rows_iter = rows.into_iter();

    // Get the first row as headers, or use empty if no rows
    let headers = if let Some(first_row) = rows_iter.next() {
        first_row.into_iter().collect::<Vec<String>>()
    } else {
        let mut writer = JsonWriter::new(output, pretty);
        writer.write_header(&[])?;
        writer.finalize()?;
        return Ok(());
    };

    let mut writer = JsonWriter::new(output, pretty);
    writer.write_header(&headers)?;

    for row in rows_iter {
        let row_vec: Vec<String> = row.into_iter().collect();
        writer.write_row(&row_vec)?;
    }

    writer.finalize()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_json_writer_format() {
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = JsonWriter::new(&mut cursor, false);

        // Test header
        writer.write_header(&["col1".to_string(), "col2".to_string()]).unwrap();

        // Test first row
        writer.write_row(&["val1".to_string(), "val2".to_string()]).unwrap();

        // Test second row
        writer.write_row(&["val3".to_string(), "val4".to_string()]).unwrap();

        // Test finalize
        writer.finalize().unwrap();

        let output = String::from_utf8(cursor.into_inner()).unwrap();
        let expected = r#"{"data":[{"col1":"val1","col2":"val2"},{"col1":"val3","col2":"val4"}]}"#;

        assert_eq!(output, expected);
    }
}
