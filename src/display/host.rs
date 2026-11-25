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
            Self::IPAddress(ip) => Display::fmt(ip, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, Host, IPv4Address, IPv6Address};

    #[test]
    fn display() {
        let domain: Host = Domain::localhost().to_host();
        let v4: Host = IPv4Address::LOCALHOST.to_host();
        let v6: Host = IPv6Address::LOCALHOST.to_host();
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "domain {}",
                format!("{}", domain),
                format!("{}", domain.to_ref()),
                "localhost",
            ),
            (
                "domain {:>12}",
                format!("{:>12}", domain),
                format!("{:>12}", domain.to_ref()),
                "   localhost",
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
                "127.0.0.1",
            ),
            (
                "v4 {:>12}",
                format!("{:>12}", v4),
                format!("{:>12}", v4.to_ref()),
                "   127.0.0.1",
            ),
            (
                "v6 {}",
                format!("{}", v6),
                format!("{}", v6.to_ref()),
                "::1",
            ),
            (
                "v6 {:>12}",
                format!("{:>12}", v6),
                format!("{:>12}", v6.to_ref()),
                "         ::1",
            ),
        ];

        for (case, owned, host, expected) in test_cases {
            assert_eq!(owned, expected, "owned case={}", case);
            assert_eq!(host, expected, "ref case={}", case);
        }
    }

    #[test]
    fn debug() {
        let domain: Host = Domain::localhost().to_host();
        let v4: Host = IPv4Address::LOCALHOST.to_host();
        let v6: Host = IPv6Address::LOCALHOST.to_host();
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "domain {:?}",
                format!("{:?}", domain),
                format!("{:?}", domain.to_ref()),
                "localhost",
            ),
            (
                "domain {:>12?}",
                format!("{:>12?}", domain),
                format!("{:>12?}", domain.to_ref()),
                "   localhost",
            ),
            (
                "v4 {:?}",
                format!("{:?}", v4),
                format!("{:?}", v4.to_ref()),
                "127.0.0.1",
            ),
            (
                "v6 {:?}",
                format!("{:?}", v6),
                format!("{:?}", v6.to_ref()),
                "::1",
            ),
        ];

        for (case, owned, host, expected) in test_cases {
            assert_eq!(owned, expected, "owned case={}", case);
            assert_eq!(host, expected, "ref case={}", case);
        }
    }
}
