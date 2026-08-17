use crate::{Authority, Domain, Endpoint, EndpointRef};

impl Endpoint {
    //! Conversions

    /// Converts the endpoint to an endpoint reference.
    pub fn to_ref(&self) -> EndpointRef<'_> {
        EndpointRef::new(self.domain(), self.port())
    }

    /// Converts the endpoint to an authority.
    pub fn to_authority(self) -> Authority {
        let (domain, port): (Domain, u16) = self.into();
        Authority::new(domain.to_host(), port)
    }
}

impl<'a> From<EndpointRef<'a>> for Endpoint {
    fn from(endpoint: EndpointRef<'a>) -> Self {
        endpoint.to_endpoint()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, Domain, DomainRef, Endpoint, EndpointRef};

    #[test]
    fn endpoint_to_ref() {
        let endpoint: Endpoint = Endpoint::new(Domain::localhost(), 80);
        let result: EndpointRef = endpoint.to_ref();
        let expected: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn endpoint_to_authority() {
        let endpoint: Endpoint = Endpoint::new(Domain::localhost(), 80);
        let result: Authority = endpoint.to_authority();
        let expected: Authority = Authority::new(Domain::localhost().to_host(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn endpoint_from() {
        let result: Endpoint = EndpointRef::new(DomainRef::LOCALHOST, 80).into();
        let expected: Endpoint = Endpoint::new(Domain::localhost(), 80);
        assert_eq!(result, expected);
    }
}
