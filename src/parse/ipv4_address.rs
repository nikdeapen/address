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

impl_parse!(
    IPv4Address,
    parse,
    "Matches the standard library: four decimal octets, no leading zeros. (`127.0.0.01` is invalid)"
);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPv4Address;
    use crate::{IPv4Address, ParseError};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<IPv4Address, ParseError>)] = &[
            ("", Err(InvalidIPv4Address)),
            ("127.0.0.01", Err(InvalidIPv4Address)),
            ("127.000.000.001", Err(InvalidIPv4Address)),
            ("1.2.3", Err(InvalidIPv4Address)),
            ("1.2.3.4.5", Err(InvalidIPv4Address)),
            ("256.1.1.1", Err(InvalidIPv4Address)),
            ("1.2.3.4", Ok(IPv4Address::from([1, 2, 3, 4]))),
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

    /// Each canonical string must parse and display back to the exact same string.
    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["0.0.0.0", "127.0.0.1", "1.2.3.4", "255.255.255.255"];

        for input in canonical {
            let value: IPv4Address = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
