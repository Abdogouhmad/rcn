use core::panic;

use rcn::{Commands, Rcn};

#[test]
fn parse_modeles() {
    let args = vec![
        "rcn".to_string(),
        "modeles".to_string(),
        "MyComponent".to_string(),
        "ts".to_string(),
    ];
    let rcn = Rcn::parse(&args);
    if let Some(Commands::Modeles { name, ts_js }) = rcn.commands {
        assert_eq!(name, "MyComponent");
        assert_eq!(ts_js, "ts");
    } else {
        panic!("Expected Modeles command.");
    }
}

#[test]
#[should_panic]
fn parse_modeles_wrongly() {
    let args = vec!["rcn".to_string()];

    let rcn = Rcn::parse(&args);
    if let Some(Commands::Modeles { name, ts_js }) = rcn.commands {
        assert_eq!(name, "Statusbar");
        assert_eq!(ts_js, "ts");
    } else {
        panic!("Expecated modeles command.")
    }
}
