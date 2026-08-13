use crate::{Domain, DomainRef, Endpoint, Host};

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
        Host::Name(self)
    }
}

impl<'a> From<DomainRef<'a>> for Domain {
    fn from(domain: DomainRef<'a>) -> Self {
        domain.to_domain()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Endpoint, Host};

    #[test]
    fn domain_to_ref() {
        let domain: Domain = Domain::localhost();
        let result: DomainRef = domain.to_ref();
        let expected: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(result, expected);
    }

    #[test]
    fn domain_to_endpoint() {
        let domain: Domain = Domain::localhost();
        let result: Endpoint = domain.to_endpoint(80);
        let expected: Endpoint = Endpoint::new(Domain::localhost(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn domain_to_host() {
        let domain: Domain = Domain::localhost();
        let result: Host = domain.to_host();
        let expected: Host = Host::Name(Domain::localhost());
        assert_eq!(result, expected);
    }

    #[test]
    fn domain_from() {
        let result: Domain = DomainRef::LOCALHOST.into();
        let expected: Domain = Domain::localhost();
        assert_eq!(result, expected);
    }
}
