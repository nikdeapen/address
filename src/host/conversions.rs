use crate::{Authority, Domain, DomainRef, Host, HostRef, IPAddress};

impl Host {
    //! Conversions

    /// Converts the host to a host reference.
    pub fn to_ref(&self) -> HostRef<'_> {
        match self {
            Self::Domain(domain) => HostRef::Domain(domain.to_ref()),
            Self::IP(ip) => HostRef::IP(*ip),
        }
    }

    /// Converts the host to an authority with the `port`.
    pub const fn to_authority(self, port: u16) -> Authority {
        Authority::new(self, port)
    }

    /// Converts the host to a domain.
    pub fn to_domain(self) -> Result<Domain, Self> {
        match self {
            Self::Domain(domain) => Ok(domain),
            host => Err(host),
        }
    }

    /// Converts the host to an optional IP address.
    #[must_use]
    pub const fn to_ip(&self) -> Option<IPAddress> {
        if let Self::IP(ip) = self {
            Some(*ip)
        } else {
            None
        }
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
        let host: Host = Host::Domain(Domain::localhost());
        let result: HostRef = host.to_ref();
        let expected: HostRef = HostRef::Domain(DomainRef::LOCALHOST);
        assert_eq!(result, expected);

        let host: Host = Host::IP(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: HostRef = host.to_ref();
        let expected: HostRef = HostRef::IP(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }

    #[test]
    fn host_to_authority() {
        let host: Host = Domain::localhost().to_host();
        let result: Authority = host.to_authority(80);
        let expected: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(result, expected);

        let host: Host = IPv4Address::LOCALHOST.to_host();
        let result: Authority = host.to_authority(80);
        let expected: Authority =
            Authority::new(Host::IP(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn host_to_domain() {
        let host: Host = Domain::localhost().to_host();
        let result: Result<Domain, Host> = host.to_domain();
        let expected: Result<Domain, Host> = Ok(Domain::localhost());
        assert_eq!(result, expected);

        let host: Host = IPv4Address::LOCALHOST.to_host();
        let result: Result<Domain, Host> = host.to_domain();
        let expected: Result<Domain, Host> = Err(IPv4Address::LOCALHOST.to_host());
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
        let expected: Host = Host::Domain(Domain::localhost());

        let result: Host = HostRef::Domain(DomainRef::LOCALHOST).into();
        assert_eq!(result, expected);

        let result: Host = Domain::localhost().into();
        assert_eq!(result, expected);

        let result: Host = DomainRef::LOCALHOST.into();
        assert_eq!(result, expected);

        let expected: Host = Host::IP(IPAddress::V4(IPv4Address::LOCALHOST));
        let result: Host = IPv4Address::LOCALHOST.into();
        assert_eq!(result, expected);
    }
}
