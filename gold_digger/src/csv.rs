use std::io::Write;

use csv::{QuoteStyle, WriterBuilder};

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
