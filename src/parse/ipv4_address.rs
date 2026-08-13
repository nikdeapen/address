use crate::ParseError::InvalidIPv4Address;
use crate::{IPv4Address, ParseError, impl_parse};
use std::net::Ipv4Addr;
use std::str::FromStr;

impl IPv4Address {
    //! Parse

    /// The maximum length of an IPv4 address string. (255.255.255.255)
    const MAX_STR_LEN: usize = 15;

    /// Parses the IPv4 address text. (a public `&[u8]` conversion would read as raw octets, not text)
    pub(crate) fn parse(ip: &[u8]) -> Result<Self, ParseError> {
        if ip.len() > Self::MAX_STR_LEN {
            return Err(InvalidIPv4Address);
        }
        let ip: &str = std::str::from_utf8(ip).map_err(|_| InvalidIPv4Address)?;
        Ok(Ipv4Addr::from_str(ip).map_err(|_| InvalidIPv4Address)?.into())
    }
}

impl_parse!(IPv4Address, parse);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPv4Address;
    use crate::{IPv4Address, ParseError};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<IPv4Address, ParseError>)] = &[
            ("", Err(InvalidIPv4Address)),
            ("0.0.0.0", Ok(IPv4Address::UNSPECIFIED)),
            ("127.0.0.1", Ok(IPv4Address::LOCALHOST)),
            ("255.255.255.255", Ok(IPv4Address::BROADCAST)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPv4Address, ParseError> = IPv4Address::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<IPv4Address, ParseError> = IPv4Address::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
