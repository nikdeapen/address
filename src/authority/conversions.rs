use crate::{
    Authority, AuthorityRef, Endpoint, EndpointRef, Host, SocketAddress, SocketAddressV4,
    SocketAddressV6,
};

impl Authority {
    //! Conversions

    /// Converts the authority to an authority reference.
    pub fn to_ref(&self) -> AuthorityRef<'_> {
        AuthorityRef::new(self.host(), self.port())
    }

    /// Converts the authority to an endpoint.
    pub fn to_endpoint(self) -> Result<Endpoint, Self> {
        let (host, port): (Host, u16) = self.into();
        match host.to_domain() {
            Ok(domain) => Ok(Endpoint::new(domain, port)),
            Err(host) => Err(Self::new(host, port)),
        }
    }

    /// Converts the authority to a socket address.
    pub fn to_socket(self) -> Result<SocketAddress, Self> {
        let (host, port): (Host, u16) = self.into();
        match host.to_ip() {
            Ok(ip) => Ok(SocketAddress::new(ip, port)),
            Err(host) => Err(Self::new(host, port)),
        }
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
        endpoint.to_authority()
    }
}

impl From<SocketAddress> for Authority {
    fn from(socket: SocketAddress) -> Self {
        socket.to_authority()
    }
}

impl From<SocketAddressV4> for Authority {
    fn from(socket: SocketAddressV4) -> Self {
        socket.to_authority()
    }
}

impl From<SocketAddressV6> for Authority {
    fn from(socket: SocketAddressV6) -> Self {
        socket.to_authority()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef,
        IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6,
    };

    #[test]
    fn authority_to_ref() {
        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        let result: AuthorityRef = authority.to_ref();
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_to_endpoint() {
        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        let result: Result<Endpoint, Authority> = authority.to_endpoint();
        let expected: Result<Endpoint, Authority> = Ok(Endpoint::new(Domain::localhost(), 80));
        assert_eq!(result, expected);

        let authority: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        let result: Result<Endpoint, Authority> = authority.to_endpoint();
        let expected: Result<Endpoint, Authority> =
            Err(Authority::new(IPv4Address::LOCALHOST.to_host(), 80));
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_to_socket() {
        let authority: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        let result: Result<SocketAddress, Authority> = authority.to_socket();
        let expected: Result<SocketAddress, Authority> =
            Ok(IPv4Address::LOCALHOST.to_ip().to_socket(80));
        assert_eq!(result, expected);

        let authority: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        let result: Result<SocketAddress, Authority> = authority.to_socket();
        let expected: Result<SocketAddress, Authority> =
            Err(Authority::new(Host::Domain(Domain::localhost()), 80));
        assert_eq!(result, expected);
    }

    #[test]
    fn authority_from() {
        let expected: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);

        let result: Authority = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80).into();
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
