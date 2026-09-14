use crate::ParseError::InvalidIPv4Address;
use crate::{IPv4Address, ParseError, impl_parse};
use std::net::Ipv4Addr;
use std::str::FromStr;

impl IPv4Address {
    //! Parse

    /// The maximum length of an IPv4 address string. (255.255.255.255)
    const MAX_STR_LEN: usize = 15;

    /// Parses an [IPv4Address] from the `text`.
    ///
    /// # Notes
    /// - No leading zeros, matching the standard library. (`127.0.0.01` is invalid)
    pub fn parse(text: &[u8]) -> Result<Self, ParseError> {
        if text.len() > Self::MAX_STR_LEN {
            return Err(InvalidIPv4Address);
        }
        let text: &str = std::str::from_utf8(text).map_err(|_| InvalidIPv4Address)?;
        Ok(Ipv4Addr::from_str(text)
            .map_err(|_| InvalidIPv4Address)?
            .into())
    }
}

impl_parse!(IPv4Address);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPv4Address;
    use crate::{IPv4Address, ParseError};
    use std::str::FromStr;

    type TestCase<'a> = (&'a [u8], Result<IPv4Address, ParseError>);

    #[test]
    fn parse() {
        let over_max: Vec<u8> = vec![b'1'; IPv4Address::MAX_STR_LEN + 1];
        let test_cases: &[TestCase] = &[
            (b"", Err(InvalidIPv4Address)),
            (b"1.2.3.4", Ok(IPv4Address::from([1, 2, 3, 4]))),
            (b"0.0.0.0", Ok(IPv4Address::UNSPECIFIED)),
            (b"127.0.0.1", Ok(IPv4Address::LOCALHOST)),
            (b"255.255.255.255", Ok(IPv4Address::BROADCAST)),
            (b"127.0.0.01", Err(InvalidIPv4Address)),
            (b"127.000.000.001", Err(InvalidIPv4Address)),
            (b"1.2.3", Err(InvalidIPv4Address)),
            (b"1.2.3.4.5", Err(InvalidIPv4Address)),
            (b"256.1.1.1", Err(InvalidIPv4Address)),
            (b"127.0.0.\xFF", Err(InvalidIPv4Address)),
            (b"\xFF\xFF\xFF\xFF", Err(InvalidIPv4Address)),
            (over_max.as_slice(), Err(InvalidIPv4Address)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPv4Address, ParseError> = IPv4Address::parse(input);
            assert_eq!(result, *expected, "parse input={:?}", input);

            let Ok(text) = std::str::from_utf8(input) else {
                continue;
            };

            let result: Result<IPv4Address, ParseError> = IPv4Address::from_str(text);
            assert_eq!(result, *expected, "from_str input={}", text);

            let result: Result<IPv4Address, ParseError> = IPv4Address::try_from(text);
            assert_eq!(result, *expected, "try_from(&str) input={}", text);
        }
    }

    #[test]
    fn round_trip() {
        let canonical: &[&str] = &["0.0.0.0", "127.0.0.1", "1.2.3.4", "255.255.255.255"];

        for input in canonical {
            let value: IPv4Address = input.parse().unwrap();
            assert_eq!(value.to_string(), *input, "input={}", input);
        }
    }
}
