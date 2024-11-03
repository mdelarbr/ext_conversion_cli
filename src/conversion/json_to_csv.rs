use csv::Writer;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

use crate::utils::files::{check_file_exists, check_file_format};

pub fn convert_json_to_csv(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    check_file_exists(input_path)?;
    check_file_format(input_path, "json")?;

    let file = File::open(input_path)?;
    let json_data: Vec<HashMap<String, Value>> = serde_json::from_reader(file)?;

    write_data_into_csv(json_data, output_path)?;

    println!("Conversion done");
    Ok(())
}

fn write_data_into_csv(
    json_data: Vec<HashMap<String, Value>>,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(output_path)?;

    let headers: Vec<String> = if let Some(first_record) = json_data.first() {
        first_record.keys().cloned().collect()
    } else {
        return Err("Empty JSON file".into());
    };
    wtr.write_record(&headers)?;

    for row in json_data.iter() {
        let csv_row: Vec<String> = headers
            .iter()
            .map(|header| {
                row.get(header)
                    .map(|value| match value {
                        Value::String(s) => s.clone(),
                        _ => value.to_string(),
                    })
                    .unwrap_or_else(|| "".to_string())
            })
            .collect();

        wtr.write_record(&csv_row)?;
    }

    wtr.flush()?;
    Ok(())
}
