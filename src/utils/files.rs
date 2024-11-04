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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_file_exists_success() {

        let temp_file = "test_exists.json";
        std::fs::File::create(temp_file).unwrap();
        
        assert!(check_file_exists(temp_file).is_ok());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_check_file_exists_failure() {
        assert!(check_file_exists("non_existent_file.json").is_err());
    }

    #[test]
    fn test_check_format_json_success() {
        let temp_file = "test_format_json_s.json";
        std::fs::File::create(temp_file).unwrap();

        assert!(check_file_format(temp_file, "json").is_ok());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_check_format_json_fail_csv() {
        let temp_file = "test_format_json_f.csv";
        std::fs::File::create(temp_file).unwrap();

        assert!(check_file_format(temp_file, "json").is_err());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_check_format_json_fail_empty() {
        let temp_file = "test_format_json_e";
        std::fs::File::create(temp_file).unwrap();

        assert!(check_file_format(temp_file, "json").is_err());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_check_format_csv_success() {
        let temp_file = "test_format_csv_s.csv";
        std::fs::File::create(temp_file).unwrap();

        assert!(check_file_format(temp_file, "csv").is_ok());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_check_format_csv_fail_json() {
        let temp_file = "test_format_csv_f.json";
        std::fs::File::create(temp_file).unwrap();

        assert!(check_file_format(temp_file, "csv").is_err());

        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_check_format_csv_fail_empty() {
        let temp_file = "test_format_csv_e";
        std::fs::File::create(temp_file).unwrap();

        assert!(check_file_format(temp_file, "csv").is_err());

        std::fs::remove_file(temp_file).unwrap();
    }
}