use crate::{Domain, DomainRef, EndpointRef, HostRef};

impl<'a> DomainRef<'a> {
    //! Conversions

    /// Converts the domain reference to a domain.
    pub fn to_domain(self) -> Domain {
        unsafe { Domain::new_unchecked(self.name()) }
    }

    /// Converts the domain reference to an endpoint reference with the `port`.
    pub const fn to_endpoint_ref(self, port: u16) -> EndpointRef<'a> {
        EndpointRef::new(self, port)
    }

    /// Converts the domain reference to a host reference.
    pub const fn to_host_ref(self) -> HostRef<'a> {
        HostRef::Name(self)
    }
}

impl<'a> From<&'a Domain> for DomainRef<'a> {
    fn from(domain: &'a Domain) -> Self {
        domain.to_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, EndpointRef, HostRef};

    #[test]
    fn ref_to_domain() {
        let domain: DomainRef = DomainRef::LOCALHOST;
        let result: Domain = domain.to_domain();
        let expected: Domain = Domain::localhost();
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_endpoint() {
        let domain: DomainRef = DomainRef::LOCALHOST;
        let result: EndpointRef = domain.to_endpoint_ref(80);
        let expected: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_host() {
        let domain: DomainRef = DomainRef::LOCALHOST;
        let result: HostRef = domain.to_host_ref();
        let expected: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from() {
        let owned: Domain = Domain::localhost();
        let result: DomainRef = (&owned).into();
        let expected: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(result, expected);
    }
}
