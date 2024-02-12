pub fn args_parser() -> Result<Vec<String>, &'static str> {
    if std::env::args().len() < 2 {
        return Err("Please provide a file path");
    }
    Ok(std::env::args().collect())
}
