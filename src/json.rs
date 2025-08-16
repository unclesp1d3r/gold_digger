use std::{collections::HashMap, io::Write};

use serde_json::json;

/// Writes rows to a JSON output using the provided writer.
///
/// # Arguments
///
/// * `rows` - A vector of string vectors representing the data.
/// * `output` - A writer to output the JSON data.
///
/// # Returns
///
/// A Result indicating success or failure.
pub fn write<W>(rows: Vec<Vec<String>>, mut output: W) -> anyhow::Result<()>
where
    W: Write,
{
    let headers = match rows.first() {
        Some(header_row) => header_row.to_owned(),
        None => anyhow::bail!("No header row found"),
    };
    let mut results: Vec<HashMap<String, String>> = Vec::new();

    for row in rows.into_iter().skip(1) {
        let item: HashMap<String, String> =
            headers.clone().into_iter().zip(row.into_iter()).collect();
        results.push(item);
    }

    let result: String = json!({ "data": results }).to_string();
    output.write_all(result.as_bytes())?;
    Ok(())
}
