use crate::ParseError;
use crate::ParseError::InvalidPort;
use crate::parse_digits;

/// Parses the port from the `text`, returning `(text_before_port_colon, port)`.
///
/// # Notes
/// - The port must be decimal digits only, with no sign.
/// - Leading zeros are allowed, matching the standard library.
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
    let colon: usize = text.iter().rposition(|c| *c == b':').ok_or(InvalidPort)?;
    let port: u16 = parse_digits(&text[colon + 1..])
        .and_then(|port| u16::try_from(port).ok())
        .ok_or(InvalidPort)?;
    Ok((&text[..colon], port))
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

    #[test]
    fn non_utf8_ports() {
        let test_cases: &[&[u8]] = &[b":\xFF", b":8\xFF", b"localhost:\xFF", b":\xC3\xA9"];

        for input in test_cases {
            let result: Result<(&[u8], u16), ParseError> = parse_port(input);
            assert_eq!(result, Err(InvalidPort), "input={:?}", input);
        }
    }
}
