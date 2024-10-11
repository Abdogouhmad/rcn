use core::panic;

use rcn::cli::{Commands, Rcn, RcnError};

#[test]
fn parse_modeles() {
    let args = vec![
        "rcn".to_string(),
        "model".to_string(),
        "MyComponent".to_string(),
        "ts".to_string(),
    ];
    let rcn = Rcn::parse(&args).expect("Failed to parse 'model' command");
    if let Some(Commands::Models { name, ts_js }) = rcn.commands {
        assert_eq!(name, "MyComponent");
        assert_eq!(ts_js, "ts");
    } else {
        panic!("Expected models command.");
    }
}

#[test]
fn unknown_subcommand() {
    let args = vec!["rcn".to_string(), "unknown".to_string()];

    let result = Rcn::parse(&args);

    if let Err(RcnError::UnknownSubcommand(subcmd)) = result {
        assert_eq!(subcmd, "unknown");
    } else {
        panic!("Expected UnknownSubcommand error");
    }
}

#[test]
fn invalid_argument_for_model() {
    let args = vec![
        "rcn".to_string(),
        "model".to_string(),
        "MyComponent".to_string(),
        "invalid_lang".to_string(),
    ];

    let result = Rcn::parse(&args);

    if let Err(RcnError::InvalidArgument(arg)) = result {
        assert_eq!(arg, "invalid_lang");
    } else {
        panic!("Expected InvalidArgument error");
    }
}
