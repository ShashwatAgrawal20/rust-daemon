use shitty_daemon::args_parser::args_parser;
use shitty_daemon::watcher::watcher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args_parser() {
        Ok(file) => watcher(&file)?,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };
    Ok(())
}
