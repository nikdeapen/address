use crate::ParseError::{InvalidAuthority, InvalidHost};
use crate::{IPAddress, IPv4Address, IPv6Address, ParseError};

/// The host form of an authority.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub(crate) enum HostForm {
    /// An IP address host, bracketed or not.
    IP(IPAddress),

    /// A domain name host. The domain must still be validated.
    Domain,
}

impl HostForm {
    //! Classification

    /// Classifies the authority `host`.
    pub(crate) fn classify(host: &[u8]) -> Result<Self, ParseError> {
        if host.starts_with(b"[") {
            Ok(Self::IP(IPv6Address::parse_bracketed(host)?.to_ip()))
        } else if let Ok(ip) = IPv4Address::parse(host) {
            Ok(Self::IP(ip.to_ip()))
        } else {
            Ok(Self::Domain)
        }
    }

    /// Gets the error for an invalid domain `host`.
    pub(crate) fn domain_error(host: &[u8]) -> ParseError {
        if host.contains(&b':') {
            InvalidAuthority
        } else {
            InvalidHost
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidIPv6Address};
    use crate::{HostForm, IPv4Address, IPv6Address, ParseError};

    #[test]
    fn classify() {
        let test_cases: &[(&str, Result<HostForm, ParseError>)] = &[
            ("[::1]", Ok(HostForm::IP(IPv6Address::LOCALHOST.to_ip()))),
            ("[::1%1]", Ok(HostForm::IP(IPv6Address::LOCALHOST.to_ip()))),
            ("[::1%eth0]", Err(InvalidIPv6Address)),
            ("[]", Err(InvalidIPv6Address)),
            ("[127.0.0.1]", Err(InvalidIPv6Address)),
            ("[::1", Err(InvalidIPv6Address)),
            (
                "127.0.0.1",
                Ok(HostForm::IP(IPv4Address::LOCALHOST.to_ip())),
            ),
            ("::1", Ok(HostForm::Domain)),
            ("localhost", Ok(HostForm::Domain)),
        ];

        for (input, expected) in test_cases {
            let result: Result<HostForm, ParseError> = HostForm::classify(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn domain_error() {
        let test_cases: &[(&str, ParseError)] = &[
            ("::1", InvalidAuthority),
            ("fe80::1", InvalidAuthority),
            ("2001:db8::1", InvalidAuthority),
            ("a:b", InvalidAuthority),
            (":", InvalidAuthority),
            ("", InvalidHost),
            ("Local!Host", InvalidHost),
        ];

        for (input, expected) in test_cases {
            let result: ParseError = HostForm::domain_error(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn classify_does_not_validate_domains() {
        let test_cases: &[&str] = &["", "Local!Host", "a..b", "-a"];

        for input in test_cases {
            let result: Result<HostForm, ParseError> = HostForm::classify(input.as_bytes());
            assert_eq!(result, Ok(HostForm::Domain), "input={}", input);
        }
    }
}
