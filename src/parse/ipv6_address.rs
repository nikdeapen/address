use crate::ParseError::InvalidIPv6Address;
use crate::{IPv6Address, ParseError, impl_parse};
use std::net::Ipv6Addr;
use std::str::FromStr;

impl IPv6Address {
    //! Parse

    /// The maximum length of an IPv6 address string. (ffff:ffff:ffff:ffff:ffff:ffff:255.255.255.255)
    const MAX_STR_LEN: usize = 45;

    /// Parses the IPv6 address text. (a public `&[u8]` conversion would read as raw octets, not text)
    pub(crate) fn parse(ip: &[u8]) -> Result<Self, ParseError> {
        if ip.len() > Self::MAX_STR_LEN {
            return Err(InvalidIPv6Address);
        }
        let ip: &str = std::str::from_utf8(ip).map_err(|_| InvalidIPv6Address)?;
        Ok(Ipv6Addr::from_str(ip).map_err(|_| InvalidIPv6Address)?.into())
    }
}

impl_parse!(IPv6Address, parse);

#[cfg(test)]
mod tests {
    use crate::ParseError::InvalidIPv6Address;
    use crate::{IPv6Address, ParseError};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<IPv6Address, ParseError>)] = &[
            ("", Err(InvalidIPv6Address)),
            ("::", Ok(IPv6Address::UNSPECIFIED)),
            ("::1", Ok(IPv6Address::LOCALHOST)),
        ];

        for (input, expected) in test_cases {
            let result: Result<IPv6Address, ParseError> = IPv6Address::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<IPv6Address, ParseError> = IPv6Address::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
