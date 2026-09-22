use crate::{AuthorityRef, SocketAddress, SocketAddressV4, SocketAddressV6};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

impl<'a> From<SocketAddr> for AuthorityRef<'a> {
    /// The `flow_info` & `scope_id` are discarded for IPv6 socket addresses.
    fn from(socket: SocketAddr) -> Self {
        SocketAddress::from(socket).to_authority_ref()
    }
}

impl<'a> From<SocketAddrV4> for AuthorityRef<'a> {
    fn from(socket: SocketAddrV4) -> Self {
        SocketAddressV4::from(socket).to_authority_ref()
    }
}

impl<'a> From<SocketAddrV6> for AuthorityRef<'a> {
    /// The `flow_info` & `scope_id` are discarded.
    fn from(socket: SocketAddrV6) -> Self {
        SocketAddressV6::from(socket).to_authority_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthorityRef, IPv4Address, IPv6Address};
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn ref_from_std() {
        let expected: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);

        let result: AuthorityRef =
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80)).into();
        assert_eq!(result, expected);

        let result: AuthorityRef = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80).into();
        assert_eq!(result, expected);

        let expected: AuthorityRef = AuthorityRef::new(IPv6Address::LOCALHOST.to_host_ref(), 80);

        let result: AuthorityRef = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0).into();
        assert_eq!(result, expected);
    }
}
