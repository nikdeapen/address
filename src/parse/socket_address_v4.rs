use crate::parse_port;
use crate::{IPv4Address, ParseError, SocketAddressV4, impl_parse};

impl_parse!(SocketAddressV4);

impl TryFrom<&[u8]> for SocketAddressV4 {
    type Error = ParseError;

    fn try_from(socket: &[u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(socket)?;
        let ip: IPv4Address = IPv4Address::parse(s)?;
        Ok(Self::new(ip, port))
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv4Address, InvalidPort};
    use crate::{IPv4Address, ParseError, SocketAddressV4};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<SocketAddressV4, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("127.0.0.1:", Err(InvalidPort)),
            ("127.0.0.1:xx", Err(InvalidPort)),
            (":80", Err(InvalidIPv4Address)),
            ("xx:80", Err(InvalidIPv4Address)),
            ("127.0.0.1:80", Ok(IPv4Address::LOCALHOST.to_socket(80))),
            ("127.0.0.1:65535", Ok(IPv4Address::LOCALHOST.to_socket(65535))),
            ("127.0.0.1:65536", Err(InvalidPort)),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV4, ParseError> = SocketAddressV4::try_from(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
