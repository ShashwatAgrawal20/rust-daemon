use shitty_daemon::args_parser::args_parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args_parser() {
        Ok(filename) => println!("Filename extracted: {}", filename),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    Ok(())
}
