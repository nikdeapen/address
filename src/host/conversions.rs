use crate::{Authority, Domain, DomainRef, Host, HostRef, IPAddress};

impl Host {
    //! Conversions

    /// Converts the host to a host reference.
    pub fn to_ref(&self) -> HostRef<'_> {
        match self {
            Self::Name(domain) => HostRef::Name(domain.to_ref()),
            Self::Address(ip) => HostRef::Address(*ip),
        }
    }

    /// Converts the host to an authority with the `port`.
    pub const fn to_authority(self, port: u16) -> Authority {
        Authority::new(self, port)
    }

    /// Converts the host to an optional domain.
    #[must_use]
    pub fn to_domain(self) -> Option<Domain> {
        if let Self::Name(domain) = self {
            Some(domain)
        } else {
            None
        }
    }

    /// Converts the host to an optional IP address.
    #[must_use]
    pub const fn to_ip(&self) -> Option<IPAddress> {
        if let Self::Address(ip) = self { Some(*ip) } else { None }
    }
}

impl<'a> From<HostRef<'a>> for Host {
    fn from(host: HostRef<'a>) -> Self {
        host.to_host()
    }
}

impl From<Domain> for Host {
    fn from(domain: Domain) -> Self {
        domain.to_host()
    }
}

impl<'a> From<DomainRef<'a>> for Host {
    fn from(domain: DomainRef<'a>) -> Self {
        domain.to_host()
    }
}

impl<A: Into<IPAddress>> From<A> for Host {
    fn from(ip: A) -> Self {
        ip.into().to_host()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, Domain, DomainRef, Host, HostRef, IPAddress, IPv4Address};

    #[test]
    fn host_to_ref() {
        let host: Host = Host::Name(Domain::localhost());
        let result: HostRef = host.to_ref();
        let expected: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        assert_eq!(result, expected);

        let host: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: HostRef = host.to_ref();
        let expected: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn host_to_authority() {
        let host: Host = Domain::localhost().to_host();
        let result: Authority = host.to_authority(80);
        let expected: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        assert_eq!(result, expected);

        let host: Host = IPv4Address::LOCALHOST.to_host();
        let result: Authority = host.to_authority(80);
        let expected: Authority = Authority::new(Host::Address(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn host_to_domain() {
        let host: Host = Domain::localhost().to_host();
        let result: Option<Domain> = host.to_domain();
        let expected: Option<Domain> = Some(Domain::localhost());
        assert_eq!(result, expected);

        let host: Host = IPv4Address::LOCALHOST.to_host();
        let result: Option<Domain> = host.to_domain();
        let expected: Option<Domain> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn host_to_ip() {
        let host: Host = IPv4Address::LOCALHOST.to_host();
        let result: Option<IPAddress> = host.to_ip();
        let expected: Option<IPAddress> = Some(IPv4Address::LOCALHOST.to_ip());
        assert_eq!(result, expected);

        let host: Host = Domain::localhost().to_host();
        let result: Option<IPAddress> = host.to_ip();
        let expected: Option<IPAddress> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn host_from() {
        let expected: Host = Host::Name(Domain::localhost());

        let result: Host = HostRef::Name(DomainRef::LOCALHOST).into();
        assert_eq!(result, expected);

        let result: Host = Domain::localhost().into();
        assert_eq!(result, expected);

        let result: Host = DomainRef::LOCALHOST.into();
        assert_eq!(result, expected);

        let expected: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: Host = IPv4Address::LOCALHOST.into();
        assert_eq!(result, expected);
    }
}
