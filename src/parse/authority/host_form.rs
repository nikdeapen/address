use crate::ParseError::InvalidAuthority;
use crate::{IPAddress, IPv6Address, ParseError};

/// The host form of an authority.
#[must_use]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum HostForm {
    /// An IP address host, bracketed or not.
    IP(IPAddress),

    /// A domain name host, which the caller parses from the text.
    Domain,
}

impl HostForm {
    //! Classification

    /// Classifies the authority `host`, rejecting an unbracketed IPv6 address.
    ///
    /// The domain form is not validated; the caller parses it to reuse its buffer.
    pub(crate) fn classify(host: &[u8]) -> Result<Self, ParseError> {
        if let Some(ip) = IPv6Address::parse_bracketed(host) {
            Ok(Self::IP(ip?.to_ip()))
        } else if let Ok(ip) = IPAddress::parse(host) {
            if ip.is_v6() {
                Err(InvalidAuthority)
            } else {
                Ok(Self::IP(ip))
            }
        } else {
            Ok(Self::Domain)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidIPv6Address};
    use crate::{HostForm, IPv4Address, IPv6Address, ParseError};

    #[test]
    fn classify() {
        let test_cases: &[(&str, Result<HostForm, ParseError>)] = &[
            ("[::1]", Ok(HostForm::IP(IPv6Address::LOCALHOST.to_ip()))),
            ("[::1%1]", Ok(HostForm::IP(IPv6Address::LOCALHOST.to_ip()))),
            ("[::1%eth0]", Err(InvalidIPv6Address)),
            ("[]", Err(InvalidIPv6Address)),
            ("[127.0.0.1]", Err(InvalidIPv6Address)),
            (
                "127.0.0.1",
                Ok(HostForm::IP(IPv4Address::LOCALHOST.to_ip())),
            ),
            ("::1", Err(InvalidAuthority)),
            ("fe80::1", Err(InvalidAuthority)),
            ("localhost", Ok(HostForm::Domain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<HostForm, ParseError> = HostForm::classify(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    /// The domain form only means "not an IP address"; the caller does the validating.
    #[test]
    fn classify_does_not_validate_domains() {
        let test_cases: &[&str] = &["", "Local!Host", "a..b", "-a"];

        for input in test_cases {
            let result: Result<HostForm, ParseError> = HostForm::classify(input.as_bytes());
            assert_eq!(result, Ok(HostForm::Domain), "input={}", input);
        }
    }
}
