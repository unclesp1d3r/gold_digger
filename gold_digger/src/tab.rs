use csv::{QuoteStyle, WriterBuilder};
use std::io::Write;

pub fn write<W>(rows: Vec<Vec<String>>, output: W) -> anyhow::Result<()>
where
    W: Write,
{
    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(QuoteStyle::Necessary)
        .from_writer(output);

    for row in rows.iter() {
        wtr.write_record(row)?;
    }

    Ok(())
}
