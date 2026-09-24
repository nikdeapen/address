/// Implements `FromStr` & `TryFrom<&str>` for an owned type, delegating to its byte-slice parser.
macro_rules! impl_parse {
    ($ty:ident) => {
        impl ::std::str::FromStr for crate::$ty {
            type Err = crate::ParseError;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                Self::parse(text.as_bytes())
            }
        }

        impl TryFrom<&str> for crate::$ty {
            type Error = crate::ParseError;

            fn try_from(text: &str) -> Result<Self, Self::Error> {
                Self::parse(text.as_bytes())
            }
        }
    };
}

/// Implements `TryFrom<String>` for an owned type, delegating to its string parser.
macro_rules! impl_parse_string {
    ($ty:ident) => {
        impl TryFrom<String> for crate::$ty {
            type Error = crate::InvalidAddressError<String>;

            /// The `text` is returned unmodified in the error.
            fn try_from(text: String) -> Result<Self, Self::Error> {
                Self::parse_string(text)
            }
        }
    };
}

/// Implements `TryFrom<&str>` for a reference type, delegating to its byte-slice parser.
macro_rules! impl_parse_ref {
    ($ty:ident) => {
        impl<'a> TryFrom<&'a str> for crate::$ty<'a> {
            type Error = crate::ParseError;

            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                Self::parse(text.as_bytes())
            }
        }
    };
}

pub(crate) use impl_parse;
pub(crate) use impl_parse_ref;
pub(crate) use impl_parse_string;
