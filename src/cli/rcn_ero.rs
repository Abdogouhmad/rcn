use std::fmt;

#[derive(Debug)]
pub enum RcnError {
    MissingSubcommand,
    UnknownSubcommand(String),
    MissingArgument(String),
    InvalidArgument(String),
    CustomError(String),
}

impl fmt::Display for RcnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RcnError::MissingSubcommand => write!(f, "Error: Missing subcommand."),
            RcnError::UnknownSubcommand(subcmd) => {
                write!(f, "Error: Unknown subcommand '{}'.", subcmd)
            }
            RcnError::MissingArgument(arg) => {
                write!(f, "Error: Missing required argument '{}'.", arg)
            }
            RcnError::InvalidArgument(arg) => write!(f, "Error: Invalid argument '{}'.", arg),
            RcnError::CustomError(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for RcnError {}
