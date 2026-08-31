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

    type TestCase<'a> = (&'a [u8], Result<SocketAddressV6, ParseError>);

    /// Every entry point must agree on every case, non-UTF-8 bytes included.
    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"[::1]:", Err(InvalidPort)),
            (b"[::1]:xx", Err(InvalidPort)),
            (b"[::1]:65536", Err(InvalidPort)),
            (b"[::1]:\xFF", Err(InvalidPort)),
            (b"[::1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80))),
            (b"[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80))),
            (b"[xx]:80", Err(InvalidIPv6Address)),
            (b"[]:80", Err(InvalidIPv6Address)),
            (b"[::1%]:80", Err(InvalidIPv6Address)),
            (b"[::1%eth0]:80", Err(InvalidIPv6Address)),
            (b"[::1%4294967296]:80", Err(InvalidIPv6Address)),
            (b"[\xFF]:80", Err(InvalidIPv6Address)),
            (b":80", Err(InvalidSocketAddressV6)),
            (b"xx:80", Err(InvalidSocketAddressV6)),
            (b"::1:80", Err(InvalidSocketAddressV6)),
            (b"\xFF:80", Err(InvalidSocketAddressV6)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::parse_text(input);
            assert_eq!(result, *expected, "parse_text input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "[::]:0",
            "[::1]:80",
            "[::ffff:1.2.3.4]:443",
            "[fe80::1]:65535",
        ];

        for input in canonical {
            let value: SocketAddressV6 = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
