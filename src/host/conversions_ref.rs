use crate::{Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPAddress};

impl<'a> HostRef<'a> {
    //! Conversions

    /// Converts the host reference to a host.
    pub fn to_host(self) -> Host {
        match self {
            Self::Domain(domain) => Host::Domain(domain.to_domain()),
            Self::IPAddress(ip) => Host::IPAddress(ip),
        }
    }

    /// Converts the host reference to a domain.
    pub fn to_domain(self) -> Result<Domain, Self> {
        if let Self::Domain(domain) = self {
            Ok(domain.to_domain())
        } else {
            Err(self)
        }
    }

    /// Converts the host reference to a domain reference.
    pub const fn to_domain_ref(self) -> Result<DomainRef<'a>, Self> {
        if let Self::Domain(domain) = self {
            Ok(domain)
        } else {
            Err(self)
        }
    }

    /// Converts the host reference to an IP address.
    pub const fn to_ip(self) -> Result<IPAddress, Self> {
        if let Self::IPAddress(ip) = self {
            Ok(ip)
        } else {
            Err(self)
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
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPAddress, IPv4Address,
    };

    #[test]
    fn ref_to_host() {
        let host: HostRef = HostRef::Domain(DomainRef::LOCALHOST);
        let result: Host = host.to_host();
        let expected: Host = Host::Domain(Domain::localhost());
        assert_eq!(result, expected);

        let host: HostRef = HostRef::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: Host = host.to_host();
        let expected: Host = Host::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_domain() {
        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        let result: Result<Domain, HostRef> = host.to_domain();
        let expected: Result<Domain, HostRef> = Ok(Domain::localhost());
        assert_eq!(result, expected);

        let result: Result<DomainRef, HostRef> = host.to_domain_ref();
        let expected: Result<DomainRef, HostRef> = Ok(DomainRef::LOCALHOST);
        assert_eq!(result, expected);

        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        let result: Result<Domain, HostRef> = host.to_domain();
        let expected: Result<Domain, HostRef> = Err(host);
        assert_eq!(result, expected);

        let result: Result<DomainRef, HostRef> = host.to_domain_ref();
        let expected: Result<DomainRef, HostRef> = Err(host);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_ip() {
        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        let result: Result<IPAddress, HostRef> = host.to_ip();
        let expected: Result<IPAddress, HostRef> = Ok(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(result, expected);

        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        let result: Result<IPAddress, HostRef> = host.to_ip();
        let expected: Result<IPAddress, HostRef> = Err(host);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_authority() {
        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        let result: Authority = host.to_authority(80);
        let expected: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = host.to_authority_ref(80);
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);
        assert_eq!(result, expected);

        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        let result: Authority = host.to_authority(80);
        let expected: Authority =
            Authority::new(Host::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = host.to_authority_ref(80);
        let expected: AuthorityRef = AuthorityRef::new(
            HostRef::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST)),
            80,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from() {
        let expected: HostRef = HostRef::Domain(DomainRef::LOCALHOST);

        let owned: Host = Host::Domain(Domain::localhost());
        let result: HostRef = (&owned).into();
        assert_eq!(result, expected);

        let result: HostRef = DomainRef::LOCALHOST.into();
        assert_eq!(result, expected);

        let expected: HostRef = HostRef::IPAddress(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: HostRef = IPv4Address::LOCALHOST.into();
        assert_eq!(result, expected);
    }
}
