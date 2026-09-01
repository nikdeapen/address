use crate::{
    Authority, AuthorityRef, Endpoint, EndpointRef, HostRef, SocketAddress, SocketAddressV4,
    SocketAddressV6,
};

impl<'a> AuthorityRef<'a> {
    //! Conversions

    /// Converts the authority reference to an authority.
    pub fn to_authority(self) -> Authority {
        Authority::new(self.host().to_host(), self.port())
    }

    /// Converts the authority reference to an endpoint.
    pub fn to_endpoint(self) -> Result<Endpoint, Self> {
        if let HostRef::Domain(domain) = self.host() {
            Ok(Endpoint::new(domain.to_domain(), self.port()))
        } else {
            Err(self)
        }
    }

    /// Converts the authority reference to an endpoint reference.
    pub const fn to_endpoint_ref(self) -> Result<EndpointRef<'a>, Self> {
        if let HostRef::Domain(domain) = self.host() {
            Ok(EndpointRef::new(domain, self.port()))
        } else {
            Err(self)
        }
    }

    /// Converts the authority reference to a socket address.
    pub const fn to_socket(self) -> Result<SocketAddress, Self> {
        if let HostRef::IPAddress(ip) = self.host() {
            Ok(SocketAddress::new(ip, self.port()))
        } else {
            Err(self)
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
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef,
        IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6,
    };

    #[test]
    fn ref_to_authority() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);
        let result: Authority = authority.to_authority();
        let expected: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_endpoint() {
        let authority: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);
        let result: Result<Endpoint, AuthorityRef> = authority.to_endpoint();
        let expected: Result<Endpoint, AuthorityRef> = Ok(Endpoint::new(Domain::localhost(), 80));
        assert_eq!(result, expected);

        let result: Result<EndpointRef, AuthorityRef> = authority.to_endpoint_ref();
        let expected: Result<EndpointRef, AuthorityRef> =
            Ok(EndpointRef::new(DomainRef::LOCALHOST, 80));
        assert_eq!(result, expected);

        let authority: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);
        let result: Result<Endpoint, AuthorityRef> = authority.to_endpoint();
        let expected: Result<Endpoint, AuthorityRef> = Err(authority);
        assert_eq!(result, expected);

        let result: Result<EndpointRef, AuthorityRef> = authority.to_endpoint_ref();
        let expected: Result<EndpointRef, AuthorityRef> = Err(authority);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_socket() {
        let authority: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);
        let result: Result<SocketAddress, AuthorityRef> = authority.to_socket();
        let expected: Result<SocketAddress, AuthorityRef> =
            Ok(IPv4Address::LOCALHOST.to_ip().to_socket(80));
        assert_eq!(result, expected);

        let authority: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);
        let result: Result<SocketAddress, AuthorityRef> = authority.to_socket();
        let expected: Result<SocketAddress, AuthorityRef> = Err(authority);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_from() {
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Domain(DomainRef::LOCALHOST), 80);

        let owned: Authority = Authority::new(Host::Domain(Domain::localhost()), 80);
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
