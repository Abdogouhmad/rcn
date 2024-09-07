use rcn::{Commands, Rcn};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rcn = Rcn::parse(&args);

    match rcn.commands {
        Some(Commands::Modeles { name, ts_js }) => {
            Commands::create_modele(name, ts_js).expect("can't create");
        }
        Some(Commands::Help) => Rcn::handle_help(),
        Some(Commands::Version) => println!("{}", Rcn::print_version()),
        None => {
            eprintln!("\x1b[1;31mNo Command provided.\x1b[0m \x1b[1;33mTry:\x1b[0m rcn help");
        }
    }
}
