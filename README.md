
# Gold Digger

A simple MySQL/MariaDB query tool that accepts parameters as environmental variables.


![GitHub](https://img.shields.io/github/license/unclesp1d3r/gold_digger)
![GitHub issues](https://img.shields.io/github/issues/unclesp1d3r/gold_digger)
![GitHub Repo stars](https://img.shields.io/github/stars/unclesp1d3r/gold_digger?style=social)
![GitHub last commit](https://img.shields.io/github/last-commit/unclesp1d3r/gold_digger)
![Maintenance](https://img.shields.io/maintenance/yes/2022)
## Installation

Build and install using cargo

```bash
git clone git@github.com:unclesp1d3r/gold_digger.git
cd gold_digger
cargo install
```
    
## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`OUTPUT_FILE` This is the path to a text file that will contain the output of the query. The extension of the file will determine the format (csv, txt, or json)

`DATABASE_URL` The connection url for accessing the database. This is formatted in the typical MySQL/MariDB format (`protocol//[hosts][/database][?properties]`)

`DATABASE_QUERY` The SQL query string to be used to query the database server.
## Authors

- [@unclesp1d3r](https://www.github.com/unclesp1d3r)


## Contributing

Contributions are always welcome!

See `CONTRIBUTING.md` for ways to get started.

Please adhere to this project's `CODE_OF_CONDUCT.md`.


## Feedback

If you have any feedback, please reach out to us at unclespider@protonmail.com

