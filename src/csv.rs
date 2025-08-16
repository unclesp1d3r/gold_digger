use std::io::Write;

use csv::{QuoteStyle, WriterBuilder};

/// Writes rows to a CSV output using the provided writer.
///
/// # Arguments
///
/// * `rows` - A vector of string vectors representing the data.
/// * `output` - A writer to output the CSV data.
///
/// # Returns
///
/// A Result indicating success or failure.
pub fn write<W>(rows: Vec<Vec<String>>, output: W) -> anyhow::Result<()>
where
    W: Write,
{
    let mut wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::NonNumeric)
        .from_writer(output);

    for row in rows.iter() {
        wtr.write_record(row)?;
    }

    Ok(())
}
