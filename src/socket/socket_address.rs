use crate::IPAddress;

/// An [IPAddress] with an associated port.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SocketAddress {
    ip: IPAddress,
    port: u16,
}

impl SocketAddress {
    //! Construction

    /// Creates a new [SocketAddress].
    pub const fn new(ip: IPAddress, port: u16) -> Self {
        Self { ip, port }
    }
}

impl<A: Into<IPAddress>> From<(A, u16)> for SocketAddress {
    fn from(tuple: (A, u16)) -> Self {
        Self::new(tuple.0.into(), tuple.1)
    }
}

impl From<SocketAddress> for (IPAddress, u16) {
    fn from(socket: SocketAddress) -> Self {
        (socket.ip, socket.port)
    }
}

impl SocketAddress {
    //! Properties

    /// Gets the IP address.
    pub const fn ip(self) -> IPAddress {
        self.ip
    }

    /// Gets the port.
    #[must_use]
    pub const fn port(self) -> u16 {
        self.port
    }
}

impl SocketAddress {
    //! Matching

    /// Checks if the socket address is an IPv4 socket address.
    #[must_use]
    pub const fn is_v4(self) -> bool {
        self.ip.is_v4()
    }

    /// Checks if the socket address is an IPv6 socket address.
    #[must_use]
    pub const fn is_v6(self) -> bool {
        self.ip.is_v6()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPAddress, IPv4Address, IPv6Address, SocketAddress};

    #[test]
    fn construction() {
        let socket: SocketAddress = SocketAddress::new(IPAddress::V4(IPv4Address::LOCALHOST), 80);
        assert_eq!(socket.ip, IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(socket.port, 80);

        let socket: SocketAddress = (IPv4Address::LOCALHOST, 80).into();
        assert_eq!(socket.ip, IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(socket.port, 80);
    }

    #[test]
    fn deconstruction() {
        let socket: SocketAddress = (IPv4Address::LOCALHOST, 80).into();
        let (ip, port): (IPAddress, u16) = socket.into();
        assert_eq!(ip, IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(port, 80);
    }

    #[test]
    fn properties() {
        let socket: SocketAddress = (IPv4Address::LOCALHOST, 80).into();
        assert_eq!(socket.ip(), IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(socket.port(), 80);
    }

    #[test]
    fn matching() {
        let socket: SocketAddress = (IPv4Address::LOCALHOST, 80).into();
        assert!(socket.is_v4());
        assert!(!socket.is_v6());

        let socket: SocketAddress = (IPv6Address::LOCALHOST, 80).into();
        assert!(!socket.is_v4());
        assert!(socket.is_v6());
    }
}
