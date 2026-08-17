use crate::{Domain, NameClass};

impl Domain {
    //! Label Validation

    /// The maximum length of a domain label.
    pub const MAX_LABEL_LEN: usize = 63;

    /// Classifies the domain `label`.
    ///
    /// The accepted bytes are ASCII, so a non-`Invalid` class proves the label is valid UTF-8. The parse impls
    /// rely on that to convert classified bytes without re-validating; widening the byte set here would make
    /// those conversions unsound.
    pub(crate) fn classify_label(label: &[u8]) -> NameClass {
        if (label.is_empty() || label.len() > Self::MAX_LABEL_LEN)
            || (label[0] == b'-' || label[label.len() - 1] == b'-')
        {
            NameClass::Invalid
        } else {
            let mut class: NameClass = NameClass::Lowercase;
            for c in label {
                if c.is_ascii_uppercase() {
                    class = NameClass::MixedCase;
                } else if !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-') {
                    return NameClass::Invalid;
                }
            }
            class
        }
    }

    /// Checks if the domain `label` is valid.
    ///
    /// A valid label is 1 to 63 (`MAX_LABEL_LEN`) bytes of ASCII lowercase letters, digits, and dashes and must not
    /// start or end with a dash. Uppercase letters are also valid with `ignore_case`.
    #[must_use]
    pub fn is_valid_label(label: &[u8], ignore_case: bool) -> bool {
        match Self::classify_label(label) {
            NameClass::Lowercase => true,
            NameClass::MixedCase => ignore_case,
            NameClass::Invalid => false,
        }
    }

    /// Checks if the domain `label` is valid.
    #[must_use]
    pub fn is_valid_label_str(label: &str, ignore_case: bool) -> bool {
        Self::is_valid_label(label.as_bytes(), ignore_case)
    }
}

impl Domain {
    //! Domain Validation

    /// The maximum length of a domain name.
    pub const MAX_NAME_LEN: usize = 253;

    /// Classifies the domain `name`.
    pub(crate) fn classify_name(name: &[u8]) -> NameClass {
        if name.is_empty() || name.len() > Self::MAX_NAME_LEN {
            NameClass::Invalid
        } else {
            let mut class: NameClass = NameClass::Lowercase;
            for label in name.split(|c| *c == b'.') {
                match Self::classify_label(label) {
                    NameClass::Invalid => return NameClass::Invalid,
                    NameClass::MixedCase => class = NameClass::MixedCase,
                    NameClass::Lowercase => {}
                }
            }
            class
        }
    }

    /// Checks if the domain `name` is valid.
    ///
    /// A valid name is 1 to 253 (`MAX_NAME_LEN`) bytes of dot-separated valid labels. Labels cannot be empty, so
    /// leading, trailing, and consecutive dots are invalid. Labels may be entirely numeric. Underscores are invalid,
    /// so service names like `_sip._tcp.example.com` cannot be represented. Valid names are ASCII, so they are
    /// always valid UTF-8. Uppercase letters are also valid with `ignore_case`.
    #[must_use]
    pub fn is_valid_name(name: &[u8], ignore_case: bool) -> bool {
        match Self::classify_name(name) {
            NameClass::Lowercase => true,
            NameClass::MixedCase => ignore_case,
            NameClass::Invalid => false,
        }
    }

    /// Checks if the domain `name` is valid.
    #[must_use]
    pub fn is_valid_name_str(name: &str, ignore_case: bool) -> bool {
        Self::is_valid_name(name.as_bytes(), ignore_case)
    }
}

#[cfg(test)]
mod tests {
    use crate::Domain;

    #[test]
    fn is_valid_label() {
        let test_cases: &[(&str, bool, bool)] = &[
            ("", false, false),
            ("09", true, true),
            ("az", true, true),
            ("AZ", false, true),
            ("-a", false, false),
            ("a-", false, false),
            ("a--a", true, true),
            ("a-a", true, true),
            ("a-a-a", true, true),
        ];
        for (label, expected, expected_ignore_case) in test_cases {
            let result: bool = Domain::is_valid_label_str(label, false);
            assert_eq!(result, *expected, "label={}", label);

            let result: bool = Domain::is_valid_label_str(label, true);
            assert_eq!(result, *expected_ignore_case, "label={}", label);
        }
    }

    #[test]
    fn label_length_boundaries() {
        let test_cases: &[(usize, bool)] = &[(Domain::MAX_LABEL_LEN, true), (Domain::MAX_LABEL_LEN + 1, false)];

        for (len, expected) in test_cases {
            let label: String = "a".repeat(*len);
            let result: bool = Domain::is_valid_label_str(label.as_str(), false);
            assert_eq!(result, *expected, "len={}", len);
        }
    }

    #[test]
    fn is_valid_name() {
        let test_cases: &[(&str, bool, bool)] = &[
            ("", false, false),
            ("09", true, true),
            ("az", true, true),
            ("AZ", false, true),
            (".a", false, false),
            ("a.", false, false),
            ("a..a", false, false),
            ("a.a", true, true),
            ("a.a.a", true, true),
            ("a-a.a-a.a-a", true, true),
        ];
        for (name, expected, expected_ignore_case) in test_cases {
            let result: bool = Domain::is_valid_name_str(name, false);
            assert_eq!(result, *expected, "name={}", name);

            let result: bool = Domain::is_valid_name_str(name, true);
            assert_eq!(result, *expected_ignore_case, "name={}", name);
        }
    }

    #[test]
    fn name_length_boundaries() {
        let test_cases: &[(usize, usize, bool)] =
            &[(61, Domain::MAX_NAME_LEN, true), (62, Domain::MAX_NAME_LEN + 1, false)];

        for (tail_len, expected_len, expected) in test_cases {
            let label: String = "a".repeat(Domain::MAX_LABEL_LEN);
            let name: String = format!("{}.{}.{}.{}", label, label, label, "a".repeat(*tail_len));
            assert_eq!(name.len(), *expected_len, "tail_len={}", tail_len);

            let result: bool = Domain::is_valid_name_str(name.as_str(), false);
            assert_eq!(result, *expected, "tail_len={}", tail_len);
        }
    }
}
