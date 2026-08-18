use crate::SocketAddressV4;
use std::net::SocketAddrV4;

impl SocketAddressV4 {
    //! Standard Library Conversions

    /// Converts the address to a standard library address.
    #[must_use]
    pub const fn to_std(self) -> SocketAddrV4 {
        SocketAddrV4::new(self.ip().to_std(), self.port())
    }
}

impl From<SocketAddrV4> for SocketAddressV4 {
    fn from(std: SocketAddrV4) -> Self {
        Self::new((*std.ip()).into(), std.port())
    }
}

impl From<SocketAddressV4> for SocketAddrV4 {
    fn from(socket: SocketAddressV4) -> Self {
        socket.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPv4Address, SocketAddressV4};
    use std::net::{Ipv4Addr, SocketAddrV4};

    #[test]
    fn v4_to_std() {
        let socket: SocketAddressV4 = SocketAddressV4::new(IPv4Address::LOCALHOST, 80);
        let std: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80);

        let result: SocketAddrV4 = socket.to_std();
        assert_eq!(result, std);

        let result: SocketAddrV4 = socket.into();
        assert_eq!(result, std);
    }

    #[test]
    fn v4_from_std() {
        let result: SocketAddressV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80).into();
        assert_eq!(result, SocketAddressV4::new(IPv4Address::LOCALHOST, 80));
    }
}
