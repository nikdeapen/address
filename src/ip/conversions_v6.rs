use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddressV6};

impl IPv6Address {
    //! Conversions

    /// Converts an IPv4 compatible (::a.b.c.d) or IPv4 mapped (::ffff:a.b.c.d) address to an IPv4 address.
    ///
    /// Plain IPv6 addresses match the compatible pattern (`::1` -> `Some(0.0.0.1)`); use [`Self::to_v4_mapped`] to
    /// avoid these false positives.
    #[must_use]
    pub const fn to_v4(self) -> Option<IPv4Address> {
        match self.address() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, a, b, c, d] => Some(IPv4Address::new([a, b, c, d])),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d] => Some(IPv4Address::new([a, b, c, d])),
            _ => None,
        }
    }

    /// Converts an IPv4 mapped (::ffff:a.b.c.d) address to an IPv4 address.
    ///
    /// Unlike [`Self::to_v4`], IPv4 compatible addresses (::a.b.c.d) return `None`.
    #[must_use]
    pub const fn to_v4_mapped(self) -> Option<IPv4Address> {
        match self.address() {
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, a, b, c, d] => Some(IPv4Address::new([a, b, c, d])),
            _ => None,
        }
    }

    /// Converts the address to an IP address.
    pub const fn to_ip(self) -> IPAddress {
        IPAddress::V6(self)
    }

    /// Converts the address to a socket address with the `port`.
    pub const fn to_socket(self, port: u16) -> SocketAddressV6 {
        SocketAddressV6::new(self, port)
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
    use crate::{Host, HostRef, IPAddress, IPv4Address, IPv6Address, SocketAddressV6};

    #[test]
    fn v6_to_v4() {
        let test_cases: &[(IPv6Address, Option<IPv4Address>)] = &[
            (
                IPv6Address::from([0, 0, 0, 0, 0, 0, 0x7F00, 1]),
                Some(IPv4Address::LOCALHOST),
            ),
            (
                IPv6Address::from([0, 0, 0, 0, 0, 0xFFFF, 0x7F00, 1]),
                Some(IPv4Address::LOCALHOST),
            ),
            (IPv6Address::from([1, 0, 0, 0, 0, 0, 0, 0]), None),
            (IPv6Address::from([0, 0, 0, 0, 0, 1, 0, 0]), None),
        ];

        for (ip, expected) in test_cases {
            let result: Option<IPv4Address> = ip.to_v4();
            assert_eq!(result, *expected, "ip={}", ip);
        }

        let ip: IPv6Address = IPv6Address::from([0, 0, 0, 0, 0, 0xFFFF, 0x7F00, 1]);
        let result: Option<IPv4Address> = ip.to_v4_mapped();
        let expected: Option<IPv4Address> = Some(IPv4Address::LOCALHOST);
        assert_eq!(result, expected);

        let ip: IPv6Address = IPv6Address::from([0, 0, 0, 0, 0, 0, 0x7F00, 1]);
        let result: Option<IPv4Address> = ip.to_v4_mapped();
        let expected: Option<IPv4Address> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn v6_to_ip() {
        let ip: IPv6Address = IPv6Address::LOCALHOST;
        let result: IPAddress = ip.to_ip();
        let expected: IPAddress = IPAddress::V6(IPv6Address::LOCALHOST);
        assert_eq!(result, expected);
    }

    #[test]
    fn v6_to_socket() {
        let ip: IPv6Address = IPv6Address::LOCALHOST;
        let result: SocketAddressV6 = ip.to_socket(80);
        let expected: SocketAddressV6 = SocketAddressV6::new(IPv6Address::LOCALHOST, 80);
        assert_eq!(result, expected);
    }

    #[test]
    fn v6_to_host() {
        let ip: IPv6Address = IPv6Address::LOCALHOST;

        let result: Host = ip.to_host();
        let expected: Host = Host::Address(IPAddress::V6(IPv6Address::LOCALHOST));
        assert_eq!(result, expected);

        let result: HostRef = ip.to_host_ref();
        let expected: HostRef = HostRef::Address(IPAddress::V6(IPv6Address::LOCALHOST));
        assert_eq!(result, expected);
    }
}
