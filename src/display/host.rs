use crate::{Host, HostRef};
use std::fmt::{Debug, Display, Formatter};

impl Debug for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_ref(), f)
    }
}

impl<'a> Debug for HostRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for HostRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Domain(domain) => Display::fmt(domain, f),
            Self::IP(ip) => Display::fmt(ip, f),
        }
    }
}
