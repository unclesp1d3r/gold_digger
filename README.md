# Gold Digger

Gold Digger is a Rust-based query tool that automates the routine collection of database queries for MySQL and MariaDB systems. This tool is designed to run headless, making it ideal for use in scheduled or routine tasks.

![GitHub](https://img.shields.io/github/license/unclesp1d3r/gold_digger)
![GitHub issues](https://img.shields.io/github/issues/unclesp1d3r/gold_digger)
![GitHub Repo stars](https://img.shields.io/github/stars/unclesp1d3r/gold_digger?style=social)
![GitHub last commit](https://img.shields.io/github/last-commit/unclesp1d3r/gold_digger)
![Maintenance](https://img.shields.io/maintenance/yes/2023)

## Description

This tool is configurable using environmental variables, allowing you to set up your database connection details and other parameters without modifying the source code. It accepts parameters such as output file path, database connection URL, and SQL query string, making it easy to use in a variety of settings and on different systems.

Overall, Gold Digger is a practical solution for managing and analyzing data in MySQL and MariaDB environments. With its headless design and configurable options, it's well-suited for regular use in any database administration workflow.

## Installation

To build and install Gold Digger, use the following commands in your terminal:

```bash
git clone git@github.com:unclesp1d3r/gold_digger.git
cd gold_digger
cargo install
```

## Environment Variables

To run Gold Digger, you will need to set the following environment variables in your .env file or in your environment:

-   `OUTPUT_FILE`: This is the path to a text file that will contain the output of the query. The extension of the file
    will determine the format (csv, txt, or json).

-   `DATABASE_URL`: The connection URL for accessing the database. This is formatted in the typical MySQL/MariaDB
    format (`protocol://[host]/[database]?[properties]`).

-   `DATABASE_QUERY`: The SQL query string to be used to query the database server.

## Authors

Gold Digger is authored by [@unclesp1d3r](https://www.github.com/unclesp1d3r)

## Contributing and Feedback

We welcome your feedback and suggestions for Gold Digger! If you have any ideas for new features, encounter any bugs or
issues, or have any other comments, please reach out to us by creating an issue on
our [GitHub repository](https://github.com/unclesp1d3r/gold_digger/issues).

If you're interested in contributing to Gold Digger, we encourage you to submit a pull request. Please see
our `CONTRIBUTING.md` for more information on how to get started.

Our team is committed to providing a welcoming and inclusive environment for all contributors. Please adhere to
our `CODE_OF_CONDUCT.md` when contributing to the project.

Thank you for your interest in Gold Digger, and we look forward to hearing from you!
