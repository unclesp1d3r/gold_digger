use anyhow::Result;
use gold_digger::{csv, get_extension_from_filename, rows_to_strings, tab};
use mysql::{prelude::Queryable, Conn, Opts, Row};
use std::{env, fs::File};

fn main() -> Result<()> {
    let output_file = match env::var("OUTPUT_FILE") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("couldn't find OUTPUT_FILE in environment variable");
            std::process::exit(-1);
        }
    };

    let database_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("couldn't find DATABASE_URL in environment variable");
            std::process::exit(-1);
        }
    };

    let database_query = match env::var("DATABASE_QUERY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("couldn't find DATABASE_QUERY in environment variable");
            std::process::exit(-1);
        }
    };

    let opts = Opts::from_url(&database_url)?;
    let mut conn = Conn::new(opts)?;

    println!("Connecting to database...");
    let result: Vec<Row> = conn.query(database_query)?;
    println!("Outputting {} records in {}.", result.len(), &output_file);

    if result.is_empty() {
        println!("No records found in database.");
        std::process::exit(1);
    } else {
        let rows = rows_to_strings(result)?;
        let output = File::create(&output_file)?;

        match get_extension_from_filename(&output_file) {
            Some("csv") => csv::write(rows, output)?,
            Some("json") => gold_digger::json::write(rows, output)?,
            Some(&_) => tab::write(rows, output)?,
            None => {
                eprintln!("Couldn't find extension");
                std::process::exit(-1);
            }
        }
    }

    Ok(())
}
