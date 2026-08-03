use std::str::FromStr;

use crate::ParseError;
use crate::ParseError::InvalidPort;

/// Parses the port from the `address` string.
///
/// Returns `(address_without_last_colon, port)`.
///
/// The port must be canonical decimal: digits only, with no sign and no leading zeros.
///
/// # Examples
/// localhost:80    -> `Ok("localhost", 80)`
/// :80             -> `Ok("", 80)`
/// :0              -> `Ok("", 0)`
/// :8x             -> `Err(InvalidPort)`
/// :+80            -> `Err(InvalidPort)`
/// :080            -> `Err(InvalidPort)`
/// 80              -> `Err(InvalidPort)`
pub(crate) fn parse_port(address: &str) -> Result<(&str, u16), ParseError> {
    if let Some(colon) = address.as_bytes().iter().rposition(|c| *c == b':') {
        let port: &str = &address[colon + 1..];
        let canonical: bool = !port.is_empty()
            && port.bytes().all(|c| c.is_ascii_digit())
            && (port.len() == 1 || !port.starts_with('0'));
        if !canonical {
            return Err(InvalidPort);
        }
        let port: u16 = u16::from_str(port).map_err(|_| InvalidPort)?;
        let s: &str = &address[..colon];
        Ok((s, port))
    } else {
        Err(InvalidPort)
    }
}
