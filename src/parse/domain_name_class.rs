/// The validity classification of a domain name or label.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub(crate) enum NameClass {
    /// Valid & already lowercase.
    Lowercase,

    /// Valid once uppercase letters are lowercased.
    MixedCase,

    /// Invalid even ignoring case.
    Invalid,
}
