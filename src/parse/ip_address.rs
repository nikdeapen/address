use crate::ParseError::InvalidIPAddress;
use crate::{IPAddress, IPv4Address, IPv6Address, ParseError, impl_parse};

impl IPAddress {
    //! Parse

    /// Parses the IP address text. (a public `&[u8]` conversion would read as raw octets, not text)
    pub(crate) fn parse(ip: &[u8]) -> Result<Self, ParseError> {
        if let Ok(ip) = IPv4Address::parse(ip) {
            Ok(ip.to_ip())
        } else if let Ok(ip) = IPv6Address::parse(ip) {
            Ok(ip.to_ip())
        } else {
            Err(InvalidIPAddress)
        }
    }
}

impl_parse!(IPAddress, parse);

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
        ];

        for (input, expected) in test_cases {
            let result: Result<IPAddress, ParseError> = IPAddress::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<IPAddress, ParseError> = IPAddress::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
