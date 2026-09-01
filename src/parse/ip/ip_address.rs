use crate::ParseError::InvalidIPAddress;
use crate::{IPAddress, IPv4Address, IPv6Address, ParseError, impl_parse};

impl IPAddress {
    //! Parse

    /// Parses an [IPAddress] from the `text`.
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        if let Ok(ip) = IPv4Address::parse(text) {
            Ok(ip.to_ip())
        } else if let Ok(ip) = IPv6Address::parse(text) {
            Ok(ip.to_ip())
        } else {
            Err(InvalidIPAddress)
        }
    }
}

impl_parse!(IPAddress);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPAddress;
    use crate::{IPAddress, IPv4Address, IPv6Address, ParseError};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<IPAddress, ParseError>);

    #[test]
    fn parse() {
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidIPAddress)),
            (b"127.0.0.1", Ok(IPv4Address::LOCALHOST.to_ip())),
            (b"::1", Ok(IPv6Address::LOCALHOST.to_ip())),
            (
                b"::ffff:1.2.3.4",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0xFFFF, 0x0102, 0x0304]).to_ip()),
            ),
            (b"[::1]", Err(InvalidIPAddress)),
            (b"fe80::1%1", Err(InvalidIPAddress)),
            (b"localhost", Err(InvalidIPAddress)),
            (b"\xFF", Err(InvalidIPAddress)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPAddress, ParseError> = IPAddress::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<IPAddress, ParseError> = IPAddress::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<IPAddress, ParseError> = IPAddress::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &[
            "127.0.0.1",
            "255.255.255.255",
            "::1",
            "fe80::1",
            "::ffff:1.2.3.4",
        ];

        for input in canonical {
            let value: IPAddress = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
