use crate::{Domain, DomainRef};

impl Domain {
    //! Conversions

    /// Converts the domain to a domain reference.
    pub fn to_ref(&self) -> DomainRef<'_> {
        unsafe { DomainRef::new(self.name()) }
    }
}

impl<'a> DomainRef<'a> {
    //! Conversions

    /// Converts the domain reference to a domain.
    pub fn to_domain(&self) -> Domain {
        unsafe { Domain::new(self.name()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef};

    #[test]
    fn domain_to_ref() {
        let domain: Domain = Domain::localhost();

        let result: DomainRef = domain.to_ref();
        let expected: DomainRef = DomainRef::LOCALHOST;
        assert_eq!(result, expected);
    }

    #[test]
    fn ref_to_domain() {
        let domain: DomainRef = DomainRef::LOCALHOST;

        let result: Domain = domain.to_domain();
        let expected: Domain = Domain::localhost();
        assert_eq!(result, expected);
    }
}
