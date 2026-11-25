use crate::{Domain, DomainRef};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};

impl Debug for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.to_ref(), f)
    }
}

impl AsRef<str> for Domain {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl Borrow<str> for Domain {
    fn borrow(&self) -> &str {
        self.name()
    }
}

impl<'a> Debug for DomainRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<'a> Display for DomainRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(self.name())
    }
}

impl<'a> AsRef<str> for DomainRef<'a> {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl<'a> Borrow<str> for DomainRef<'a> {
    fn borrow(&self) -> &str {
        self.name()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef};
    use std::borrow::Borrow;
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn display() {
        let owned: Domain = Domain::example();
        let domain: DomainRef = DomainRef::EXAMPLE;
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "{}",
                format!("{}", owned),
                format!("{}", domain),
                "example.com",
            ),
            (
                "{:15}",
                format!("{:15}", owned),
                format!("{:15}", domain),
                "example.com    ",
            ),
            (
                "{:>15}",
                format!("{:>15}", owned),
                format!("{:>15}", domain),
                "    example.com",
            ),
            (
                "{:^15}",
                format!("{:^15}", owned),
                format!("{:^15}", domain),
                "  example.com  ",
            ),
            (
                "{:*<15}",
                format!("{:*<15}", owned),
                format!("{:*<15}", domain),
                "example.com****",
            ),
            (
                "{:.7}",
                format!("{:.7}", owned),
                format!("{:.7}", domain),
                "example",
            ),
            (
                "{:>9.7}",
                format!("{:>9.7}", owned),
                format!("{:>9.7}", domain),
                "  example",
            ),
        ];

        for (spec, owned, domain, expected) in test_cases {
            assert_eq!(owned, expected, "owned spec={}", spec);
            assert_eq!(domain, expected, "ref spec={}", spec);
        }
    }

    #[test]
    fn debug() {
        let owned: Domain = Domain::example();
        let domain: DomainRef = DomainRef::EXAMPLE;
        let test_cases: &[(&str, String, String, &str)] = &[
            (
                "{:?}",
                format!("{:?}", owned),
                format!("{:?}", domain),
                "example.com",
            ),
            (
                "{:#?}",
                format!("{:#?}", owned),
                format!("{:#?}", domain),
                "example.com",
            ),
            (
                "{:>15?}",
                format!("{:>15?}", owned),
                format!("{:>15?}", domain),
                "    example.com",
            ),
        ];

        for (spec, owned, domain, expected) in test_cases {
            assert_eq!(owned, expected, "owned spec={}", spec);
            assert_eq!(domain, expected, "ref spec={}", spec);
        }
    }

    #[test]
    fn as_ref() {
        let owned: Domain = Domain::example();
        let name: &str = owned.as_ref();
        assert_eq!(name, "example.com");

        let domain: DomainRef = DomainRef::EXAMPLE;
        let name: &str = domain.as_ref();
        assert_eq!(name, "example.com");
    }

    #[test]
    fn borrow() {
        let owned: Domain = Domain::example();
        let name: &str = owned.borrow();
        assert_eq!(name, "example.com");

        let domain: DomainRef = DomainRef::EXAMPLE;
        let name: &str = domain.borrow();
        assert_eq!(name, "example.com");
    }

    #[test]
    fn borrow_lookup() {
        let mut hashed: HashMap<Domain, u16> = HashMap::default();
        hashed.insert(Domain::example(), 443);
        hashed.insert(Domain::localhost(), 80);
        assert_eq!(hashed.get("example.com"), Some(&443));
        assert_eq!(hashed.get("localhost"), Some(&80));
        assert_eq!(hashed.get("example.org"), None);

        let mut ordered: BTreeMap<DomainRef, u16> = BTreeMap::default();
        ordered.insert(DomainRef::EXAMPLE, 443);
        ordered.insert(DomainRef::LOCALHOST, 80);
        assert_eq!(ordered.get("example.com"), Some(&443));
        assert_eq!(ordered.get("localhost"), Some(&80));
        assert_eq!(ordered.get("example.org"), None);
    }
}
