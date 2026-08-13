use crate::{SocketAddress, SocketAddressV6};

impl SocketAddressV6 {
    //! Conversions

    /// Converts the IPv6 socket address to a socket address.
    pub const fn to_socket(self) -> SocketAddress {
        SocketAddress::new(self.ip().to_ip(), self.port())
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPv6Address, SocketAddress, SocketAddressV6};

    #[test]
    fn v6_to_socket() {
        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: SocketAddress = socket.to_socket();
        let expected: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        assert_eq!(result, expected);
    }
}
