use std::path::Path;

pub fn check_file_exists(file_path: &str) -> Result<(), String> {
    if Path::new(file_path).exists() {
        Ok(())
    } else {
        Err(format!("File {} does not exist.", file_path))
    }
}

pub fn check_file_format(file_path: &str, expected_extension: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    match path.extension() {
        Some(ext) if ext == expected_extension => Ok(()),
        Some(ext) => Err(format!(
            "Unexpected extension, excpected: {} and find {:?}",
            expected_extension, ext
        )),
        None => Err("Unable to find format".to_string()),
    }
}
