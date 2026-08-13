use crate::{Authority, AuthorityRef, Endpoint, EndpointRef, Host, SocketAddress, SocketAddressV4, SocketAddressV6};

impl Authority {
    //! Conversions

    /// Converts the authority to an authority reference.
    pub fn to_ref(&self) -> AuthorityRef<'_> {
        AuthorityRef::new(self.host(), self.port())
    }

    /// Converts the authority to an optional endpoint.
    #[must_use]
    pub fn to_endpoint(self) -> Option<Endpoint> {
        let (host, port): (Host, u16) = self.into();
        if let Host::Name(domain) = host {
            Some(Endpoint::new(domain, port))
        } else {
            None
        }
    }

    /// Converts the authority to an optional socket address.
    #[must_use]
    pub fn to_socket(&self) -> Option<SocketAddress> {
        self.to_ref().to_socket()
    }
}

impl<'a> From<AuthorityRef<'a>> for Authority {
    fn from(authority: AuthorityRef<'a>) -> Self {
        authority.to_authority()
    }
}

impl From<Endpoint> for Authority {
    fn from(endpoint: Endpoint) -> Self {
        endpoint.to_authority()
    }
}

impl<'a> From<EndpointRef<'a>> for Authority {
    fn from(endpoint: EndpointRef<'a>) -> Self {
        endpoint.to_authority_ref().to_authority()
    }
}

impl From<SocketAddress> for Authority {
    fn from(socket: SocketAddress) -> Self {
        socket.to_authority()
    }
}

impl From<SocketAddressV4> for Authority {
    fn from(socket: SocketAddressV4) -> Self {
        socket.to_socket().to_authority()
    }
}

impl From<SocketAddressV6> for Authority {
    fn from(socket: SocketAddressV6) -> Self {
        socket.to_socket().to_authority()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef, IPv4Address, IPv6Address,
        SocketAddress, SocketAddressV4, SocketAddressV6,
    };

    #[test]
    fn authority_to_ref() {
        let authority: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        let result: AuthorityRef = authority.to_ref();
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_to_endpoint() {
        let authority: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        let result: Option<Endpoint> = authority.to_endpoint();
        let expected: Option<Endpoint> = Some(Endpoint::new(Domain::localhost(), 80));
        assert_eq!(result, expected);

        let authority: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        let result: Option<Endpoint> = authority.to_endpoint();
        let expected: Option<Endpoint> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_to_socket() {
        let authority: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        let result: Option<SocketAddress> = authority.to_socket();
        let expected: Option<SocketAddress> = Some(IPv4Address::LOCALHOST.to_ip().to_socket(80));
        assert_eq!(result, expected);

        let authority: Authority = Authority::new(Host::Name(Domain::localhost()), 80);
        let result: Option<SocketAddress> = authority.to_socket();
        let expected: Option<SocketAddress> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_from() {
        let expected: Authority = Authority::new(Host::Name(Domain::localhost()), 80);

        let result: Authority = AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80).into();
        assert_eq!(result, expected);

        let result: Authority = Endpoint::new(Domain::localhost(), 80).into();
        assert_eq!(result, expected);

        let result: Authority = EndpointRef::new(DomainRef::LOCALHOST, 80).into();
        assert_eq!(result, expected);

        let expected: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);

        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let result: Authority = socket.into();
        assert_eq!(result, expected);

        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let result: Authority = socket.into();
        assert_eq!(result, expected);

        let expected: Authority = Authority::new(IPv6Address::LOCALHOST.to_host(), 80);

        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: Authority = socket.into();
        assert_eq!(result, expected);
    }
}
