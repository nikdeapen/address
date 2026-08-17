use crate::ParseError::InvalidSocketAddressV6;
use crate::parse_port;
use crate::{IPv6Address, ParseError, SocketAddressV6, doc_ignored_zone, impl_parse};

impl_parse!(SocketAddressV6, doc_ignored_zone!());

impl TryFrom<&[u8]> for SocketAddressV6 {
    type Error = ParseError;

    #[doc = doc_ignored_zone!()]
    fn try_from(socket: &[u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(socket)?;
        match IPv6Address::parse_bracketed(s) {
            Some(ip) => Ok(Self::new(ip?, port)),
            None => Err(InvalidSocketAddressV6),
        }
    }
}

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
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::try_from(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
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
