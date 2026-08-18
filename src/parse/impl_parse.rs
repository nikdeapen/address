/// Implements `FromStr` & `TryFrom<&str>` for an owned type, delegating to its byte-slice parser.
macro_rules! impl_parse {
    ($ty:ident $(, $doc:expr)*) => {
        impl ::std::str::FromStr for crate::$ty {
            type Err = crate::ParseError;

            $(#[doc = $doc])*
            fn from_str(text: &str) -> Result<Self, Self::Err> {
                Self::parse_text(text.as_bytes())
            }
        }

        impl TryFrom<&str> for crate::$ty {
            type Error = crate::ParseError;

            $(#[doc = $doc])*
            fn try_from(text: &str) -> Result<Self, Self::Error> {
                Self::parse_text(text.as_bytes())
            }
        }
    };
}

/// Implements `TryFrom<String>` for an owned type, delegating to its byte-vector parser.
///
/// The byte-vector parser must leave the value unmodified on failure, which is what makes the recovered `String` sound.
macro_rules! impl_parse_string {
    ($ty:ident $(, $doc:expr)*) => {
        impl TryFrom<String> for crate::$ty {
            type Error = crate::InvalidAddressError<String>;

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
            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                Self::parse_text(text.as_bytes())
            }
        }
    };
}

pub(crate) use impl_parse;
pub(crate) use impl_parse_ref;
pub(crate) use impl_parse_string;
