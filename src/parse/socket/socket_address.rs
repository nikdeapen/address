use crate::ParseError::InvalidSocketAddress;
use crate::parse_port;
use crate::{IPv4Address, IPv6Address, ParseError, SocketAddress, impl_parse};

impl SocketAddress {
    //! Parse

    /// An IPv4 address or a bracketed IPv6 address, & a decimal port: `127.0.0.1:80` or `[::1]:80`.
    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        let (ip, port): (&[u8], u16) = parse_port(text)?;
        if let Some(ip) = IPv6Address::parse_bracketed(ip) {
            Ok(ip?.to_ip().to_socket(port))
        } else {
            let ip: IPv4Address = IPv4Address::parse_text(ip).map_err(|_| InvalidSocketAddress)?;
            Ok(ip.to_ip().to_socket(port))
        }
    }
}

impl_parse!(
    SocketAddress,
    "An IPv4 address or a bracketed IPv6 address, & a decimal port: `127.0.0.1:80` or `[::1]:80`.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv6Address, InvalidPort, InvalidSocketAddress};
    use crate::{IPv4Address, IPv6Address, ParseError, SocketAddress};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<SocketAddress, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("[::1]:", Err(InvalidPort)),
            ("[::1]:xx", Err(InvalidPort)),
            (":80", Err(InvalidSocketAddress)),
            ("xx:80", Err(InvalidSocketAddress)),
            ("::1:80", Err(InvalidSocketAddress)),
            ("[]:80", Err(InvalidIPv6Address)),
            ("[xx]:80", Err(InvalidIPv6Address)),
            ("[::1%eth0]:80", Err(InvalidIPv6Address)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_ip().to_socket(80))),
            ("0.0.0.0:0", Ok(IPv4Address::UNSPECIFIED.to_ip().to_socket(0))),
            (
                "255.255.255.255:65535",
                Ok(IPv4Address::BROADCAST.to_ip().to_socket(65535)),
            ),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80).to_socket())),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80).to_socket())),
            ("[::1%]:80", Err(InvalidIPv6Address)),
            ("[::1%4294967296]:80", Err(InvalidIPv6Address)),
            ("[127.0.0.1]:80", Err(InvalidIPv6Address)),
            ("127.0.0.1:65536", Err(InvalidPort)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddress, ParseError> = SocketAddress::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddress, ParseError> = SocketAddress::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddress, ParseError> = SocketAddress::parse_text(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    /// Non-UTF-8 bytes reach the parser through the public `parse_text`.
    #[test]
    fn parse_text_non_utf8() {
        let test_cases: &[(&[u8], ParseError)] = &[
            (b"\xFF:80", InvalidSocketAddress),
            (b"[\xFF]:80", InvalidIPv6Address),
            (b"127.0.0.1:\xFF", InvalidPort),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddress, ParseError> = SocketAddress::parse_text(input);
            assert_eq!(result, Err(*expected), "input={:?}", input);
        }
    }

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["127.0.0.1:80", "[::1]:443", "[fe80::1]:0", "0.0.0.0:0"];

        for input in canonical {
            let value: SocketAddress = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
