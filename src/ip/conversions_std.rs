use crate::{IPAddress, IPv4Address, IPv6Address};
use std::net::IpAddr;

impl IPAddress {
    //! Standard Library Conversions

    /// Converts the address to a standard library address.
    #[must_use]
    pub const fn to_std(self) -> IpAddr {
        match self {
            Self::V4(ip) => IpAddr::V4(ip.to_std()),
            Self::V6(ip) => IpAddr::V6(ip.to_std()),
        }
    }
}

impl From<IpAddr> for IPAddress {
    fn from(std: IpAddr) -> Self {
        match std {
            IpAddr::V4(ip) => Self::V4(IPv4Address::from(ip)),
            IpAddr::V6(ip) => Self::V6(IPv6Address::from(ip)),
        }
    }
}

impl From<IPAddress> for IpAddr {
    fn from(ip: IPAddress) -> Self {
        ip.to_std()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPAddress, IPv4Address, IPv6Address};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    #[test]
    fn ip() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let std: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

        let result: IpAddr = ip.to_std();
        assert_eq!(result, std);

        let result: IPAddress = std.into();
        assert_eq!(result, ip);

        let result: IpAddr = ip.into();
        assert_eq!(result, std);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        let std: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);

        let result: IpAddr = ip.to_std();
        assert_eq!(result, std);

        let result: IPAddress = std.into();
        assert_eq!(result, ip);

        let result: IpAddr = ip.into();
        assert_eq!(result, std);
    }
}
