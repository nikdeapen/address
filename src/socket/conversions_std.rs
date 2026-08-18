use crate::{IPAddress, SocketAddress, SocketAddressV4, SocketAddressV6};
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
    /// The `flow_info` & `scope_id` are discarded for IPv6 socket addresses.
    fn from(std: SocketAddr) -> Self {
        Self::new(std.ip().into(), std.port())
    }
}

impl From<SocketAddrV4> for SocketAddress {
    fn from(std: SocketAddrV4) -> Self {
        SocketAddressV4::from(std).to_socket()
    }
}

impl From<SocketAddrV6> for SocketAddress {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(std: SocketAddrV6) -> Self {
        SocketAddressV6::from(std).to_socket()
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
    fn socket_to_std() {
        let socket: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        let std: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80));

        let result: SocketAddr = socket.to_std();
        assert_eq!(result, std);

        let result: SocketAddr = socket.into();
        assert_eq!(result, std);

        let socket: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        let std: SocketAddr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0));

        let result: SocketAddr = socket.to_std();
        assert_eq!(result, std);

        let result: SocketAddr = socket.into();
        assert_eq!(result, std);
    }

    /// The `flow_info` & `scope_id` apply to IPv6 addresses only.
    #[test]
    fn socket_to_std_with() {
        let socket: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);
        let result: SocketAddr = socket.to_std_with(123, 456);
        let expected: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80));
        assert_eq!(result, expected);

        let socket: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);
        let result: SocketAddr = socket.to_std_with(123, 456);
        let expected: SocketAddr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456));
        assert_eq!(result, expected);
    }

    /// The `flow_info` & `scope_id` are discarded, so both zones give the same address.
    #[test]
    fn socket_from_std() {
        let socket: SocketAddress = SocketAddress::new(IPv4Address::LOCALHOST.to_ip(), 80);

        let result: SocketAddress = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80)).into();
        assert_eq!(result, socket);

        let result: SocketAddress = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80).into();
        assert_eq!(result, socket);

        let socket: SocketAddress = SocketAddress::new(IPv6Address::LOCALHOST.to_ip(), 80);

        let result: SocketAddress = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0)).into();
        assert_eq!(result, socket);

        let result: SocketAddress = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0).into();
        assert_eq!(result, socket);

        let result: SocketAddress = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456)).into();
        assert_eq!(result, socket);

        let result: SocketAddress = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456).into();
        assert_eq!(result, socket);
    }
}
