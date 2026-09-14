use crate::{IPAddress, IPv4Address, IPv6Address};
use std::fmt::{Debug, Display, Formatter};

impl Debug for IPAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for IPAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V4(ip) => Display::fmt(ip, f),
            Self::V6(ip) => Display::fmt(ip, f),
        }
    }
}

impl Debug for IPv4Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for IPv4Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_std(), f)
    }
}

impl Debug for IPv6Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for IPv6Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_std(), f)
    }
}
