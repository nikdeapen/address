use crate::ParseError::InvalidSocketAddress;
use crate::parse_port;
use crate::{IPv4Address, IPv6Address, ParseError, SocketAddress, impl_parse};

impl SocketAddress {
    //! Parse

    /// Parses a [SocketAddress] from the `text`.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        let (ip, port): (&[u8], u16) = parse_port(text)?;
        if let Some(ip) = IPv6Address::parse_bracketed(ip) {
            Ok(ip?.to_ip().to_socket(port))
        } else {
            let ip: IPv4Address = IPv4Address::parse(ip).map_err(|_| InvalidSocketAddress)?;
            Ok(ip.to_ip().to_socket(port))
        }
    }
}

impl_parse!(
    SocketAddress,
    "An IPv6 address must be bracketed: `[::1]:80`.",
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv6Address, InvalidPort, InvalidSocketAddress};
    use crate::{IPv4Address, IPv6Address, ParseError, SocketAddress};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<SocketAddress, ParseError>);

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"[::1]:", Err(InvalidPort)),
            (b"[::1]:xx", Err(InvalidPort)),
            (b"127.0.0.1:65536", Err(InvalidPort)),
            (b"127.0.0.1:\xFF", Err(InvalidPort)),
            (
                b"127.0.0.1:80",
                Ok(IPv4Address::LOCALHOST.to_ip().to_socket(80)),
            ),
            (
                b"0.0.0.0:0",
                Ok(IPv4Address::UNSPECIFIED.to_ip().to_socket(0)),
            ),
            (
                b"255.255.255.255:65535",
                Ok(IPv4Address::BROADCAST.to_ip().to_socket(65535)),
            ),
            (
                b"[::1]:80",
                Ok(IPv6Address::LOCALHOST.to_socket(80).to_socket()),
            ),
            (
                b"[::1%1]:80",
                Ok(IPv6Address::LOCALHOST.to_socket(80).to_socket()),
            ),
            (b"[]:80", Err(InvalidIPv6Address)),
            (b"[xx]:80", Err(InvalidIPv6Address)),
            (b"[::1%]:80", Err(InvalidIPv6Address)),
            (b"[::1%eth0]:80", Err(InvalidIPv6Address)),
            (b"[::1%4294967296]:80", Err(InvalidIPv6Address)),
            (b"[127.0.0.1]:80", Err(InvalidIPv6Address)),
            (b"[\xFF]:80", Err(InvalidIPv6Address)),
            (b":80", Err(InvalidSocketAddress)),
            (b"xx:80", Err(InvalidSocketAddress)),
            (b"::1:80", Err(InvalidSocketAddress)),
            (b"\xFF:80", Err(InvalidSocketAddress)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddress, ParseError> = SocketAddress::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<SocketAddress, ParseError> = SocketAddress::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<SocketAddress, ParseError> = SocketAddress::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["127.0.0.1:80", "[::1]:443", "[fe80::1]:0", "0.0.0.0:0"];

        for input in canonical {
            let value: SocketAddress = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
