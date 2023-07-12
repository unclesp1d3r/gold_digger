use std::{ffi::OsStr, path::Path};

use mysql::{from_value, Row};

pub mod csv;
pub mod json;
pub mod tab;

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
            .map(|column| from_value::<String>(row[column.name_str().as_ref()].to_owned()))
            .collect::<Vec<String>>();
        result_rows.push(data_row);
    }

    Ok(result_rows)
}

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}
