use std::fmt::{Display, Formatter};

/// An error parsing an address.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ParseError {
    InvalidDomain,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Self::InvalidDomain => "invalid domain",
        };
        write!(f, "{}", s)
    }
}

impl std::error::Error for ParseError {}
