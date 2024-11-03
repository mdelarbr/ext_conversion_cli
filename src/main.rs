use clap::Parser;

mod conversion;
mod utils;
use conversion::csv_to_json::convert_csv_to_json;
use conversion::json_to_csv::convert_json_to_csv;

#[derive(Parser)]
#[command(
    author = "mdelarbr",
    version = "0.1",
    about = "Convert JSON to CSV and vice-versa"
)]
struct Cli {
    /// Input path of the file.
    #[arg(short, long)]
    input_path: String,

    /// Output path of the converted file.
    #[arg(short, long)]
    output_path: String,

    /// Convertion format of the output "json" or "csv"
    #[arg(short, long)]
    format: String,
}

fn main() {
    let args = Cli::parse();

    let conversion_fn = match args.format.as_str() {
        "csv" => convert_json_to_csv,
        "json" => convert_csv_to_json,
        _ => {
            eprintln!("Unsupported format, use 'json' or 'csv'.");
            return;
        }
    };

    if let Err(error) = conversion_fn(&args.input_path, &args.output_path) {
        eprintln!("Error: {}", error);
    }
}
