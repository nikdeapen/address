use crate::ParseError::InvalidAuthority;
use crate::{Authority, AuthorityRef, Domain, HostRef, IPv6Address, ParseError};
use crate::{parse_port, strip_brackets};
use std::str::FromStr;

impl FromStr for Authority {
    type Err = ParseError;

    /// Domain names are normalized to lowercase.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.as_bytes())
    }
}

impl TryFrom<&[u8]> for Authority {
    type Error = ParseError;

    /// Domain names are normalized to lowercase.
    fn try_from(authority: &[u8]) -> Result<Self, Self::Error> {
        match AuthorityRef::try_from(authority) {
            Ok(authority) => Ok(authority.to_authority()),
            Err(error) => {
                // Lowercasing can only rescue a mixed-case domain host; other failures keep the original error.
                if authority.iter().any(|b| b.is_ascii_uppercase())
                    && let Ok((host, port)) = parse_port(authority)
                    && let Ok(domain) = Domain::try_from(host)
                {
                    Ok(domain.to_host().to_authority(port))
                } else {
                    Err(error)
                }
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for AuthorityRef<'a> {
    type Error = ParseError;

    /// Domain names must already be in lowercase.
    fn try_from(authority: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(authority.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for AuthorityRef<'a> {
    type Error = ParseError;

    /// Domain names must already be in lowercase.
    fn try_from(authority: &'a [u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(authority)?;
        if let Some(s) = strip_brackets(s) {
            let host: HostRef = IPv6Address::parse(s)?.to_host_ref();
            Ok(host.to_authority_ref(port))
        } else {
            let host: HostRef = HostRef::try_from(s)?;
            if let HostRef::Address(ip) = host
                && ip.is_v6()
            {
                return Err(InvalidAuthority);
            }
            Ok(host.to_authority_ref(port))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidAuthority, InvalidHost, InvalidPort};
    use crate::{Authority, AuthorityRef, Domain, DomainRef, HostRef, IPv4Address, IPv6Address, ParseError};
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let test_cases: &[(&str, Result<Authority, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("localhost:", Err(InvalidPort)),
            ("localhost:xx", Err(InvalidPort)),
            (":80", Err(InvalidHost)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_host().to_authority(80))),
            ("::1:80", Err(InvalidAuthority)),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_host().to_authority(80))),
            (
                "[::FFFF]:80",
                Ok(IPv6Address::from([0, 0, 0, 0, 0, 0, 0, 0xFFFF])
                    .to_host()
                    .to_authority(80)),
            ),
            ("localhost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("LocalHost:80", Ok(Domain::localhost().to_host().to_authority(80))),
            ("Local_Host:80", Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<Authority, ParseError> = Authority::from_str(input);
            assert_eq!(result, *expected, "input={}", input);
        }
    }

    #[test]
    fn try_from_slice() {
        let test_cases: &[(&[u8], Result<AuthorityRef, ParseError>)] = &[
            (
                "localhost:80".as_bytes(),
                Ok(AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80)),
            ),
            ("LocalHost:80".as_bytes(), Err(InvalidHost)),
            (b"\xFF:80".as_slice(), Err(InvalidHost)),
            ("ü:80".as_bytes(), Err(InvalidHost)),
        ];

        for (input, expected) in test_cases {
            let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from(*input);
            assert_eq!(result, *expected, "input={:?}", input);
        }
    }
}
