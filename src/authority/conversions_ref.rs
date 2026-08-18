use crate::{Authority, AuthorityRef, Endpoint, EndpointRef, HostRef, SocketAddress, SocketAddressV4, SocketAddressV6};

impl<'a> AuthorityRef<'a> {
    //! Conversions

    /// Converts the authority reference to an authority.
    pub fn to_authority(self) -> Authority {
        Authority::new(self.host().to_host(), self.port())
    }

    /// Converts the authority reference to an optional endpoint.
    #[must_use]
    pub fn to_endpoint(self) -> Option<Endpoint> {
        if let HostRef::Name(domain) = self.host() {
            Some(Endpoint::new(domain.to_domain(), self.port()))
        } else {
            None
        }
    }

    /// Converts the authority reference to an optional endpoint reference.
    #[must_use]
    pub const fn to_endpoint_ref(self) -> Option<EndpointRef<'a>> {
        if let HostRef::Name(domain) = self.host() {
            Some(EndpointRef::new(domain, self.port()))
        } else {
            None
        }
    }

    /// Converts the authority reference to an optional socket address.
    #[must_use]
    pub const fn to_socket(self) -> Option<SocketAddress> {
        if let HostRef::Address(ip) = self.host() {
            Some(SocketAddress::new(ip, self.port()))
        } else {
            None
        }
    }
}

impl<'a> From<&'a Authority> for AuthorityRef<'a> {
    fn from(authority: &'a Authority) -> Self {
        authority.to_ref()
    }
}

impl<'a> From<EndpointRef<'a>> for AuthorityRef<'a> {
    fn from(endpoint: EndpointRef<'a>) -> Self {
        endpoint.to_authority_ref()
    }
}

impl<'a> From<SocketAddress> for AuthorityRef<'a> {
    fn from(socket: SocketAddress) -> Self {
        socket.to_authority_ref()
    }
}

impl<'a> From<SocketAddressV4> for AuthorityRef<'a> {
    fn from(socket: SocketAddressV4) -> Self {
        socket.to_authority_ref()
    }
}

impl<'a> From<SocketAddressV6> for AuthorityRef<'a> {
    fn from(socket: SocketAddressV6) -> Self {
        socket.to_authority_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef, IPv4Address, IPv6Address,
        SocketAddress, SocketAddressV4, SocketAddressV6,
    };

    #[test]
    fn ref_to_authority() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        let result: Authority = authority.to_authority();
        let expected: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_endpoint() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        let result: Option<Endpoint> = authority.to_endpoint();
        let expected: Option<Endpoint> = Some(Endpoint::new(Domain::localhost(), 80));
        assert_eq!(result, expected);

        let result: Option<EndpointRef> = authority.to_endpoint_ref();
        let expected: Option<EndpointRef> = Some(EndpointRef::new(DomainRef::LOCALHOST, 80));
        assert_eq!(result, expected);

        let authority: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);
        let result: Option<Endpoint> = authority.to_endpoint();
        let expected: Option<Endpoint> = None;
        assert_eq!(result, expected);

        let result: Option<EndpointRef> = authority.to_endpoint_ref();
        let expected: Option<EndpointRef> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_socket() {
        let authority: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);
        let result: Option<SocketAddress> = authority.to_socket();
        let expected: Option<SocketAddress> = Some(IPv4Address::LOCALHOST.to_ip().to_socket(80));
        assert_eq!(result, expected);

        let authority: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        let result: Option<SocketAddress> = authority.to_socket();
        let expected: Option<SocketAddress> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from() {
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);

        let owned: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        let result: AuthorityRef = (&owned).into();
        assert_eq!(result, expected);

        let result: AuthorityRef = EndpointRef::new(DomainRef::LOCALHOST, 80).into();
        assert_eq!(result, expected);

        let expected: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);

        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);

        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);

        let expected: AuthorityRef = AuthorityRef::new(IPv6Address::LOCALHOST.to_host_ref(), 80);

        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);
    }
}
