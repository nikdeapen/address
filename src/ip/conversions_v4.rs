use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddressV4};

impl IPv4Address {
    //! Conversions

    /// Converts the address to an IPv6 compatible address. (::a.b.c.d)
    ///
    /// The compatible format is deprecated (RFC 4291); prefer [`Self::to_v6_mapped`].
    pub const fn to_v6_compatible(self) -> IPv6Address {
        let (a, b, c, d) = self.bytes();
        IPv6Address::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, a, b, c, d])
    }

    /// Converts the address to an IPv6 mapped address. (::ffff:a.b.c.d)
    pub const fn to_v6_mapped(self) -> IPv6Address {
        let (a, b, c, d) = self.bytes();
        IPv6Address::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d])
    }

    /// Converts the address to an IP address.
    pub const fn to_ip(self) -> IPAddress {
        IPAddress::V4(self)
    }

    /// Converts the address to a socket address with the `port`.
    pub const fn to_socket(self, port: u16) -> SocketAddressV4 {
        SocketAddressV4::new(self, port)
    }

    /// Converts the address to a host.
    pub const fn to_host(self) -> Host {
        Host::Address(self.to_ip())
    }

    /// Converts the address to a host reference.
    pub const fn to_host_ref(self) -> HostRef<'static> {
        HostRef::Address(self.to_ip())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddressV4};

    #[test]
    fn v4_to_v6() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let result: IPv6Address = ip.to_v6_compatible();
        let expected: IPv6Address = IPv6Address::from([0, 0, 0, 0, 0, 0, 0x7F00, 1]);
        assert_eq!(result, expected);

        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let result: IPv6Address = ip.to_v6_mapped();
        let expected: IPv6Address = IPv6Address::from([0, 0, 0, 0, 0, 0xFFFF, 0x7F00, 1]);
        assert_eq!(result, expected);
    }

    #[test]
    fn v4_to_ip() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let result: IPAddress = ip.to_ip();
        let expected: IPAddress = IPAddress::V4(IPv4Address::LOCALHOST);
        assert_eq!(result, expected);
    }

    #[test]
    fn v4_to_socket() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;
        let result: SocketAddressV4 = ip.to_socket(80);
        let expected: SocketAddressV4 = SocketAddressV4::new(IPv4Address::LOCALHOST, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn v4_to_host() {
        let ip: IPv4Address = IPv4Address::LOCALHOST;

        let result: Host = ip.to_host();
        let expected: Host = Host::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);

        let result: HostRef = ip.to_host_ref();
        let expected: HostRef = HostRef::Address(IPAddress::V4(IPv4Address::LOCALHOST));
        assert_eq!(result, expected);
    }
}
