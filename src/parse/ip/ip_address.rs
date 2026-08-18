use crate::ParseError::InvalidIPAddress;
use crate::{IPAddress, IPv4Address, IPv6Address, ParseError, impl_parse};

impl IPAddress {
    //! Parse

    /// Parses the IP address text.
    pub fn parse_text(text: &[u8]) -> Result<Self, ParseError> {
        if let Ok(ip) = IPv4Address::parse_text(text) {
            Ok(ip.to_ip())
        } else if let Ok(ip) = IPv6Address::parse_text(text) {
            Ok(ip.to_ip())
        } else {
            Err(InvalidIPAddress)
        }
    }
}

impl_parse!(IPAddress, "An IPv4 or an IPv6 address in the standard library syntax.");

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPAddress;
    use crate::{IPAddress, IPv4Address, IPv6Address, ParseError};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<IPAddress, ParseError>)] = &[
            ("", Err(InvalidIPAddress)),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST.to_ip())),
            ("::1", Ok(IPv6Address::LOCALHOST.to_ip())),
            (
                "::ffff:1.2.3.4",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0xFFFF, 0x0102, 0x0304]).to_ip()),
            ),
            ("[::1]", Err(InvalidIPAddress)),
            ("fe80::1%1", Err(InvalidIPAddress)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPAddress, ParseError> = IPAddress::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<IPAddress, ParseError> = IPAddress::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<IPAddress, ParseError> = IPAddress::parse_text(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["127.0.0.1", "255.255.255.255", "::1", "fe80::1", "::ffff:1.2.3.4"];

        for input in canonical {
            let value: IPAddress = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
