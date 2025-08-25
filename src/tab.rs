use std::io::Write;

use csv::{QuoteStyle, WriterBuilder};

/// Writes rows to a tab-delimited output using the provided writer.
///
/// # Arguments
///
/// * `rows` - An iterator over records, where each record is an iterator over fields.
/// * `output` - A writer to output the tab-delimited data.
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
    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(QuoteStyle::Necessary)
        .buffer_capacity(8 * 1024) // 8KB buffer for better performance
        .from_writer(output);

    for row in rows {
        wtr.write_record(row)?;
    }

    wtr.flush()?; // Ensure all data is written
    Ok(())
}

/// Writes rows to a tab-delimited output using the provided writer with generic field types.
///
/// This version accepts any type that can be converted to bytes, providing
/// better performance by avoiding unnecessary string allocations.
///
/// # Arguments
///
/// * `rows` - An iterator over records, where each record is an iterator over fields.
/// * `output` - A writer to output the tab-delimited data.
///
/// # Returns
///
/// A Result indicating success or failure.
pub fn write_bytes<R, F, T, W>(rows: R, output: W) -> anyhow::Result<()>
where
    R: IntoIterator<Item = F>,
    F: IntoIterator<Item = T>,
    T: AsRef<[u8]>,
    W: Write,
{
    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(QuoteStyle::Necessary)
        .from_writer(output);

    for row in rows {
        wtr.write_record(row)?;
    }

    Ok(())
}
