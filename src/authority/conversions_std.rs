use crate::{Authority, SocketAddress, SocketAddressV4, SocketAddressV6};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

impl From<SocketAddr> for Authority {
    /// The `flow_info` & `scope_id` are discarded for IPv6 socket addresses.
    fn from(socket: SocketAddr) -> Self {
        SocketAddress::from(socket).to_authority()
    }
}

impl From<SocketAddrV4> for Authority {
    fn from(socket: SocketAddrV4) -> Self {
        SocketAddressV4::from(socket).to_authority()
    }
}

impl From<SocketAddrV6> for Authority {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(socket: SocketAddrV6) -> Self {
        SocketAddressV6::from(socket).to_authority()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, IPv4Address, IPv6Address};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn from() {
        let expected: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);

        let result: Authority = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80)).into();
        assert_eq!(result, expected);

        let result: Authority = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80).into();
        assert_eq!(result, expected);

        let expected: Authority = Authority::new(IPv6Address::LOCALHOST.to_host(), 80);

        let result: Authority =
            SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0)).into();
        assert_eq!(result, expected);

        let result: Authority = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0).into();
        assert_eq!(result, expected);
    }
}
