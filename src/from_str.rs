use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::ParseError::InvalidPort;

/// An error parsing an address.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum ParseError {
    InvalidDomain,
    InvalidEndpoint,
    InvalidIPv4Address,
    InvalidIPv6Address,
    InvalidIPAddress,
    InvalidPort,
    InvalidSocketAddressV4,
    InvalidSocketAddressV6,
    InvalidSocketAddress,
    InvalidHost,
    InvalidAuthority,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

/// Extracts the port from address string.
pub(crate) fn extract_port(s: &str) -> Result<(&str, u16), ParseError> {
    if let Some(colon) = s.as_bytes().iter().rposition(|c| *c == b':') {
        let port: u16 = u16::from_str(&s[colon + 1..]).map_err(|_| InvalidPort)?;
        let s: &str = &s[..colon];
        Ok((s, port))
    } else {
        Err(InvalidPort)
    }
}
