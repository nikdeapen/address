use crate::{Authority, IPAddress, SocketAddress, SocketAddressV4, SocketAddressV6};

impl SocketAddress {
    //! Conversions

    /// Converts the socket address to an optional IPv4 socket address.
    #[must_use]
    pub const fn to_v4(self) -> Option<SocketAddressV4> {
        if let IPAddress::V4(v4) = self.ip() {
            Some(SocketAddressV4::new(v4, self.port()))
        } else {
            None
        }
    }

    /// Converts the socket address to an optional IPv6 socket address.
    #[must_use]
    pub const fn to_v6(self) -> Option<SocketAddressV6> {
        if let IPAddress::V6(v6) = self.ip() {
            Some(SocketAddressV6::new(v6, self.port()))
        } else {
            None
        }
    }

    /// Converts the socket address to an authority.
    pub const fn to_authority(self) -> Authority {
        Authority::new(self.ip().to_host(), self.port())
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

#[cfg(test)]
mod tests {
    use crate::{Authority, IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6};

    #[test]
    fn socket_to_v4() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let result: Option<SocketAddressV4> = socket.to_v4();
        let expected: Option<SocketAddressV4> = Some(SocketAddressV4::new(IPv4Address::LOCALHOST, 80));
        assert_eq!(result, expected);

        let socket: SocketAddress = IPv6Address::LOCALHOST.to_ip().to_socket(80);
        let result: Option<SocketAddressV4> = socket.to_v4();
        let expected: Option<SocketAddressV4> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn socket_to_v6() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_ip().to_socket(80);
        let result: Option<SocketAddressV6> = socket.to_v6();
        let expected: Option<SocketAddressV6> = None;
        assert_eq!(result, expected);

        let socket: SocketAddress = IPv6Address::LOCALHOST.to_ip().to_socket(80);
        let result: Option<SocketAddressV6> = socket.to_v6();
        let expected: Option<SocketAddressV6> = Some(SocketAddressV6::new(IPv6Address::LOCALHOST, 80));
        assert_eq!(result, expected);
    }

    #[test]
    fn socket_to_authority() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_socket(80).to_socket();
        let result: Authority = socket.to_authority();
        let expected: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn socket_from() {
        let expected: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        let result: SocketAddress = IPv4Address::LOCALHOST.to_socket(80).into();
        assert_eq!(result, expected);

        let expected: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        let result: SocketAddress = IPv6Address::LOCALHOST.to_socket(80).into();
        assert_eq!(result, expected);
    }
}
