use crate::{Authority, AuthorityRef, SocketAddress, SocketAddressV6};

impl SocketAddressV6 {
    //! Conversions

    /// Converts the IPv6 socket address to a socket address.
    pub const fn to_socket(self) -> SocketAddress {
        SocketAddress::new(self.ip().to_ip(), self.port())
    }

    /// Converts the IPv6 socket address to an authority.
    pub const fn to_authority(self) -> Authority {
        Authority::new(self.ip().to_host(), self.port())
    }

    /// Converts the IPv6 socket address to an authority reference.
    pub const fn to_authority_ref(self) -> AuthorityRef<'static> {
        AuthorityRef::new(self.ip().to_host_ref(), self.port())
    }
}

impl TryFrom<SocketAddress> for SocketAddressV6 {
    type Error = SocketAddress;

    fn try_from(socket: SocketAddress) -> Result<Self, Self::Error> {
        socket.to_v6()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Authority, AuthorityRef, Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddress,
        SocketAddressV6,
    };

    #[test]
    fn to_socket() {
        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: SocketAddress = socket.to_socket();
        let expected: SocketAddress = SocketAddress::new(IPAddress::V6(IPv6Address::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn to_authority() {
        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: Authority = socket.to_authority();
        let expected: Authority =
            Authority::new(Host::IPAddress(IPAddress::V6(IPv6Address::LOCALHOST)), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = socket.to_authority_ref();
        let expected: AuthorityRef = AuthorityRef::new(
            HostRef::IPAddress(IPAddress::V6(IPv6Address::LOCALHOST)),
            80,
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn try_from() {
        let socket: SocketAddress = IPv6Address::LOCALHOST.to_ip().to_socket(80);
        let expected: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        assert_eq!(SocketAddressV6::try_from(socket), Ok(expected));

        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        assert_eq!(SocketAddressV6::try_from(socket), Err(socket));
    }
}
