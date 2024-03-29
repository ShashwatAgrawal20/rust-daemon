use std::path::Path;

fn extract_filename(file_path: &str) -> Option<String> {
    let path = Path::new(file_path);

    if path.exists() {
        if let Some(file) = path.to_str() {
            return Some(file.to_string());
        }
    }
    None
}

pub fn args_parser() -> Result<String, &'static str> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Please provide a file path");
    }

    match extract_filename(&args[1]) {
        Some(filename) => Ok(filename),
        None => Err("Invalid filename"),
    }
}
