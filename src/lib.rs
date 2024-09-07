use std::{env, fs, io::Error, path::Path};

pub struct Rcn<'a> {
    pub commands: Option<Commands<'a>>,
}

pub enum Commands<'a> {
    Modeles { name: &'a str, ts_js: &'a str },
    Help,
    Version,
}

impl<'a> Rcn<'a> {
    pub fn parse(args: &'a [String]) -> Self {
        if args.len() < 2 {
            return Self { commands: None };
        }

        let subcommand = &args[1];

        match subcommand.as_str() {
            "modeles" => {
                Self::args_check("modeles", args);
                let name = &args[2];
                let ts_js = &args[3];
                Self {
                    commands: Some(Commands::Modeles { name, ts_js }),
                }
            }
            "help" | "-h" => Self {
                commands: Some(Commands::Help),
            },
            "version" | "-V" => Self {
                commands: Some(Commands::Version),
            },
            _ => {
                eprintln!("Error: Unknown subcommand '{}'.", subcommand);
                std::process::exit(1);
            }
        }
    }

    // check the arguments
    pub fn args_check(commandname: &str, args: &[String]) {
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
            "\x1b[1;32mrcn is a react component generator that generates the component with needed files\x1b[0m\n"
        );
        println!("\x1b[1;34mUsage: rcn <subcommand> [options]\x1b[0m");
        println!("\n");
        println!("\x1b[1;34mSubcommands:\x1b[0m");
        println!("  \x1b[1;36mmodeles <name> <language>\x1b[0m    Generate a React component with the specified name and language (js or ts).");
        println!("  \x1b[1;36mhelp\x1b[0m                         Display this help message.");
        println!(
            "  \x1b[1;36m--version/-V\x1b[0m                 Display the version of the tool."
        );
        println!("\n");
        println!("\x1b[1;34mExamples:\x1b[0m");
        println!("  \x1b[1;36mrcn modeles MyComponent js\x1b[0m   Generate a JavaScript React component named MyComponent.");
        println!("  \x1b[1;36mrcn modeles MyComponent ts\x1b[0m   Generate a TypeScript React component named MyComponent.");
        println!("  \x1b[1;36mrcn help/-h\x1b[0m                  Display this help message.");
        println!(
            "  \x1b[1;36mrcn --version/-V\x1b[0m             Display the version of the tool."
        );
    }

    // print the version make it dynamic
    pub fn print_version() -> String {
        format!("rcn version {}", env!("CARGO_PKG_VERSION"))
    }
}

// Implementation for Commands (if needed)
impl<'a> Commands<'a> {
    pub fn create_modele(name: &str, ts_js: &str) -> Result<(), Error> {
        let extensions_js = vec![".component.jsx", ".service.js"];
        let extensions_ts = vec![".component.tsx", ".service.ts", ".types.ts"];

        // Create the directory if it doesn't exist
        if !Path::new(&name).exists() {
            fs::create_dir_all(name)?;
        }

        let extensions = match ts_js {
            "js" => extensions_js,
            "ts" => extensions_ts,
            _ => {
                eprintln!("Unsupported language: {}", ts_js);
                std::process::exit(1);
            }
        };

        for extension in extensions {
            let file_to_create = format!("{}{}", name, extension);
            let file_path = Path::new(&name).join(file_to_create.to_lowercase());

            // Create the file
            fs::File::create(file_path.clone())?;

            println!("Created file: {}", file_path.display());
        }
        Ok(())
    }
}
