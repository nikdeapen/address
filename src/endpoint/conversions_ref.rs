use crate::{Authority, AuthorityRef, Endpoint, EndpointRef};

impl<'a> EndpointRef<'a> {
    //! Conversions

    /// Converts the endpoint reference to an endpoint.
    pub fn to_endpoint(self) -> Endpoint {
        Endpoint::new(self.domain().to_domain(), self.port())
    }

    /// Converts the endpoint reference to an authority.
    pub fn to_authority(self) -> Authority {
        Authority::new(self.domain().to_host(), self.port())
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

impl<'a> TryFrom<AuthorityRef<'a>> for EndpointRef<'a> {
    type Error = AuthorityRef<'a>;

    fn try_from(authority: AuthorityRef<'a>) -> Result<Self, Self::Error> {
        authority.to_endpoint_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef,
        IPv4Address,
    };

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
        let result: Authority = endpoint.to_authority();
        let expected: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = endpoint.to_authority_ref();
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_try_from() {
        let authority: AuthorityRef = DomainRef::LOCALHOST.to_host_ref().to_authority_ref(80);
        let expected: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        assert_eq!(EndpointRef::try_from(authority), Ok(expected));

        let authority: AuthorityRef = IPv4Address::LOCALHOST.to_host_ref().to_authority_ref(80);
        assert_eq!(EndpointRef::try_from(authority), Err(authority));
    }

    #[test]
    fn ref_from() {
        let owned: Endpoint = Endpoint::new(Domain::localhost(), 80);
        let result: EndpointRef = (&owned).into();
        let expected: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        assert_eq!(result, expected);
    }
}
