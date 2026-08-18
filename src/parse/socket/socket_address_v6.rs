use crate::ParseError::InvalidSocketAddressV6;
use crate::parse_port;
use crate::{IPv6Address, ParseError, SocketAddressV6, impl_parse};

impl SocketAddressV6 {
    //! Parse

    /// A bracketed IPv6 address & a decimal port: `[::1]:80`.
    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        let (ip, port): (&[u8], u16) = parse_port(text)?;
        match IPv6Address::parse_bracketed(ip) {
            Some(ip) => Ok(Self::new(ip?, port)),
            None => Err(InvalidSocketAddressV6),
        }
    }
}

impl_parse!(
    SocketAddressV6,
    "A bracketed IPv6 address & a decimal port: `[::1]:80`.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv6Address, InvalidPort, InvalidSocketAddressV6};
    use crate::{IPv6Address, ParseError, SocketAddressV6};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<SocketAddressV6, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("[::1]:", Err(InvalidPort)),
            ("[::1]:xx", Err(InvalidPort)),
            (":80", Err(InvalidSocketAddressV6)),
            ("xx:80", Err(InvalidSocketAddressV6)),
            ("[xx]:80", Err(InvalidIPv6Address)),
            ("[::1%]:80", Err(InvalidIPv6Address)),
            ("[::1%eth0]:80", Err(InvalidIPv6Address)),
            ("[::1%4294967296]:80", Err(InvalidIPv6Address)),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80))),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80))),
            ("[]:80", Err(InvalidIPv6Address)),
            ("::1:80", Err(InvalidSocketAddressV6)),
            ("[::1]:65536", Err(InvalidPort)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::parse_text(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    /// Non-UTF-8 bytes reach the parser through the public `parse_text`.
    #[test]
    fn parse_text_non_utf8() {
        let test_cases: &[(&[u8], ParseError)] = &[
            (b"[\xFF]:80", InvalidIPv6Address),
            (b"\xFF:80", InvalidSocketAddressV6),
            (b"[::1]:\xFF", InvalidPort),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::parse_text(input);
            assert_eq!(result, Err(*expected), "input={:?}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["[::]:0", "[::1]:80", "[::ffff:1.2.3.4]:443", "[fe80::1]:65535"];

        for input in canonical {
            let value: SocketAddressV6 = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
