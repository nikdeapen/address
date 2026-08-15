use crate::ParseError::InvalidSocketAddress;
use crate::{IPv4Address, IPv6Address, ParseError, SocketAddress, impl_parse};
use crate::{parse_port, strip_brackets, strip_zone};

impl_parse!(
    SocketAddress,
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

impl TryFrom<&[u8]> for SocketAddress {
    type Error = ParseError;

    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    fn try_from(socket: &[u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(socket)?;
        if let Some(s) = strip_brackets(s) {
            let s: &[u8] = strip_zone(s).ok_or(InvalidSocketAddress)?;
            let ip: IPv6Address = IPv6Address::parse(s)?;
            Ok(ip.to_ip().to_socket(port))
        } else {
            let ip: IPv4Address = IPv4Address::parse(s).map_err(|_| InvalidSocketAddress)?;
            Ok(ip.to_ip().to_socket(port))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv6Address, InvalidPort, InvalidSocketAddress};
    use crate::{IPv6Address, ParseError, SocketAddress};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<SocketAddress, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("[::1]:", Err(InvalidPort)),
            ("[::1]:xx", Err(InvalidPort)),
            (":80", Err(InvalidSocketAddress)),
            ("xx:80", Err(InvalidSocketAddress)),
            ("::1:80", Err(InvalidSocketAddress)),
            ("[]:80", Err(InvalidIPv6Address)),
            ("[xx]:80", Err(InvalidIPv6Address)),
            ("[::1%eth0]:80", Err(InvalidSocketAddress)),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80).to_socket())),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80).to_socket())),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddress, ParseError> = SocketAddress::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddress, ParseError> = SocketAddress::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddress, ParseError> = SocketAddress::try_from(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
