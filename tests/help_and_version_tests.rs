use core::panic;

use rcn::{Commands, Rcn};

#[test]
fn long_help() {
    let args = vec!["rcn".to_string(), "help".to_string()];

    let rcn = Rcn::parse(&args);

    if let Some(Commands::Help) = rcn.commands {
    } else {
        panic!("Expected Help command.")
    }
}

#[test]
fn short_help() {
    let args = vec!["rcn".to_string(), "-h".to_string()];

    let rcn = Rcn::parse(&args);

    if let Some(Commands::Help) = rcn.commands {
    } else {
        panic!("Expected help command.")
    }
}

#[test]
fn long_version() {
    let args = vec!["rcn".to_string(), "version".to_string()];

    let rcn = Rcn::parse(&args);

    if let Some(Commands::Version) = rcn.commands {
        let version = format!("rcn version {}", env!("CARGO_PKG_VERSION"));
        assert_eq!(Rcn::print_version(), version)
    } else {
        panic!("Expected version command")
    }
}
#[test]
fn short_version() {
    let args = vec!["rcn".to_string(), "-V".to_string()];

    let rcn = Rcn::parse(&args);

    if let Some(Commands::Version) = rcn.commands {
        let version = format!("rcn version {}", env!("CARGO_PKG_VERSION"));
        assert_eq!(Rcn::print_version(), version)
    } else {
        panic!("Expected version/-V command")
    }
}
