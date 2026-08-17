use crate::{SocketAddressV6, doc_discards_zone_info};
use std::net::SocketAddrV6;

impl SocketAddressV6 {
    //! Standard Library Conversions

    /// Converts the address to a standard library address with a zero `flow_info` and `scope_id`.
    #[must_use]
    pub const fn to_std(self) -> SocketAddrV6 {
        self.to_std_with(0, 0)
    }

    /// Converts the address to a standard library address with the `flow_info` and `scope_id`.
    #[must_use]
    pub const fn to_std_with(self, flow_info: u32, scope_id: u32) -> SocketAddrV6 {
        SocketAddrV6::new(self.ip().to_std(), self.port(), flow_info, scope_id)
    }
}

impl From<SocketAddrV6> for SocketAddressV6 {
    #[doc = doc_discards_zone_info!()]
    fn from(std: SocketAddrV6) -> Self {
        Self::new((*std.ip()).into(), std.port())
    }
}

impl From<SocketAddressV6> for SocketAddrV6 {
    fn from(socket: SocketAddressV6) -> Self {
        socket.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPv6Address, SocketAddressV6};
    use std::net::{Ipv6Addr, SocketAddrV6};

    #[test]
    fn v6() {
        let socket: SocketAddressV6 = SocketAddressV6::new(IPv6Address::LOCALHOST, 80);
        let std: SocketAddrV6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0);

        let result: SocketAddrV6 = socket.to_std();
        assert_eq!(result, std);

        let result: SocketAddressV6 = std.into();
        assert_eq!(result, socket);

        let result: SocketAddrV6 = socket.into();
        assert_eq!(result, std);

        let result: SocketAddrV6 = socket.to_std_with(123, 456);
        let expected: SocketAddrV6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456);
        assert_eq!(result, expected);

        let std: SocketAddrV6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 123, 456);
        let result: SocketAddressV6 = std.into();
        let expected: SocketAddressV6 = SocketAddressV6::new(IPv6Address::LOCALHOST, 80);
        assert_eq!(result, expected);
    }
}
