use shitty_daemon::args_parser::args_parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = match args_parser() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    println!("{:?}", args);
    Ok(())
}
