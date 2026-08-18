use crate::parse_port;
use crate::{IPv4Address, ParseError, SocketAddressV4, impl_parse};

impl SocketAddressV4 {
    //! Parse

    /// An IPv4 address & a decimal port: `127.0.0.1:80`.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        let (ip, port): (&[u8], u16) = parse_port(text)?;
        let ip: IPv4Address = IPv4Address::parse_text(ip)?;
        Ok(Self::new(ip, port))
    }
}

impl_parse!(SocketAddressV4, "An IPv4 address & a decimal port: `127.0.0.1:80`.");

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv4Address, InvalidPort};
    use crate::{IPv4Address, ParseError, SocketAddressV4};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<SocketAddressV4, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("127.0.0.1:", Err(InvalidPort)),
            ("127.0.0.1:xx", Err(InvalidPort)),
            (":80", Err(InvalidIPv4Address)),
            ("xx:80", Err(InvalidIPv4Address)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_socket(80))),
            ("127.0.0.1:65535", Ok(IPv4Address::LOCALHOST.to_socket(65535))),
            ("127.0.0.1:65536", Err(InvalidPort)),
            ("[127.0.0.1]:80", Err(InvalidIPv4Address)),
            ("::1:80", Err(InvalidIPv4Address)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::parse_text(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    /// Non-UTF-8 bytes reach the parser through the public `parse_text`.
    #[test]
    fn parse_text_non_utf8() {
        let test_cases: &[(&[u8], ParseError)] = &[
            (b"\xFF:80", InvalidIPv4Address),
            (b"127.0.0.\xFF:80", InvalidIPv4Address),
            (b"127.0.0.1:\xFF", InvalidPort),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::parse_text(input);
            assert_eq!(result, Err(*expected), "input={:?}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["0.0.0.0:0", "127.0.0.1:80", "255.255.255.255:65535"];

        for input in canonical {
            let value: SocketAddressV4 = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
