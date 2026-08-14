use crate::{Authority, SocketAddress, SocketAddressV4};

impl SocketAddressV4 {
    //! Conversions

    /// Converts the IPv4 socket address to a socket address.
    pub const fn to_socket(self) -> SocketAddress {
        SocketAddress::new(self.ip().to_ip(), self.port())
    }

    /// Converts the IPv4 socket address to an authority.
    pub const fn to_authority(self) -> Authority {
        Authority::new(self.ip().to_host(), self.port())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, IPv4Address, SocketAddress, SocketAddressV4};

    #[test]
    fn v4_to_socket() {
        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let result: SocketAddress = socket.to_socket();
        let expected: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn v4_to_authority() {
        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let result: Authority = socket.to_authority();
        let expected: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);
        assert_eq!(result, expected);
    }
}
