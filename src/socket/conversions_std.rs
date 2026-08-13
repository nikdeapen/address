use crate::{IPAddress, SocketAddress};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

impl SocketAddress {
    //! Standard Library Conversions

    /// Converts the address to a standard library address with a zero `flow_info` and `scope_id` for IPv6 addresses.
    #[must_use]
    pub const fn to_std(self) -> SocketAddr {
        self.to_std_with(0, 0)
    }

    /// Converts the address to a standard library address with the `flow_info` and `scope_id` for IPv6 addresses.
    #[must_use]
    pub const fn to_std_with(self, flow_info: u32, scope_id: u32) -> SocketAddr {
        match self.ip() {
            IPAddress::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip.to_std(), self.port())),
            IPAddress::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip.to_std(), self.port(), flow_info, scope_id)),
        }
    }
}

impl From<SocketAddr> for SocketAddress {
    /// Converts the standard library address, discarding the `flow_info` & `scope_id` of IPv6 addresses.
    fn from(std: SocketAddr) -> Self {
        Self::new(std.ip().into(), std.port())
    }
}

impl From<SocketAddress> for SocketAddr {
    fn from(socket: SocketAddress) -> Self {
        socket.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPv4Address, IPv6Address, SocketAddress};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn socket() {
        let socket: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        let std: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80));

        let result: SocketAddr = socket.to_std();
        assert_eq!(result, std);

        let result: SocketAddress = std.into();
        assert_eq!(result, socket);

        let result: SocketAddr = socket.into();
        assert_eq!(result, std);

        let socket: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        let std: SocketAddr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0));

        let result: SocketAddr = socket.to_std();
        assert_eq!(result, std);

        let result: SocketAddress = std.into();
        assert_eq!(result, socket);

        let result: SocketAddr = socket.into();
        assert_eq!(result, std);

        let socket: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        let result: SocketAddr = socket.to_std_with(123, 456);
        let expected: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80));
        assert_eq!(result, expected);

        let socket: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        let result: SocketAddr = socket.to_std_with(123, 456);
        let expected: SocketAddr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456));
        assert_eq!(result, expected);

        let std: SocketAddr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456));
        let result: SocketAddress = std.into();
        let expected: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        assert_eq!(result, expected);
    }
}
