use crate::{Domain, Endpoint, EndpointRef};
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;

impl Debug for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_ref(), f)
    }
}

impl<'a> Debug for EndpointRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for EndpointRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const MAX_LEN: usize = Domain::MAX_NAME_LEN + 1 + "65535".len();
        if f.width().is_none() && f.precision().is_none() {
            write!(f, "{}:{}", self.domain(), self.port())
        } else {
            let mut bytes: [u8; MAX_LEN] = [0; MAX_LEN];
            let mut cursor: &mut [u8] = &mut bytes;
            write!(cursor, "{}:{}", self.domain(), self.port()).map_err(|_| std::fmt::Error)?;
            let len: usize = MAX_LEN - cursor.len();
            f.pad(unsafe { std::str::from_utf8_unchecked(&bytes[..len]) })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef, Endpoint, EndpointRef};

    #[test]
    fn display() {
        let owned: Endpoint = Endpoint::new(Domain::example(), 443);
        let endpoint: EndpointRef = EndpointRef::new(DomainRef::EXAMPLE, 443);
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "{}",
                format!("{}", owned),
                format!("{}", endpoint),
                "example.com:443",
            ),
            (
                "{:19}",
                format!("{:19}", owned),
                format!("{:19}", endpoint),
                "example.com:443    ",
            ),
            (
                "{:>19}",
                format!("{:>19}", owned),
                format!("{:>19}", endpoint),
                "    example.com:443",
            ),
            (
                "{:^19}",
                format!("{:^19}", owned),
                format!("{:^19}", endpoint),
                "  example.com:443  ",
            ),
            (
                "{:*<19}",
                format!("{:*<19}", owned),
                format!("{:*<19}", endpoint),
                "example.com:443****",
            ),
            (
                "{:1}",
                format!("{:1}", owned),
                format!("{:1}", endpoint),
                "example.com:443",
            ),
            (
                "{:.7}",
                format!("{:.7}", owned),
                format!("{:.7}", endpoint),
                "example",
            ),
            (
                "{:.13}",
                format!("{:.13}", owned),
                format!("{:.13}", endpoint),
                "example.com:4",
            ),
            (
                "{:>9.7}",
                format!("{:>9.7}", owned),
                format!("{:>9.7}", endpoint),
                "  example",
            ),
        ];

        for (spec, owned, endpoint, expected) in test_cases {
            assert_eq!(owned, expected, "owned spec={}", spec);
            assert_eq!(endpoint, expected, "ref spec={}", spec);
        }
    }

    #[test]
    fn display_max_len() {
        let label: String = "a".repeat(Domain::MAX_LABEL_LEN);
        let name: String = format!("{}.{}.{}.{}", label, label, label, "a".repeat(61));
        assert_eq!(name.len(), Domain::MAX_NAME_LEN);

        let endpoint: Endpoint = Endpoint::new(name.parse().unwrap(), u16::MAX);
        let expected: String = format!("{}:65535", name);
        assert_eq!(format!("{}", endpoint), expected);
        assert_eq!(format!("{:1}", endpoint), expected);
        assert_eq!(format!("{:.300}", endpoint), expected);

        let padded: String = format!("{}{}", " ".repeat(300 - expected.len()), expected);
        assert_eq!(format!("{:>300}", endpoint), padded);
    }

    #[test]
    fn debug() {
        let owned: Endpoint = Endpoint::new(Domain::example(), 443);
        let endpoint: EndpointRef = EndpointRef::new(DomainRef::EXAMPLE, 443);
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "{:?}",
                format!("{:?}", owned),
                format!("{:?}", endpoint),
                "example.com:443",
            ),
            (
                "{:#?}",
                format!("{:#?}", owned),
                format!("{:#?}", endpoint),
                "example.com:443",
            ),
            (
                "{:>19?}",
                format!("{:>19?}", owned),
                format!("{:>19?}", endpoint),
                "    example.com:443",
            ),
        ];

        for (spec, owned, endpoint, expected) in test_cases {
            assert_eq!(owned, expected, "owned spec={}", spec);
            assert_eq!(endpoint, expected, "ref spec={}", spec);
        }
    }
}
