use crate::{
    AuthorityRef, SocketAddress, SocketAddressV4, SocketAddressV6, doc_discards_zone_info,
    doc_discards_zone_info_for_v6,
};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

impl<'a> From<SocketAddr> for AuthorityRef<'a> {
    #[doc = doc_discards_zone_info_for_v6!()]
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
    #[doc = doc_discards_zone_info!()]
    fn from(socket: SocketAddrV6) -> Self {
        SocketAddressV6::from(socket).to_authority_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthorityRef, IPv4Address, IPv6Address};
    use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn ref_from() {
        let expected: AuthorityRef = AuthorityRef::new(IPv4Address::LOCALHOST.to_host_ref(), 80);

        let socket: SocketAddr = IPv4Address::LOCALHOST.to_ip().to_socket(80).to_std();
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);

        let socket: SocketAddrV4 = IPv4Address::LOCALHOST.to_socket(80).into();
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);

        let expected: AuthorityRef = AuthorityRef::new(IPv6Address::LOCALHOST.to_host_ref(), 80);

        let socket: SocketAddrV6 = IPv6Address::LOCALHOST.to_socket(80).into();
        let result: AuthorityRef = socket.into();
        assert_eq!(result, expected);
    }
}
