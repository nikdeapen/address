use crate::{Endpoint, EndpointRef};

impl Endpoint {
    //! Conversions

    /// Converts the endpoint to an endpoint reference.
    pub fn to_ref(&self) -> EndpointRef {
        EndpointRef::new(self.domain(), self.port())
    }
}

impl<'a> EndpointRef<'a> {
    //! Conversions

    /// Converts the endpoint reference to an endpoint.
    pub fn to_endpoint(&self) -> Endpoint {
        Endpoint::new(self.domain().to_domain(), self.port())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Endpoint, EndpointRef};

    #[test]
    fn endpoint_to_ref() {
        let endpoint: Endpoint = Endpoint::new(Domain::localhost(), 80);

        let result: EndpointRef = endpoint.to_ref();
        let expected: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_endpoint() {
        let endpoint: EndpointRef = EndpointRef::new(DomainRef::LOCALHOST, 80);

        let result: Endpoint = endpoint.to_endpoint();
        let expected: Endpoint = Endpoint::new(Domain::localhost(), 80);
        assert_eq!(result, expected);
    }
}
