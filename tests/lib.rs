#[cfg(test)]
mod tests {
    use gold_digger::{get_extension_from_filename, rows_to_strings};
    use mysql::{Column, ColumnType, Row, Value};

    #[test]
    fn test_get_extension_from_filename() {
        assert_eq!(get_extension_from_filename("file.csv"), Some("csv"));
        assert_eq!(get_extension_from_filename("file.json"), Some("json"));
        assert_eq!(get_extension_from_filename("file"), None);
    }

    #[test]
    fn test_rows_to_strings_empty() {
        let rows: Vec<Row> = vec![];
        let result = rows_to_strings(rows).unwrap();
        assert_eq!(result.len(), 0);
    }

    // More comprehensive tests would require constructing mock mysql::Row objects,
    // which is non-trivial without a database or more advanced mocking.
}
