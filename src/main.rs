use std::{env, fs, io::Error, path::Path};

struct Rcn {
    commands: Option<Commands>,
}

enum Commands {
    Modeles { name: String, ts_js: String },
    Help,
    Version,
}

impl Rcn {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Self { commands: None };
        }

        let subcommand = &args[1];

        match subcommand.as_str() {
            "modeles" => {
                Self::args_check("modeles".to_string(), &args);
                let name = args[2].clone();
                let ts_js = args[3].clone();
                Self {
                    commands: Some(Commands::Modeles { name, ts_js }),
                }
            }
            "help" | "-h" => Self {
                commands: Some(Commands::Help),
            },
            "--version" | "-V" => Self {
                commands: Some(Commands::Version),
            },
            _ => {
                eprintln!("Error: Unknown subcommand '{}'.", subcommand);
                std::process::exit(1);
            }
        }
    }

    // check the arguments
    fn args_check(commandname: String, args: &[String]) {
        if args.len() < 3 {
            eprintln!(
                "Error: '{}' subcommand requires a name argument.",
                commandname
            );
            std::process::exit(1);
        }
    }

    // Print the help message
    pub fn handle_help() {
        println!(
            "rcn is a react component generator that generates the component with needed files\n"
        );
        println!("Usage: rcn <subcommand> [options]");
        println!("\n");
        println!("commands:");
        println!("  modeles <name> <language>    Generate a React component with the specified name and language (js or ts).");
        println!("  help/-h                      Display this help message.");
        println!("  --version/-V                 Display the version of the tool.");
        println!("\n");
        println!("Examples:");
        println!("  rcn modeles MyComponent js   Generate a JavaScript React component named MyComponent.");
        println!("  rcn modeles MyComponent ts   Generate a TypeScript React component named MyComponent.");
        println!("  rcn help/-h                  Display this help message.");
        println!("  rcn --version/-V             Display the version of the tool.");
    }

    // print the version make it dynamic
    pub fn print_version() {
        println!("rcn version {}", env!("CARGO_PKG_VERSION"));
    }
}

// Implementation for Commands (if needed)
impl Commands {
    pub fn create_modele(name: String, ts_js: String) -> Result<(), Error> {
        let extensions_js = vec![".component.jsx", ".service.js"];
        let extensions_ts = vec![".component.tsx", ".service.ts", ".types.ts"];

        // Create the directory if it doesn't exist
        if !Path::new(&name).exists() {
            fs::create_dir_all(&name)?;
        }

        let extensions = if ts_js.contains("js") {
            extensions_js
        } else if ts_js.contains("ts") {
            extensions_ts
        } else {
            eprintln!("Unsupported language: {}", ts_js);
            std::process::exit(1);
        };

        for extension in extensions {
            let file_to_create = format!("{}{}", name, extension);
            let file_path = Path::new(&name).join(&file_to_create);

            // Create the file
            fs::File::create(file_path.clone())?;

            println!("Created file: {}", file_path.display());
        }
        Ok(())
    }
}

fn main() {
    let rcn = Rcn::parse();

    match rcn.commands {
        Some(Commands::Modeles { name, ts_js }) => {
            Commands::create_modele(name, ts_js).expect("can't create");
        }
        Some(Commands::Help) => Rcn::handle_help(),
        Some(Commands::Version) => Rcn::print_version(),
        None => {
            println!("No subcommand provided.");
        }
    }
}
