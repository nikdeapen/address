use crate::{SocketAddress, SocketAddressV4, SocketAddressV6};
use std::fmt::{Debug, Display, Formatter};

impl Debug for SocketAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for SocketAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_std(), f)
    }
}

impl Debug for SocketAddressV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for SocketAddressV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_std(), f)
    }
}

impl Debug for SocketAddressV6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for SocketAddressV6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_std(), f)
    }
}

#[cfg(test)]
mod tests {
    use crate::{IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6};

    #[test]
    fn display() {
        let v4: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let v6: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let socket_v4: SocketAddress = v4.to_socket();
        let socket_v6: SocketAddress = v6.to_socket();
        let test_cases: &[(&str, String, &str)] = &[
            ("v4 {}", format!("{}", v4), "127.0.0.1:80"),
            ("v4 {:>16}", format!("{:>16}", v4), "    127.0.0.1:80"),
            ("v4 {:<16}", format!("{:<16}", v4), "127.0.0.1:80    "),
            ("v6 {}", format!("{}", v6), "[::1]:80"),
            ("v6 {:>16}", format!("{:>16}", v6), "        [::1]:80"),
            ("socket v4 {}", format!("{}", socket_v4), "127.0.0.1:80"),
            (
                "socket v4 {:>16}",
                format!("{:>16}", socket_v4),
                "    127.0.0.1:80",
            ),
            ("socket v6 {}", format!("{}", socket_v6), "[::1]:80"),
            (
                "socket v6 {:>16}",
                format!("{:>16}", socket_v6),
                "        [::1]:80",
            ),
        ];

        for (case, result, expected) in test_cases {
            assert_eq!(result, expected, "case={}", case);
        }
    }

    #[test]
    fn debug() {
        let v4: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let v6: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let socket_v4: SocketAddress = v4.to_socket();
        let socket_v6: SocketAddress = v6.to_socket();
        let test_cases: &[(&str, String, &str)] = &[
            ("v4 {:?}", format!("{:?}", v4), "127.0.0.1:80"),
            ("v4 {:>16?}", format!("{:>16?}", v4), "    127.0.0.1:80"),
            ("v6 {:?}", format!("{:?}", v6), "[::1]:80"),
            ("v6 {:>16?}", format!("{:>16?}", v6), "        [::1]:80"),
            ("socket v4 {:?}", format!("{:?}", socket_v4), "127.0.0.1:80"),
            ("socket v6 {:?}", format!("{:?}", socket_v6), "[::1]:80"),
        ];

        for (case, result, expected) in test_cases {
            assert_eq!(result, expected, "case={}", case);
        }
    }
}
