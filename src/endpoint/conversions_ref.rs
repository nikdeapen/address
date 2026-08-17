use crate::{AuthorityRef, Endpoint, EndpointRef};

impl<'a> EndpointRef<'a> {
    //! Conversions

    /// Converts the endpoint reference to an endpoint.
    pub fn to_endpoint(self) -> Endpoint {
        Endpoint::new(self.domain().to_domain(), self.port())
    }

    /// Converts the endpoint reference to an authority reference.
    pub const fn to_authority_ref(self) -> AuthorityRef<'a> {
        AuthorityRef::new(self.domain().to_host_ref(), self.port())
    }
}

impl<'a> From<&'a Endpoint> for EndpointRef<'a> {
    fn from(endpoint: &'a Endpoint) -> Self {
        endpoint.to_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, HostRef};

    #[test]
    fn ref_to_endpoint() {
        let endpoint: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        let result: Endpoint = endpoint.to_endpoint();
        let expected: Endpoint = Endpoint::new(Domain::localhost(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_authority() {
        let endpoint: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        let result: AuthorityRef = endpoint.to_authority_ref();
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from() {
        let owned: Endpoint = Endpoint::new(Domain::localhost(), 80);
        let result: EndpointRef = (&owned).into();
        let expected: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        assert_eq!(result, expected);
    }
}
