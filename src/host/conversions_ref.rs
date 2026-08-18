use crate::{Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPAddress};

impl<'a> HostRef<'a> {
    //! Conversions

    /// Converts the host reference to a host.
    pub fn to_host(self) -> Host {
        match self {
            Self::Name(domain) => Host::Name(domain.to_domain()),
            Self::Address(ip) => Host::Address(ip),
        }
    }

    /// Converts the host reference to an authority with the `port`.
    pub fn to_authority(self, port: u16) -> Authority {
        Authority::new(self.to_host(), port)
    }

    /// Converts the host reference to an authority reference with the `port`.
    pub const fn to_authority_ref(self, port: u16) -> AuthorityRef<'a> {
        AuthorityRef::new(self, port)
    }

    /// Converts the host reference to an optional domain.
    #[must_use]
    pub fn to_domain(self) -> Option<Domain> {
        if let Self::Name(domain) = self {
            Some(domain.to_domain())
        } else {
            None
        }
    }

    /// Converts the host reference to an optional domain reference.
    #[must_use]
    pub const fn to_domain_ref(self) -> Option<DomainRef<'a>> {
        if let Self::Name(domain) = self {
            Some(domain)
        } else {
            None
        }
    }

    /// Converts the host reference to an optional IP address.
    #[must_use]
    pub const fn to_ip(self) -> Option<IPAddress> {
        if let Self::Address(ip) = self { Some(ip) } else { None }
    }
}

impl<'a> From<&'a Host> for HostRef<'a> {
    fn from(host: &'a Host) -> Self {
        host.to_ref()
    }
}

impl<'a> From<DomainRef<'a>> for HostRef<'a> {
    fn from(domain: DomainRef<'a>) -> Self {
        domain.to_host_ref()
    }
}

impl<'a, A: Into<IPAddress>> From<A> for HostRef<'a> {
    fn from(ip: A) -> Self {
        ip.into().to_host_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPAddress, IPv4Address};

    #[test]
    fn ref_to_host() {
        let host: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        let result: Host = host.to_host();
        let expected: Host = Host::Name(Domain::localhost());
        assert_eq!(result, expected);

        let host: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: Host = host.to_host();
        let expected: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_authority() {
        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        let result: Authority = host.to_authority(80);
        let expected: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = host.to_authority_ref(80);
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        assert_eq!(result, expected);

        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        let result: Authority = host.to_authority(80);
        let expected: Authority = Authority::new(Host::Address(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = host.to_authority_ref(80);
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_domain() {
        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        let result: Option<Domain> = host.to_domain();
        let expected: Option<Domain> = Some(Domain::localhost());
        assert_eq!(result, expected);

        let result: Option<DomainRef> = host.to_domain_ref();
        let expected: Option<DomainRef> = Some(DomainRef::LOCALHOST);
        assert_eq!(result, expected);

        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        let result: Option<Domain> = host.to_domain();
        let expected: Option<Domain> = None;
        assert_eq!(result, expected);

        let result: Option<DomainRef> = host.to_domain_ref();
        let expected: Option<DomainRef> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_ip() {
        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        let result: Option<IPAddress> = host.to_ip();
        let expected: Option<IPAddress> = Some(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(result, expected);

        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        let result: Option<IPAddress> = host.to_ip();
        let expected: Option<IPAddress> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from() {
        let expected: HostRef = HostRef::Name(DomainRef::LOCALHOST);

        let owned: Host = Host::Name(Domain::localhost());
        let result: HostRef = (&owned).into();
        assert_eq!(result, expected);

        let result: HostRef = DomainRef::LOCALHOST.into();
        assert_eq!(result, expected);

        let expected: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: HostRef = IPv4Address::LOCALHOST.into();
        assert_eq!(result, expected);
    }
}
