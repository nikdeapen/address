use crate::ParseError;
use crate::ParseError::InvalidPort;
use std::str::FromStr;

/// Parses the port from the `text`.
///
/// Returns `(text_without_last_colon, port)`.
///
/// The port must be decimal digits only, with no sign. Leading zeros are allowed to match the standard library.
/// The digit check runs first, so the port is known to be ASCII before it is read as a string.
///
/// # Examples
/// `localhost:80` -> `Ok(("localhost", 80))`
/// `:80`          -> `Ok(("", 80))`
/// `:080`         -> `Ok(("", 80))`
/// `:0`           -> `Ok(("", 0))`
/// `:8x`          -> `Err(InvalidPort)`
/// `:+80`         -> `Err(InvalidPort)`
/// `80`           -> `Err(InvalidPort)`
pub(crate) fn parse_port(text: &[u8]) -> Result<(&[u8], u16), ParseError> {
    if let Some(colon) = text.iter().rposition(|c| *c == b':') {
        let port: &[u8] = &text[colon + 1..];
        let valid: bool = !port.is_empty() && port.iter().all(|c| c.is_ascii_digit());
        if !valid {
            return Err(InvalidPort);
        }
        let port: &str = unsafe { std::str::from_utf8_unchecked(port) };
        let port: u16 = u16::from_str(port).map_err(|_| InvalidPort)?;
        Ok((&text[..colon], port))
    } else {
        Err(InvalidPort)
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::InvalidPort;
    use crate::parse_port;

    type TestCase<'a> = (&'a str, Result<(&'a str, u16), ParseError>);

    #[test]
    fn ports() {
        let test_cases: &[TestCase] = &[
            ("", Err(InvalidPort)),
            ("80", Err(InvalidPort)),
            (":", Err(InvalidPort)),
            ("localhost:80", Ok(("localhost", 80))),
            (":80", Ok(("", 80))),
            (":0", Ok(("", 0))),
            (":8x", Err(InvalidPort)),
            (":+80", Err(InvalidPort)),
            (":-80", Err(InvalidPort)),
            (":080", Ok(("", 80))),
            (":00", Ok(("", 0))),
            (":00080", Ok(("", 80))),
            (":65535", Ok(("", 65535))),
            (":065535", Ok(("", 65535))),
            (":65536", Err(InvalidPort)),
            (":99999", Err(InvalidPort)),
            (":18446744073709551616", Err(InvalidPort)),
            ("a:b:80", Ok(("a:b", 80))),
            ("[::1]:80", Ok(("[::1]", 80))),
        ];

        for (input, expected) in test_cases {
            let result: Result<(&[u8], u16), ParseError> = parse_port(input.as_bytes());
            let expected: Result<(&[u8], u16), ParseError> = match expected {
                Ok((s, port)) => Ok((s.as_bytes(), *port)),
                Err(error) => Err(*error),
            };
            assert_eq!(result, expected, "input={}", input);
        }
    }

    /// Non-UTF-8 bytes reach `parse_port` through the `&[u8]` parse impls; the digit check must reject them
    /// before the port is read as a string.
    #[test]
    fn non_utf8_ports() {
        let test_cases: &[&[u8]] = &[b":\xFF", b":8\xFF", b"localhost:\xFF", b":\xC3\xA9"];

        for input in test_cases {
            let result: Result<(&[u8], u16), ParseError> = parse_port(input);
            assert_eq!(result, Err(InvalidPort), "input={:?}", input);
        }
    }
}
