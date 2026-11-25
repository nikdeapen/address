use std::fmt::{Display, Formatter};

/// An error parsing an address.
///
/// # Examples
/// ```
/// use address::{Domain, ParseError};
///
/// let error: ParseError = "bad_name".parse::<Domain>().unwrap_err();
/// assert_eq!(error, ParseError::InvalidDomain);
/// assert_eq!(error.to_string(), "invalid domain");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// The domain is invalid.
    InvalidDomain,

    /// The IP address is invalid. (neither IPv4 nor IPv6)
    InvalidIPAddress,

    /// The IPv4 address is invalid.
    InvalidIPv4Address,

    /// The IPv6 address is invalid.
    InvalidIPv6Address,

    /// The socket address is invalid. (neither IPv4 nor bracketed IPv6)
    InvalidSocketAddress,

    /// The IPv6 socket address is invalid. (the IPv6 address must be bracketed)
    InvalidSocketAddressV6,

    /// The port is missing or invalid. (decimal, 0 to 65535, leading zeros allowed)
    InvalidPort,

    /// The host is invalid. (neither an IP address nor a domain)
    InvalidHost,

    /// The authority is invalid. (an IPv6 host must be bracketed)
    InvalidAuthority,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            Self::InvalidDomain => "invalid domain",
            Self::InvalidIPAddress => "invalid IP address",
            Self::InvalidIPv4Address => "invalid IPv4 address",
            Self::InvalidIPv6Address => "invalid IPv6 address",
            Self::InvalidSocketAddress => "invalid socket address",
            Self::InvalidSocketAddressV6 => "invalid IPv6 socket address",
            Self::InvalidPort => "invalid port",
            Self::InvalidHost => "invalid host",
            Self::InvalidAuthority => "invalid authority",
        };
        f.pad(s)
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::{
        InvalidAuthority, InvalidDomain, InvalidHost, InvalidIPAddress, InvalidIPv4Address,
        InvalidIPv6Address, InvalidPort, InvalidSocketAddress, InvalidSocketAddressV6,
    };

    #[test]
    fn display() {
        let test_cases: &[(ParseError, &str)] = &[
            (InvalidDomain, "invalid domain"),
            (InvalidIPAddress, "invalid IP address"),
            (InvalidIPv4Address, "invalid IPv4 address"),
            (InvalidIPv6Address, "invalid IPv6 address"),
            (InvalidSocketAddress, "invalid socket address"),
            (InvalidSocketAddressV6, "invalid IPv6 socket address"),
            (InvalidPort, "invalid port"),
            (InvalidHost, "invalid host"),
            (InvalidAuthority, "invalid authority"),
        ];

        for (error, expected) in test_cases {
            assert_eq!(error.to_string(), *expected, "error={:?}", error);
        }

        assert_eq!(format!("{:>16}", InvalidPort), "    invalid port");
    }

    #[test]
    fn std_error() {
        let error: Box<dyn std::error::Error> = Box::new(InvalidPort);
        assert_eq!(error.to_string(), "invalid port");
        assert!(error.source().is_none());
    }
}
