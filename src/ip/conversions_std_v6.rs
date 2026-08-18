use crate::IPv6Address;
use std::net::Ipv6Addr;

impl IPv6Address {
    //! Standard Library Conversions

    /// Converts the address to a standard library address.
    #[must_use]
    pub const fn to_std(self) -> Ipv6Addr {
        let segments: [u16; 8] = self.segments();
        Ipv6Addr::new(
            segments[0],
            segments[1],
            segments[2],
            segments[3],
            segments[4],
            segments[5],
            segments[6],
            segments[7],
        )
    }
}

impl From<Ipv6Addr> for IPv6Address {
    fn from(std: Ipv6Addr) -> Self {
        Self::new(std.octets())
    }
}

impl From<IPv6Address> for Ipv6Addr {
    fn from(ip: IPv6Address) -> Self {
        ip.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::IPv6Address;
    use std::net::Ipv6Addr;

    #[test]
    fn v6_to_std() {
        let ip: IPv6Address = IPv6Address::LOCALHOST;
        let std: Ipv6Addr = Ipv6Addr::LOCALHOST;

        let result: Ipv6Addr = ip.to_std();
        assert_eq!(result, std);

        let result: Ipv6Addr = ip.into();
        assert_eq!(result, std);
    }

    #[test]
    fn v6_from_std() {
        let result: IPv6Address = Ipv6Addr::LOCALHOST.into();
        assert_eq!(result, IPv6Address::LOCALHOST);
    }
}
