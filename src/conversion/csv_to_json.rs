use std::{error::Error, fs::File};

use csv::ReaderBuilder;
use serde_json::{json, to_writer_pretty, Value};

use crate::utils::files::{check_file_exists, check_file_format};

pub fn convert_csv_to_json(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    check_file_exists(input_path)?;
    check_file_format(input_path, "csv")?;

    let file = File::open(input_path)?;
    let (csv_header, csv_data) = read_csv(file)?;

    let mut json_data = Vec::new();

    for data in csv_data.iter() {
        let record = data;
        let mut row_object = serde_json::Map::new();

        for (header, field) in csv_header.iter().zip(record.iter()) {
            row_object.insert(header.to_string(), json!(field));
        }

        json_data.push(Value::Object(row_object));
    }

    format_and_create_json_file(&Value::Array(json_data), output_path)?;
    Ok(())
}

fn read_csv(file: File) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(file);

    let csv_headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();
    let mut csv_data = Vec::new();

    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        csv_data.push(row);
    }

    Ok((csv_headers, csv_data))
}

fn format_and_create_json_file(json_data: &Value, output_path: &str) -> Result<(), Box<dyn Error>> {
    let output_file = File::create(output_path)?;

    to_writer_pretty(output_file, json_data)?;

    Ok(())
}
