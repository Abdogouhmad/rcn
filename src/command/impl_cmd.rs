use crate::cli::Commands;
use std::{fs, io::Error, path::Path};

// Implementation for Commands (if needed)
impl<'a> Commands<'a> {
    pub fn generate_react_files(name: &str, ts_js: &str) -> Result<(), Error> {
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
