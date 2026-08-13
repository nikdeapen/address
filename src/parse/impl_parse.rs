/// Implements `FromStr` & `TryFrom<&str>` for an owned type, delegating to its byte-slice parser.
macro_rules! impl_parse {
    ($ty:ident $(, $doc:literal)?) => {
        crate::impl_parse!($ty, try_from $(, $doc)?);
    };
    ($ty:ident, $parse:ident $(, $doc:literal)?) => {
        impl ::std::str::FromStr for crate::$ty {
            type Err = crate::ParseError;

            $(#[doc = $doc])?
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::$parse(s.as_bytes())
            }
        }

        impl TryFrom<&str> for crate::$ty {
            type Error = crate::ParseError;

            $(#[doc = $doc])?
            fn try_from(s: &str) -> Result<Self, Self::Error> {
                Self::$parse(s.as_bytes())
            }
        }
    };
}

/// Implements `TryFrom<&str>` for a reference type, delegating to its byte-slice parser.
macro_rules! impl_parse_ref {
    ($ty:ident $(, $doc:literal)?) => {
        impl<'a> TryFrom<&'a str> for crate::$ty<'a> {
            type Error = crate::ParseError;

            $(#[doc = $doc])?
            fn try_from(s: &'a str) -> Result<Self, Self::Error> {
                Self::try_from(s.as_bytes())
            }
        }
    };
}

pub(crate) use impl_parse;
pub(crate) use impl_parse_ref;
