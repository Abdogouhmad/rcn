use rcn::cli::{Commands, Rcn};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Match on the result of Rcn::parse to handle errors
    match Rcn::parse(&args) {
        Ok(rcn) => match rcn.commands {
            Some(Commands::Models { name, ts_js }) => {
                Commands::generate_react_files(name, ts_js)
                    .expect("Error: Could not create React files.");
            }
            Some(Commands::Help) => Rcn::handle_help(),
            Some(Commands::Version) => println!("{}", Rcn::print_version()),
            None => {
                eprintln!("\x1b[1;31mNo Command provided.\x1b[0m \x1b[1;33mTry:\x1b[0m rcn help");
            }
        },
        Err(e) => {
            eprintln!("\x1b[1;31m{}\x1b[0m", e);
            eprintln!("\x1b[1;33mTry:\x1b[0m rcn help");
        }
    }
}
