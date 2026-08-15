use crate::ParseError::InvalidSocketAddressV6;
use crate::{IPv6Address, ParseError, SocketAddressV6, impl_parse};
use crate::{parse_port, strip_brackets, strip_zone};

impl_parse!(
    SocketAddressV6,
    "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
);

impl TryFrom<&[u8]> for SocketAddressV6 {
    type Error = ParseError;

    /// A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`.
    fn try_from(socket: &[u8]) -> Result<Self, Self::Error> {
        let (s, port): (&[u8], u16) = parse_port(socket)?;
        let s: &[u8] = strip_brackets(s).ok_or(InvalidSocketAddressV6)?;
        let s: &[u8] = strip_zone(s).ok_or(InvalidSocketAddressV6)?;
        let ip: IPv6Address = IPv6Address::parse(s)?;
        Ok(Self::new(ip, port))
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError::{InvalidIPv6Address, InvalidPort, InvalidSocketAddressV6};
    use crate::{IPv6Address, ParseError, SocketAddressV6};
    use std::str::FromStr;

    #[test]
    fn parse() {
        let test_cases: &[(&str, Result<SocketAddressV6, ParseError>)] = &[
            ("", Err(InvalidPort)),
            ("[::1]:", Err(InvalidPort)),
            ("[::1]:xx", Err(InvalidPort)),
            (":80", Err(InvalidSocketAddressV6)),
            ("xx:80", Err(InvalidSocketAddressV6)),
            ("[xx]:80", Err(InvalidIPv6Address)),
            ("[::1%]:80", Err(InvalidSocketAddressV6)),
            ("[::1%eth0]:80", Err(InvalidSocketAddressV6)),
            ("[::1%4294967296]:80", Err(InvalidSocketAddressV6)),
            ("[::1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80))),
            ("[::1%1]:80", Ok(IPv6Address::LOCALHOST.to_socket(80))),
        ];

        for (input, expected) in test_cases {
            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::from_str(input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::try_from(*input);
            assert_eq!(result, *expected, "input={}", input);

            let result: Result<SocketAddressV6, ParseError> = SocketAddressV6::try_from(input.as_bytes());
            assert_eq!(result, *expected, "input={}", input);
        }
    }
}
