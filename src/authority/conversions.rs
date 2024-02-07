use crate::{Authority, AuthorityRef, Host, HostRef};

impl Authority {
    /// Converts the authority to an authority reference.
    pub fn to_ref(&self) -> AuthorityRef {
        AuthorityRef::new(self.host(), self.port())
    }
}

impl<H: Into<Host>> From<(H, u16)> for Authority {
    fn from(tuple: (H, u16)) -> Self {
        Self::new(tuple.0.into(), tuple.1)
    }
}

impl<'a> AuthorityRef<'a> {
    /// Converts the authority reference to an authority.
    pub fn to_authority(&self) -> Authority {
        Authority::new(self.host().to_host(), self.port())
    }
}

impl<'a, D: Into<HostRef<'a>>> From<(D, u16)> for AuthorityRef<'a> {
    fn from(tuple: (D, u16)) -> Self {
        Self::new(tuple.0.into(), tuple.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, AuthorityRef, Domain, DomainRef, HostRef};

    #[test]
    fn authority_to_ref() {
        let authority: Authority = Domain::localhost().to_host().to_authority(80);
        let result: AuthorityRef = authority.to_ref();
        let expected: AuthorityRef = AuthorityRef::new(DomainRef::LOCALHOST.to_host(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_from_tuple() {
        let authority: Authority = (Domain::localhost(), 80).into();
        assert_eq!(authority.host(), DomainRef::LOCALHOST.to_host());
        assert_eq!(authority.port(), 80);
    }

    #[test]
    fn ref_to_authority() {
        let host: HostRef = DomainRef::LOCALHOST.to_host();
        let authority: AuthorityRef = host.to_authority(80);
        let result: Authority = authority.to_authority();
        let expected: Authority = Domain::localhost().to_host().to_authority(80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from_tuple() {
        let authority: AuthorityRef = (DomainRef::LOCALHOST, 80).into();
        assert_eq!(authority.host(), DomainRef::LOCALHOST.to_host());
        assert_eq!(authority.port(), 80);
    }
}
