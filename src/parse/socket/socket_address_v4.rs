use crate::{IPv4Address, ParseError, SocketAddressV4, impl_parse, parse_port};

impl SocketAddressV4 {
    //! Parse

    /// Parses a [SocketAddressV4] from the `text`.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        let (ip, port): (&[u8], u16) = parse_port(text)?;
        let ip: IPv4Address = IPv4Address::parse(ip)?;
        Ok(ip.to_socket(port))
    }
}

impl_parse!(SocketAddressV4);

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv4Address, InvalidPort};
    use crate::{IPv4Address, ParseError, SocketAddressV4};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<SocketAddressV4, ParseError>);

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidPort)),
            (b"127.0.0.1:", Err(InvalidPort)),
            (b"127.0.0.1:xx", Err(InvalidPort)),
            (b"127.0.0.1:65536", Err(InvalidPort)),
            (b"127.0.0.1:\xFF", Err(InvalidPort)),
            (b"127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_socket(80))),
            (
                b"127.0.0.1:65535",
                Ok(IPv4Address::LOCALHOST.to_socket(65535)),
            ),
            (b":80", Err(InvalidIPv4Address)),
            (b"xx:80", Err(InvalidIPv4Address)),
            (b"[127.0.0.1]:80", Err(InvalidIPv4Address)),
            (b"::1:80", Err(InvalidIPv4Address)),
            (b"\xFF:80", Err(InvalidIPv4Address)),
            (b"127.0.0.\xFF:80", Err(InvalidIPv4Address)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["0.0.0.0:0", "127.0.0.1:80", "255.255.255.255:65535"];

        for input in canonical {
            let value: SocketAddressV4 = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
