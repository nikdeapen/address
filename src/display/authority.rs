use crate::{Authority, AuthorityRef, EndpointRef, HostRef};
use std::fmt::{Debug, Display, Formatter};

impl Debug for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Authority {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_ref(), f)
    }
}

impl<'a> Debug for AuthorityRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for AuthorityRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.host() {
            HostRef::Domain(domain) => Display::fmt(&EndpointRef::new(domain, self.port()), f),
            HostRef::IPAddress(ip) => Display::fmt(&ip.to_socket(self.port()), f),
        }
    }
}
