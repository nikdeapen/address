use crate::{SocketAddress, SocketAddressV4};

impl SocketAddressV4 {
    //! Conversions

    /// Converts the IPv4 socket address to a socket address.
    pub const fn to_socket(self) -> SocketAddress {
        SocketAddress::new(self.ip().to_ip(), self.port())
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPv4Address, SocketAddress, SocketAddressV4};

    #[test]
    fn v4_to_socket() {
        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let result: SocketAddress = socket.to_socket();
        let expected: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        assert_eq!(result, expected);
    }
}
