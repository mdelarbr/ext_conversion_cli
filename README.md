# JSON-CSV Converter CLI
A simple command-line tool written in Rust to convert between JSON and CSV formats. This tool is designed to facilitate data manipulation and make it easy to convert JSON data to CSV and vice versa, ideal for backend data processing, configuration setup, and client reporting tasks.

It’s a little project to start having fun with Rust!

## Dependencies
**Rust**: Version 1.56 or higher (supports edition 2021)</br>
[![serde](https://img.shields.io/badge/serde-1.0-blue)](https://github.com/serde-rs/serde): Serialization framework.</br>
[![serde_json](https://img.shields.io/badge/serde__json-1.0-blue)](https://github.com/serde-rs/json): JSON parsing.</br>
[![csv](https://img.shields.io/badge/csv-1.1-blue)](https://github.com/BurntSushi/rust-csv): CSV I/O.</br>

## Features
- Convert JSON to CSV: Takes a JSON file and generates a CSV file.
- Convert CSV to JSON: Takes a CSV file and generates a JSON file.


## Usage
### JSON to CSV
Convert a JSON file to CSV format.

```bash
./json-csv-converter --input data.json --output data.csv --format csv
```

### CSV to JSON
Convert a CSV file to JSON format.

```bash
./json-csv-converter --input data.csv --output data.json --format json
```

### Arguments
- `--input or -i: Path to the input file.`
- `--output or -o: Path to the output file.`
- `--format or -f: Format to convert to (csv or json).`

### Example
Convert data.json to output.csv:
```bash
./json-csv-converter --input data.json --output output.csv --format csv
```

## Project Structure
```plaintext
.
├── src/
│   ├── main.rs           # CLI entry point
│   ├── conversion/       # Conversion functions
│   │   ├── json_to_csv.rs
│   │   └── csv_to_json.rs
│   └── utils/            # Utility functions
│       ├── files.rs      # File checking and validation functions
│       └── mod.rs
├── Cargo.toml            # Project dependencies and metadata
└── README.md             # Project README
```
