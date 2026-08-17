use crate::{
    Authority, SocketAddress, SocketAddressV4, SocketAddressV6, doc_discards_zone_info, doc_discards_zone_info_for_v6,
};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

impl From<SocketAddr> for Authority {
    #[doc = doc_discards_zone_info_for_v6!()]
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
    #[doc = doc_discards_zone_info!()]
    fn from(socket: SocketAddrV6) -> Self {
        SocketAddressV6::from(socket).to_authority()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Authority, IPv4Address, IPv6Address};
    use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn authority_from() {
        let expected: Authority = Authority::new(IPv4Address::LOCALHOST.to_host(), 80);

        let socket: SocketAddr = IPv4Address::LOCALHOST.to_ip().to_socket(80).to_std();
        let result: Authority = socket.into();
        assert_eq!(result, expected);

        let socket: SocketAddrV4 = IPv4Address::LOCALHOST.to_socket(80).into();
        let result: Authority = socket.into();
        assert_eq!(result, expected);

        let expected: Authority = Authority::new(IPv6Address::LOCALHOST.to_host(), 80);

        let socket: SocketAddrV6 = IPv6Address::LOCALHOST.to_socket(80).into();
        let result: Authority = socket.into();
        assert_eq!(result, expected);
    }
}
