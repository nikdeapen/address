use crate::{Domain, DomainRef};
use std::iter::FusedIterator;

impl Domain {
    //! Labels

    /// Gets the labels.
    pub const fn labels(&self) -> Labels<'_> {
        Labels::new(self.name.as_str())
    }
}

impl<'a> DomainRef<'a> {
    //! Labels

    /// Gets the labels.
    pub const fn labels(self) -> Labels<'a> {
        Labels::new(self.name)
    }
}

/// An iterator over the labels of a domain name.
#[must_use]
#[derive(Copy, Clone, Debug)]
pub struct Labels<'a> {
    name: Option<&'a str>,
}

impl<'a> Labels<'a> {
    //! Construction

    /// Creates a new label iterator for the domain `name`.
    pub(crate) const fn new(name: &'a str) -> Self {
        Self { name: Some(name) }
    }
}

impl<'a> Iterator for Labels<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let name: &str = self.name?;
        if let Some(dot) = name.find('.') {
            self.name = Some(&name[dot + 1..]);
            Some(&name[..dot])
        } else {
            self.name = None;
            Some(name)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.name {
            // Labels are at least 1 byte & dot-separated, so at most `len / 2 + 1` labels remain.
            Some(name) => (1, Some(name.len() / 2 + 1)),
            None => (0, Some(0)),
        }
    }
}

impl<'a> DoubleEndedIterator for Labels<'a> {
    fn next_back(&mut self) -> Option<&'a str> {
        let name: &str = self.name?;
        if let Some(dot) = name.rfind('.') {
            self.name = Some(&name[..dot]);
            Some(&name[dot + 1..])
        } else {
            self.name = None;
            Some(name)
        }
    }
}

impl<'a> FusedIterator for Labels<'a> {}

#[cfg(test)]
mod tests {
    use crate::{Domain, DomainRef};

    #[test]
    fn labels() {
        let test_cases: &[(&str, &[&str])] = &[
            ("localhost", &["localhost"]),
            ("example.com", &["example", "com"]),
            ("www.example.com", &["www", "example", "com"]),
        ];

        for (name, expected) in test_cases {
            let domain: Domain = name.parse().unwrap();
            let result: Vec<&str> = domain.labels().collect();
            assert_eq!(result, *expected, "name={}", name);

            let result: Vec<&str> = domain.to_ref().labels().collect();
            assert_eq!(result, *expected, "name={}", name);
        }
    }

    #[test]
    fn labels_rev() {
        let domain: DomainRef = DomainRef::EXAMPLE;
        let result: Vec<&str> = domain.labels().rev().collect();
        let expected: Vec<&str> = vec!["com", "example"];
        assert_eq!(result, expected);
    }
}
