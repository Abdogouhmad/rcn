use crate::cli::Rcn;

use super::{Commands, RcnError};

impl<'a> Rcn<'a> {
    pub fn parse(args: &'a [String]) -> Result<Self, RcnError> {
        if args.len() < 2 {
            return Err(RcnError::MissingSubcommand);
        }

        let subcommand = &args[1];

        match subcommand.as_str() {
            "model" => {
                Self::args_check("model", args)?;
                let name = &args[2];
                let ts_js = &args[3];
                Ok(Self {
                    commands: Some(Commands::Models { name, ts_js }),
                })
            }
            "help" | "-h" => Ok(Self {
                commands: Some(Commands::Help),
            }),
            "version" | "-V" => Ok(Self {
                commands: Some(Commands::Version),
            }),
            _ => Err(RcnError::UnknownSubcommand(subcommand.to_string())),
        }
    }

    // Updated to return a Result and propagate errors
    pub fn args_check(command_name: &str, args: &[String]) -> Result<(), RcnError> {
        // check the number of args required for this subcommand
        if args.len() < 3 {
            return Err(RcnError::MissingArgument(command_name.to_string()));
        }

        // check the model last args (js/ts)
        if args.len() < 4 {
            return Err(RcnError::MissingArgument("ts/js".to_string()));
        }

        // check wither it is js or ts if not prompt error
        let ts_js = &args[3];
        if ts_js != "ts" && ts_js != "js" {
            return Err(RcnError::InvalidArgument(ts_js.to_string()));
        }

        Ok(())
    }

    // Print the help message
    pub fn handle_help() {
        println!(
            "\x1b[1;32mrcn is a react component generator that generates the component with needed files\x1b[0m\n"
        );
        println!("\x1b[1;34mUsage: rcn <subcommand> [options]\x1b[0m");
        println!("\n");
        println!("\x1b[1;34mSubcommands:\x1b[0m");
        println!("  \x1b[1;36mmodel <name> <language>\x1b[0m    Generate a React component with the specified name and language (js or ts).");
        println!("  \x1b[1;36mhelp\x1b[0m                         Display this help message.");
        println!(
            "  \x1b[1;36m--version/-V\x1b[0m                 Display the version of the tool."
        );
        println!("\n");
        println!("\x1b[1;34mExamples:\x1b[0m");
        println!("  \x1b[1;36mrcn model MyComponent js\x1b[0m   Generate a JavaScript React component named MyComponent.");
        println!("  \x1b[1;36mrcn model MyComponent ts\x1b[0m   Generate a TypeScript React component named MyComponent.");
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
