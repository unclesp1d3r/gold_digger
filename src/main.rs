use std::{env, fs::File};

use anyhow::Result;
use mysql::prelude::Queryable;
use mysql::Pool;

use gold_digger::{get_extension_from_filename, rows_to_strings};

fn main() -> Result<()> {
    let output_file = match env::var("OUTPUT_FILE") {
        Ok(val) => val,
        Err(_) => {
            #[cfg(feature = "verbose")]
            eprintln!("couldn't find OUTPUT_FILE in environment variable");
            std::process::exit(-1);
        },
    };

    let database_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            #[cfg(feature = "verbose")]
            eprintln!("couldn't find DATABASE_URL in environment variable");
            std::process::exit(-1);
        },
    };

    let database_query = match env::var("DATABASE_QUERY") {
        Ok(val) => val,
        Err(_) => {
            #[cfg(feature = "verbose")]
            eprintln!("couldn't find DATABASE_QUERY in environment variable");
            std::process::exit(-1);
        },
    };

    let pool = Pool::new(database_url.as_str())?;
    let mut conn = pool.get_conn()?;

    #[cfg(feature = "verbose")]
    println!("Connecting to database...");
    let result: Vec<mysql::Row> = conn.query(database_query)?;
    #[cfg(feature = "verbose")]
    println!("Outputting {} records in {}.", result.len(), &output_file);

    if result.is_empty() {
        #[cfg(feature = "verbose")]
        println!("No records found in database.");
        std::process::exit(1);
    } else {
        let rows = rows_to_strings(result)?;
        let output = File::create(&output_file)?;

        match get_extension_from_filename(&output_file) {
            #[cfg(feature = "csv")]
            Some("csv") => gold_digger::csv::write(rows, output)?,
            #[cfg(feature = "json")]
            Some("json") => gold_digger::json::write(rows, output)?,
            Some(&_) => gold_digger::tab::write(rows, output)?,
            None => {
                #[cfg(feature = "verbose")]
                eprintln!("Couldn't find extension");
                std::process::exit(-1);
            },
        }
    }

    Ok(())
}
