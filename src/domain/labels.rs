use std::iter::FusedIterator;
use std::str::Split;

/// An iterator over the labels of a domain name.
#[must_use]
#[derive(Clone, Debug)]
pub struct Labels<'a> {
    iter: Split<'a, char>,
}

impl<'a> Labels<'a> {
    //! Construction

    /// Creates a new label iterator for the domain `name`.
    pub(crate) fn new(name: &'a str) -> Self {
        Self { iter: name.split('.') }
    }
}

impl<'a> Iterator for Labels<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Labels<'a> {
    fn next_back(&mut self) -> Option<&'a str> {
        self.iter.next_back()
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
