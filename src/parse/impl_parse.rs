/// The lowercase-normalization note for the owned parse impls.
macro_rules! doc_normalized {
    () => {
        "Domain names are normalized to lowercase."
    };
}

/// The lowercase-normalization note for the owned [`Domain`](crate::Domain) parse impls.
macro_rules! doc_name_normalized {
    () => {
        "The name is normalized to lowercase."
    };
}

/// The lowercase requirement for the reference parse impls, naming the `owned` type to parse instead.
macro_rules! doc_lowercase_required {
    ($owned:ident) => {
        concat!(
            "Domain names must already be in lowercase. Use [`",
            stringify!($owned),
            "`](crate::",
            stringify!($owned),
            ") to parse mixed-case input."
        )
    };
}

/// The lowercase requirement for the [`DomainRef`](crate::DomainRef) parse impls.
macro_rules! doc_name_lowercase_required {
    () => {
        "The name must already be in lowercase. Use [`Domain`](crate::Domain) to parse mixed-case input."
    };
}

/// The ignored-zone note for the bracketed IPv6 parse impls.
macro_rules! doc_ignored_zone {
    () => {
        "A numeric IPv6 zone is accepted & ignored: `[fe80::1%1]:80` parses as `[fe80::1]:80`."
    };
}

/// The error-recovery note for the `TryFrom<Vec<u8>>` parse impls, naming the `value` parameter.
macro_rules! doc_recovers_value {
    ($value:literal) => {
        concat!(
            "The error contains the unmodified `",
            $value,
            "`, which `TryFrom<String>` soundly recovers as a string."
        )
    };
}

pub(crate) use doc_ignored_zone;
pub(crate) use doc_lowercase_required;
pub(crate) use doc_name_lowercase_required;
pub(crate) use doc_name_normalized;
pub(crate) use doc_normalized;
pub(crate) use doc_recovers_value;

/// Implements `FromStr` & `TryFrom<&str>` for an owned type, delegating to its byte-slice parser.
///
/// The explicit-parser arm comes first so the defaulted arm never captures a parser name as a doc fragment.
macro_rules! impl_parse {
    ($ty:ident, $parse:ident $(, $doc:expr)*) => {
        impl ::std::str::FromStr for crate::$ty {
            type Err = crate::ParseError;

            $(#[doc = $doc])*
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::$parse(s.as_bytes())
            }
        }

        impl TryFrom<&str> for crate::$ty {
            type Error = crate::ParseError;

            $(#[doc = $doc])*
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::$parse(s.as_bytes())
            }
        }
    };
    ($ty:ident $(, $doc:expr)*) => {
        crate::impl_parse!($ty, try_from $(, $doc)*);
    };
}

/// Implements `TryFrom<String>` for an owned type, delegating to its byte-vector parser.
///
/// The byte-vector parser must leave the value unmodified on failure, which is what makes the recovered
/// `String` sound.
macro_rules! impl_parse_string {
    ($ty:ident $(, $doc:expr)*) => {
        impl TryFrom<String> for crate::$ty {
            type Error = crate::InvalidAddress<String>;

            $(#[doc = $doc])*
            fn try_from(value: String) -> Result<Self, Self::Error> {
                let len: usize = value.len();
                Self::try_from(value.into_bytes()).map_err(|error| unsafe { error.into_string_unchecked(len) })
            }
        }
    };
}

/// Implements `TryFrom<&str>` for a reference type, delegating to its byte-slice parser.
macro_rules! impl_parse_ref {
    ($ty:ident $(, $doc:expr)*) => {
        impl<'a> TryFrom<&'a str> for crate::$ty<'a> {
            type Error = crate::ParseError;

            $(#[doc = $doc])*
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                Self::try_from(s.as_bytes())
            }
        }
    };
}

pub(crate) use impl_parse;
pub(crate) use impl_parse_ref;
pub(crate) use impl_parse_string;
