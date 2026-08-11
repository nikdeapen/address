/// Strips the surrounding brackets from the `address`.
///
/// Returns `None` if the address is not bracketed.
///
/// # Examples
/// `[::1]`   -> `Some("::1")`
/// `[]`      -> `Some("")`
/// `::1`     -> `None`
/// `[::1`    -> `None`
pub(crate) fn strip_brackets(address: &[u8]) -> Option<&[u8]> {
    if !address.is_empty() && address[0] == b'[' && address[address.len() - 1] == b']' {
        Some(&address[1..address.len() - 1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::strip_brackets;

    #[test]
    fn brackets() {
        let test_cases: &[(&str, Option<&str>)] = &[
            ("[::1]", Some("::1")),
            ("[]", Some("")),
            ("::1", None),
            ("[::1", None),
            ("::1]", None),
            ("", None),
            ("[", None),
        ];

        for (input, expected) in test_cases {
            let result: Option<&[u8]> = strip_brackets(input.as_bytes());
            let expected: Option<&[u8]> = expected.map(str::as_bytes);
            assert_eq!(result, expected, "input={}", input);
        }
    }
}
