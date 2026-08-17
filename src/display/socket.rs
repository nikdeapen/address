use crate::{SocketAddress, SocketAddressV4, SocketAddressV6};
use std::fmt::{Debug, Display, Formatter};

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

#[cfg(test)]
mod tests {
    use crate::{IPv4Address, IPv6Address, SocketAddress, SocketAddressV4, SocketAddressV6};

    #[test]
    fn v4() {
        let socket: SocketAddressV4 = IPv4Address::LOCALHOST.to_socket(80);
        let result: String = socket.to_string();
        let expected: &str = "127.0.0.1:80";
        assert_eq!(result, expected);
    }

    #[test]
    fn v6() {
        let socket: SocketAddressV6 = IPv6Address::LOCALHOST.to_socket(80);
        let result: String = socket.to_string();
        let expected: &str = "[::1]:80";
        assert_eq!(result, expected);
    }

    #[test]
    fn socket() {
        let socket: SocketAddress = IPv4Address::LOCALHOST.to_socket(80).to_socket();
        let result: String = socket.to_string();
        let expected: &str = "127.0.0.1:80";
        assert_eq!(result, expected);

        let socket: SocketAddress = IPv6Address::LOCALHOST.to_socket(80).to_socket();
        let result: String = socket.to_string();
        let expected: &str = "[::1]:80";
        assert_eq!(result, expected);
    }

    #[test]
    fn display_spec() {
        assert_eq!(
            format!("{:>15}", IPv4Address::LOCALHOST.to_socket(80)),
            "   127.0.0.1:80"
        );
        assert_eq!(
            format!("{:<15}|", IPv4Address::LOCALHOST.to_socket(80)),
            "127.0.0.1:80   |"
        );
        assert_eq!(format!("{:>11}", IPv6Address::LOCALHOST.to_socket(80)), "   [::1]:80");
        assert_eq!(
            format!("{:>15}", IPv4Address::LOCALHOST.to_socket(80).to_socket()),
            "   127.0.0.1:80"
        );
    }
}
