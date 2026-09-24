use crate::{Domain, DomainRef, Endpoint, Host, HostRef};

impl Domain {
    //! Conversions

    /// Converts the domain to a domain reference.
    pub fn to_ref(&self) -> DomainRef<'_> {
        unsafe { DomainRef::new_unchecked(self.name()) }
    }

    /// Converts the domain to an endpoint with the `port`.
    pub const fn to_endpoint(self, port: u16) -> Endpoint {
        Endpoint::new(self, port)
    }

    /// Converts the domain to a host.
    pub const fn to_host(self) -> Host {
        Host::Domain(self)
    }
}

impl<'a> From<DomainRef<'a>> for Domain {
    fn from(domain: DomainRef<'a>) -> Self {
        domain.to_domain()
    }
}

impl TryFrom<Host> for Domain {
    type Error = Host;

    fn try_from(host: Host) -> Result<Self, Self::Error> {
        host.to_domain()
    }
}

impl<'a> TryFrom<HostRef<'a>> for Domain {
    type Error = HostRef<'a>;

    fn try_from(host: HostRef<'a>) -> Result<Self, Self::Error> {
        host.to_domain()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Endpoint, Host, HostRef, IPv4Address};

    #[test]
    fn to_ref() {
        let domain: Domain = Domain::localhost();
        let result: DomainRef = domain.to_ref();
        let expected: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(result, expected);
    }

    #[test]
    fn to_endpoint() {
        let domain: Domain = Domain::localhost();
        let result: Endpoint = domain.to_endpoint(80);
        let expected: Endpoint = Endpoint::new(Domain::localhost(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn to_host() {
        let domain: Domain = Domain::localhost();
        let result: Host = domain.to_host();
        let expected: Host = Host::Domain(Domain::localhost());
        assert_eq!(result, expected);
    }

    #[test]
    fn from() {
        let result: Domain = DomainRef::LOCALHOST.into();
        let expected: &str = "localhost";
        assert_eq!(result.name(), expected);
    }

    #[test]
    fn try_from() {
        let host: Host = Domain::localhost().to_host();
        assert_eq!(Domain::try_from(host), Ok(Domain::localhost()));

        let host: Host = IPv4Address::LOCALHOST.to_host();
        assert_eq!(Domain::try_from(host.clone()), Err(host));

        let host: HostRef = DomainRef::LOCALHOST.to_host_ref();
        assert_eq!(Domain::try_from(host), Ok(Domain::localhost()));

        let host: HostRef = IPv4Address::LOCALHOST.to_host_ref();
        assert_eq!(Domain::try_from(host), Err(host));
    }
}
