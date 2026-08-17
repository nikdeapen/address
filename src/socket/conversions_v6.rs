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

#[cfg(test)]
mod tests {
    use crate::{Authority, AuthorityRef, Host, HostRef, IPAddress, IPv6Address, SocketAddress, SocketAddressV6};

    #[test]
    fn v6_to_socket() {
        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: SocketAddress = socket.to_socket();
        let expected: SocketAddress = SocketAddress::new(IPAddress::V6(IPv6Address::LOCALHOST), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn v6_to_authority() {
        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: Authority = socket.to_authority();
        let expected: Authority = Authority::new(Host::Address(IPAddress::V6(IPv6Address::LOCALHOST)), 80);
        assert_eq!(result, expected);

        let result: AuthorityRef = socket.to_authority_ref();
        let expected: AuthorityRef = AuthorityRef::new(HostRef::Address(IPAddress::V6(IPv6Address::LOCALHOST)), 80);
        assert_eq!(result, expected);
    }
}
