use crate::{IPAddress, IPv4Address, IPv6Address};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

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

impl From<Ipv4Addr> for IPAddress {
    fn from(std: Ipv4Addr) -> Self {
        Self::V4(IPv4Address::from(std))
    }
}

impl From<Ipv6Addr> for IPAddress {
    fn from(std: Ipv6Addr) -> Self {
        Self::V6(IPv6Address::from(std))
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
    fn ip_to_std() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();
        let std: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

        let result: IpAddr = ip.to_std();
        assert_eq!(result, std);

        let result: IpAddr = ip.into();
        assert_eq!(result, std);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();
        let std: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);

        let result: IpAddr = ip.to_std();
        assert_eq!(result, std);

        let result: IpAddr = ip.into();
        assert_eq!(result, std);
    }

    #[test]
    fn ip_from_std() {
        let ip: IPAddress = IPv4Address::LOCALHOST.to_ip();

        let result: IPAddress = IpAddr::V4(Ipv4Addr::LOCALHOST).into();
        assert_eq!(result, ip);

        let result: IPAddress = Ipv4Addr::LOCALHOST.into();
        assert_eq!(result, ip);

        let ip: IPAddress = IPv6Address::LOCALHOST.to_ip();

        let result: IPAddress = IpAddr::V6(Ipv6Addr::LOCALHOST).into();
        assert_eq!(result, ip);

        let result: IPAddress = Ipv6Addr::LOCALHOST.into();
        assert_eq!(result, ip);
    }
}
