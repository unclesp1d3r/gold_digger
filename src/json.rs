use std::{collections::HashMap, io::Write};

use mysql::serde_json::json;

pub fn write<W>(rows: Vec<Vec<String>>, mut output: W) -> anyhow::Result<()>
where
    W: Write,
{
    let headers = match rows.first() {
        Some(header_row) => header_row.to_owned(),
        None => panic!("No header row found"),
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
