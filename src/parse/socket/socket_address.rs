use crate::ParseError::InvalidSocketAddressV6;
use crate::{
    IPAddress, IPv4Address, IPv6Address, ParseError, SocketAddress, impl_parse, parse_port,
};

impl SocketAddress {
    //! Parse

    /// Parses a [SocketAddress] from the `text`.
    ///
    /// # Notes
    /// - An IPv6 address must be bracketed: `[::1]:80`.
    /// - A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    /// - The error is the one [`SocketAddressV6`](crate::SocketAddressV6) returns if the address is
    ///   bracketed or contains a colon, or the one [`SocketAddressV4`](crate::SocketAddressV4)
    ///   returns otherwise.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        let (ip, port): (&[u8], u16) = parse_port(text)?;
        let ip: IPAddress = if ip.starts_with(b"[") {
            IPv6Address::parse_bracketed(ip)?.to_ip()
        } else if ip.contains(&b':') {
            return Err(InvalidSocketAddressV6);
        } else {
            IPv4Address::parse(ip)?.to_ip()
        };
        Ok(ip.to_socket(port))
    }
}

impl_parse!(SocketAddress);

#[cfg(test)]
mod tests {
    use crate::ParseError::{
        InvalidIPv4Address, InvalidIPv6Address, InvalidPort, InvalidSocketAddressV6,
    };
    use crate::{
        IPv4Address, IPv6Address, ParseError, SocketAddress, SocketAddressV4, SocketAddressV6,
    };
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
                Ok(IPv6Address::LOCALHOST.to_ip().to_socket(80)),
            ),
            (
                b"[::1%1]:80",
                Ok(IPv6Address::LOCALHOST.to_ip().to_socket(80)),
            ),
            (b"[]:80", Err(InvalidIPv6Address)),
            (b"[xx]:80", Err(InvalidIPv6Address)),
            (b"[::1%]:80", Err(InvalidIPv6Address)),
            (b"[::1%eth0]:80", Err(InvalidIPv6Address)),
            (b"[::1%4294967296]:80", Err(InvalidIPv6Address)),
            (b"[127.0.0.1]:80", Err(InvalidIPv6Address)),
            (b"[\xFF]:80", Err(InvalidIPv6Address)),
            (b":80", Err(InvalidIPv4Address)),
            (b"xx:80", Err(InvalidIPv4Address)),
            (b"1.2.3:80", Err(InvalidIPv4Address)),
            (b"256.0.0.1:80", Err(InvalidIPv4Address)),
            (b"\xFF:80", Err(InvalidIPv4Address)),
            (b"::1:80", Err(InvalidSocketAddressV6)),
            (b"fe80::1:80", Err(InvalidSocketAddressV6)),
            (b"::1]:80", Err(InvalidSocketAddressV6)),
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
    fn parse_matches_v4_or_v6() {
        let v4_cases: &[&[u8]] = &[
            b"127.0.0.1:80",
            b"",
            b":80",
            b"xx:80",
            b"1.2.3:80",
            b"256.0.0.1:80",
            b"127.0.0.1:65536",
            b"\xFF:80",
        ];

        for input in v4_cases {
            let result: Result<SocketAddress, ParseError> = SocketAddress::parse(input);
            let expected: Result<SocketAddress, ParseError> =
                SocketAddressV4::parse(input).map(SocketAddressV4::to_socket);
            assert_eq!(result, expected, "input={:?}", input);
        }

        let v6_cases: &[&[u8]] = &[
            b"[::1]:80",
            b"[::1%1]:80",
            b"[]:80",
            b"[xx]:80",
            b"[::1%eth0]:80",
            b"[127.0.0.1]:80",
            b"[::1]:65536",
            b"::1:80",
            b"fe80::1:80",
            b"::1]:80",
        ];

        for input in v6_cases {
            let result: Result<SocketAddress, ParseError> = SocketAddress::parse(input);
            let expected: Result<SocketAddress, ParseError> =
                SocketAddressV6::parse(input).map(SocketAddressV6::to_socket);
            assert_eq!(result, expected, "input={:?}", input);
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
