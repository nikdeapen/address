use crate::ParseError::InvalidAuthority;
use crate::{AuthorityRef, HostRef, IPv6Address, ParseError, impl_parse_ref};
use crate::{parse_port, strip_brackets};

impl_parse_ref!(
    AuthorityRef,
    "Domain names must already be in lowercase. Use [`Authority`](crate::Authority) to parse mixed-case input."
);

impl<'a> TryFrom<&'a [u8]> for AuthorityRef<'a> {
    type Error = ParseError;

    /// Domain names must already be in lowercase. Use [`Authority`](crate::Authority) to parse mixed-case input.
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
    use crate::ParseError::InvalidHost;
    use crate::{AuthorityRef, DomainRef, HostRef, ParseError};

    #[test]
    fn try_from_str() {
        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("localhost:80");
        let expected: Result<AuthorityRef, ParseError> = Ok(AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80));
        assert_eq!(result, expected);

        let result: Result<AuthorityRef, ParseError> = AuthorityRef::try_from("LocalHost:80");
        let expected: Result<AuthorityRef, ParseError> = Err(InvalidHost);
        assert_eq!(result, expected);
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
