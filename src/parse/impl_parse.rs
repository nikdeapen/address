/// Implements `FromStr` & `TryFrom<&str>` for an owned type, delegating to its byte-slice parser.
macro_rules! impl_parse {
    ($ty:ident) => {
        impl ::std::str::FromStr for crate::$ty {
            type Err = crate::ParseError;

            /// Parses the `text`. (see [`Self::parse`])
            fn from_str(text: &str) -> Result<Self, Self::Err> {
                Self::parse(text.as_bytes())
            }
        }

        impl TryFrom<&str> for crate::$ty {
            type Error = crate::ParseError;

            /// Parses the `text`. (see [`Self::parse`])
            fn try_from(text: &str) -> Result<Self, Self::Error> {
                Self::parse(text.as_bytes())
            }
        }
    };
}

/// Implements `TryFrom<String>` for an owned type, delegating to its byte-vector parser.
///
/// The byte-vector parser must leave the value unmodified on failure, which is what makes the
/// recovered `String` sound.
macro_rules! impl_parse_string {
    ($ty:ident) => {
        impl TryFrom<String> for crate::$ty {
            type Error = crate::InvalidAddressError<String>;

            /// Parses the `text`. (see [`Self::parse`])
            fn try_from(text: String) -> Result<Self, Self::Error> {
                let len: usize = text.len();
                Self::parse_vec(text.into_bytes())
                    .map_err(|error| unsafe { error.into_string_unchecked(len) })
            }
        }
    };
}

/// Implements `TryFrom<&str>` for a reference type, delegating to its byte-slice parser.
macro_rules! impl_parse_ref {
    ($ty:ident) => {
        impl<'a> TryFrom<&'a str> for crate::$ty<'a> {
            type Error = crate::ParseError;

            /// Parses the `text`. (see [`Self::parse`])
            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                Self::parse(text.as_bytes())
            }
        }
    };
}

pub(crate) use impl_parse;
pub(crate) use impl_parse_ref;
pub(crate) use impl_parse_string;
