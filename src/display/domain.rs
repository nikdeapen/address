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

    #[test]
    fn domain_display() {
        let domain: Domain = Domain::localhost();
        let result: String = domain.to_string();
        let expected: &str = "localhost";
        assert_eq!(result, expected);
    }

    #[test]
    fn as_str() {
        let owned: Domain = Domain::localhost();
        let result: &str = owned.as_ref();
        assert_eq!(result, "localhost");

        let domain: DomainRef = DomainRef::LOCALHOST;
        let result: &str = domain.as_ref();
        assert_eq!(result, "localhost");
    }

    #[test]
    fn display_spec() {
        let domain: Domain = Domain::localhost();
        assert_eq!(format!("{:>12}", domain), "   localhost");
        assert_eq!(format!("{:<12}|", domain), "localhost   |");
        assert_eq!(format!("{:*^13}", domain), "**localhost**");
        assert_eq!(format!("{:.5}", domain), "local");

        let domain: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(format!("{:>12}", domain), "   localhost");
        assert_eq!(format!("{:.5}", domain), "local");
    }
}
