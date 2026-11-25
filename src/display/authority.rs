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

#[cfg(test)]
mod tests {
    use crate::{Authority, Domain, IPv4Address, IPv6Address};

    #[test]
    fn display() {
        let domain: Authority = Domain::localhost().to_host().to_authority(80);
        let v4: Authority = IPv4Address::LOCALHOST.to_host().to_authority(80);
        let v6: Authority = IPv6Address::LOCALHOST.to_host().to_authority(80);
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "domain {}",
                format!("{}", domain),
                format!("{}", domain.to_ref()),
                "localhost:80",
            ),
            (
                "domain {:>16}",
                format!("{:>16}", domain),
                format!("{:>16}", domain.to_ref()),
                "    localhost:80",
            ),
            (
                "domain {:<16}",
                format!("{:<16}", domain),
                format!("{:<16}", domain.to_ref()),
                "localhost:80    ",
            ),
            (
                "domain {:.5}",
                format!("{:.5}", domain),
                format!("{:.5}", domain.to_ref()),
                "local",
            ),
            (
                "v4 {}",
                format!("{}", v4),
                format!("{}", v4.to_ref()),
                "127.0.0.1:80",
            ),
            (
                "v4 {:>16}",
                format!("{:>16}", v4),
                format!("{:>16}", v4.to_ref()),
                "    127.0.0.1:80",
            ),
            (
                "v6 {}",
                format!("{}", v6),
                format!("{}", v6.to_ref()),
                "[::1]:80",
            ),
            (
                "v6 {:>16}",
                format!("{:>16}", v6),
                format!("{:>16}", v6.to_ref()),
                "        [::1]:80",
            ),
        ];

        for (case, owned, authority, expected) in test_cases {
            assert_eq!(owned, expected, "owned case={}", case);
            assert_eq!(authority, expected, "ref case={}", case);
        }
    }

    #[test]
    fn debug() {
        let domain: Authority = Domain::localhost().to_host().to_authority(80);
        let v4: Authority = IPv4Address::LOCALHOST.to_host().to_authority(80);
        let v6: Authority = IPv6Address::LOCALHOST.to_host().to_authority(80);
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "domain {:?}",
                format!("{:?}", domain),
                format!("{:?}", domain.to_ref()),
                "localhost:80",
            ),
            (
                "domain {:>16?}",
                format!("{:>16?}", domain),
                format!("{:>16?}", domain.to_ref()),
                "    localhost:80",
            ),
            (
                "v4 {:?}",
                format!("{:?}", v4),
                format!("{:?}", v4.to_ref()),
                "127.0.0.1:80",
            ),
            (
                "v6 {:?}",
                format!("{:?}", v6),
                format!("{:?}", v6.to_ref()),
                "[::1]:80",
            ),
        ];

        for (case, owned, authority, expected) in test_cases {
            assert_eq!(owned, expected, "owned case={}", case);
            assert_eq!(authority, expected, "ref case={}", case);
        }
    }
}
