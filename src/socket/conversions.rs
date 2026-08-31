use crate::{Authority, AuthorityRef, IPAddress, SocketAddress, SocketAddressV4, SocketAddressV6};

impl SocketAddress {
    //! Conversions

    /// Converts the socket address to an IPv4 socket address.
    pub const fn to_v4(self) -> Result<SocketAddressV4, Self> {
        if let IPAddress::V4(v4) = self.ip() {
            Ok(SocketAddressV4::new(v4, self.port()))
        } else {
            Err(self)
        }
    }

    /// Converts the socket address to an IPv6 socket address.
    pub const fn to_v6(self) -> Result<SocketAddressV6, Self> {
        if let IPAddress::V6(v6) = self.ip() {
            Ok(SocketAddressV6::new(v6, self.port()))
        } else {
            Err(self)
        }
    }

    /// Converts the socket address to an authority.
    pub const fn to_authority(self) -> Authority {
        Authority::new(self.ip().to_host(), self.port())
    }

    /// Converts the socket address to an authority reference.
    pub const fn to_authority_ref(self) -> AuthorityRef<'static> {
        AuthorityRef::new(self.ip().to_host_ref(), self.port())
    }
}

impl From<SocketAddressV4> for SocketAddress {
    fn from(socket: SocketAddressV4) -> Self {
        socket.to_socket()
    }
}

impl From<SocketAddressV6> for SocketAddress {
    fn from(socket: SocketAddressV6) -> Self {
        socket.to_socket()
    }
}

impl TryFrom<Authority> for SocketAddress {
    type Error = Authority;

    fn try_from(authority: Authority) -> Result<Self, Self::Error> {
        authority.to_socket()
    }
}

impl<'a> TryFrom<AuthorityRef<'a>> for SocketAddress {
    type Error = AuthorityRef<'a>;

    fn try_from(authority: AuthorityRef<'a>) -> Result<Self, Self::Error> {
        authority.to_socket()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Host, HostRef, IPAddress, IPv4Address,
        IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6,
    };

    #[test]
    fn socket_to_v4() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let result: Result<SocketAddressV4, SocketAddress> = socket.to_v4();
        let expected: Result<SocketAddressV4, SocketAddress> =
            Ok(SocketAddressV4::new(IPv4Address::LOCALHOST, 80));
        assert_eq!(result, expected);

        let socket: SocketAddress = IPv6Address::LOCALHOST.to_ip().to_socket(80);
        let result: Result<SocketAddressV4, SocketAddress> = socket.to_v4();
        let expected: Result<SocketAddressV4, SocketAddress> = Err(socket);
        assert_eq!(result, expected);
    }

    #[test]
    fn socket_to_v6() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let result: Result<SocketAddressV6, SocketAddress> = socket.to_v6();
        let expected: Result<SocketAddressV6, SocketAddress> = Err(socket);
        assert_eq!(result, expected);

        let socket: SocketAddress = IPv6Address::LOCALHOST.to_ip().to_socket(80);
        let result: Result<SocketAddressV6, SocketAddress> = socket.to_v6();
        let expected: Result<SocketAddressV6, SocketAddress> =
            Ok(SocketAddressV6::new(IPv6Address::LOCALHOST, 80));
        assert_eq!(result, expected);
    }

    #[test]
    fn socket_to_authority() {
        let socket: SocketAddress = SocketAddress::new(IPAddress::V4(IPv4Address::LOCALHOST), 80);
        let result: Authority = socket.to_authority();
        let expected: Authority =
            Authority::new(Host::IP(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = socket.to_authority_ref();
        let expected: AuthorityRef =
            AuthorityRef::new(HostRef::IP(IPAddress::V4(IPv4Address::LOCALHOST)), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn socket_try_from() {
        let expected: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);

        let authority: Authority = IPv4Address::LOCALHOST.to_host().to_authority(80);
        assert_eq!(SocketAddress::try_from(authority), Ok(expected));

        let authority: Authority = Domain::localhost().to_host().to_authority(80);
        assert_eq!(
            SocketAddress::try_from(authority.clone()),
            Err(authority.clone())
        );

        let authority: AuthorityRef = IPv4Address::LOCALHOST.to_host_ref().to_authority_ref(80);
        assert_eq!(SocketAddress::try_from(authority), Ok(expected));

        let authority: AuthorityRef = DomainRef::LOCALHOST.to_host_ref().to_authority_ref(80);
        assert_eq!(SocketAddress::try_from(authority), Err(authority));
    }

    #[test]
    fn socket_from() {
        let expected: SocketAddress = SocketAddress::new(IPAddress::V4(IPv4Address::LOCALHOST), 80);
        let result: SocketAddress = IPv4Address::LOCALHOST.to_socket(80).into();
        assert_eq!(result, expected);

        let expected: SocketAddress = SocketAddress::new(IPAddress::V6(IPv6Address::LOCALHOST), 80);
        let result: SocketAddress = IPv6Address::LOCALHOST.to_socket(80).into();
        assert_eq!(result, expected);
    }
}
