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

#[cfg(test)]
mod tests {
    use crate::{IPAddress, IPv4Address, IPv6Address};

    #[test]
    fn display() {
        let v4: IPv4Address = IPv4Address::LOCALHOST;
        let v6: IPv6Address = IPv6Address::LOCALHOST;
        let ip_v4: IPAddress = v4.to_ip();
        let ip_v6: IPAddress = v6.to_ip();
        let test_cases: &[(&str, String, &str)] = &[
            ("v4 {}", format!("{}", v4), "127.0.0.1"),
            ("v4 {:>12}", format!("{:>12}", v4), "   127.0.0.1"),
            ("v4 {:<12}", format!("{:<12}", v4), "127.0.0.1   "),
            ("v6 {}", format!("{}", v6), "::1"),
            ("v6 {:>12}", format!("{:>12}", v6), "         ::1"),
            ("ip v4 {}", format!("{}", ip_v4), "127.0.0.1"),
            ("ip v4 {:>12}", format!("{:>12}", ip_v4), "   127.0.0.1"),
            ("ip v6 {}", format!("{}", ip_v6), "::1"),
            ("ip v6 {:>12}", format!("{:>12}", ip_v6), "         ::1"),
        ];

        for (case, result, expected) in test_cases {
            assert_eq!(result, expected, "case={}", case);
        }
    }

    #[test]
    fn debug() {
        let v4: IPv4Address = IPv4Address::LOCALHOST;
        let v6: IPv6Address = IPv6Address::LOCALHOST;
        let ip_v4: IPAddress = v4.to_ip();
        let ip_v6: IPAddress = v6.to_ip();
        let test_cases: &[(&str, String, &str)] = &[
            ("v4 {:?}", format!("{:?}", v4), "127.0.0.1"),
            ("v4 {:>12?}", format!("{:>12?}", v4), "   127.0.0.1"),
            ("v6 {:?}", format!("{:?}", v6), "::1"),
            ("v6 {:>12?}", format!("{:>12?}", v6), "         ::1"),
            ("ip v4 {:?}", format!("{:?}", ip_v4), "127.0.0.1"),
            ("ip v6 {:?}", format!("{:?}", ip_v6), "::1"),
        ];

        for (case, result, expected) in test_cases {
            assert_eq!(result, expected, "case={}", case);
        }
    }
}
