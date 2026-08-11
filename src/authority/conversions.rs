use crate::{
    Authority, AuthorityRef, Endpoint, EndpointRef, Host, HostRef, SocketAddress, SocketAddressV4, SocketAddressV6,
};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

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

impl From<SocketAddr> for Authority {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(socket: SocketAddr) -> Self {
        SocketAddress::from(socket).to_authority()
    }
}

impl From<SocketAddrV4> for Authority {
    fn from(socket: SocketAddrV4) -> Self {
        SocketAddressV4::from(socket).to_socket().to_authority()
    }
}

impl From<SocketAddrV6> for Authority {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(socket: SocketAddrV6) -> Self {
        SocketAddressV6::from(socket).to_socket().to_authority()
    }
}

impl<'a> AuthorityRef<'a> {
    //! Conversions

    /// Converts the authority reference to an authority.
    pub fn to_authority(self) -> Authority {
        Authority::new(self.host().to_host(), self.port())
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
        Self::new(HostRef::Address(socket.ip()), socket.port())
    }
}

impl<'a> From<SocketAddressV4> for AuthorityRef<'a> {
    fn from(socket: SocketAddressV4) -> Self {
        Self::from(socket.to_socket())
    }
}

impl<'a> From<SocketAddressV6> for AuthorityRef<'a> {
    fn from(socket: SocketAddressV6) -> Self {
        Self::from(socket.to_socket())
    }
}

impl<'a> From<SocketAddr> for AuthorityRef<'a> {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(socket: SocketAddr) -> Self {
        Self::from(SocketAddress::from(socket))
    }
}

impl<'a> From<SocketAddrV4> for AuthorityRef<'a> {
    fn from(socket: SocketAddrV4) -> Self {
        Self::from(SocketAddressV4::from(socket))
    }
}

impl<'a> From<SocketAddrV6> for AuthorityRef<'a> {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(socket: SocketAddrV6) -> Self {
        Self::from(SocketAddressV6::from(socket))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef, IPv4Address, IPv6Address,
        SocketAddress, SocketAddressV4, SocketAddressV6,
    };
    use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

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

        let expected: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);

        let socket: SocketAddr = IPv4Address::LOCALHOST.to_ip().to_socket(80).to_std();
        let result: Authority = socket.into();
        assert_eq!(result, expected);

        let socket: SocketAddrV4 = IPv4Address::LOCALHOST.to_socket(80).into();
        let result: Authority = socket.into();
        assert_eq!(result, expected);

        let expected: Authority = Authority::new(IPv6Address::LOCALHOST.to_host(), 80);

        let socket: SocketAddrV6 = IPv6Address::LOCALHOST.to_socket(80).into();
        let result: Authority = socket.into();
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_authority() {
        let host: HostRef = HostRef::Name(DomainRef::LOCALHOST);
        let authority: AuthorityRef = AuthorityRef::new(host, 80);

        let result: Authority = authority.to_authority();
        let expected: Authority = Authority::new(host.to_host(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_endpoint() {
        let test_cases: &[(AuthorityRef, Option<EndpointRef>)] = &[
            (
                AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80),
                Some(EndpointRef::new(DomainRef::LOCALHOST, 80)),
            ),
            (AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80), None),
        ];

        for (authority, expected) in test_cases {
            let result: Option<EndpointRef> = authority.to_endpoint_ref();
            assert_eq!(result, *expected, "authority={:?}", authority);
        }
    }

    #[test]
    fn ref_to_socket() {
        let test_cases: &[(AuthorityRef, Option<SocketAddress>)] = &[
            (
                AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80),
                Some(IPv4Address::LOCALHOST.to_ip().to_socket(80)),
            ),
            (AuthorityRef::new(HostRef::Name(DomainRef::LOCALHOST), 80), None),
        ];

        for (authority, expected) in test_cases {
            let result: Option<SocketAddress> = authority.to_socket();
            assert_eq!(result, *expected, "authority={:?}", authority);
        }
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

        let expected: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);

        let socket: SocketAddr = IPv4Address::LOCALHOST.to_ip().to_socket(80).to_std();
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);

        let socket: SocketAddrV4 = IPv4Address::LOCALHOST.to_socket(80).into();
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);

        let expected: AuthorityRef = AuthorityRef::new(IPv6Address::LOCALHOST.to_host_ref(), 80);

        let socket: SocketAddrV6 = IPv6Address::LOCALHOST.to_socket(80).into();
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);
    }
}
