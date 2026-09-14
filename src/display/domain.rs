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
