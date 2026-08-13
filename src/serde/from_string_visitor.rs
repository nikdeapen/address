use serde::de::{Error, Unexpected, Visitor};
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

/// A serde visitor that parses a string with `FromStr`, reusing the buffers of owned strings and byte vectors
/// with the consuming `TryFrom` conversions.
pub(crate) struct FromStringVisitor<T: FromStr + TryFrom<String> + TryFrom<Vec<u8>>> {
    expecting: &'static str,
    phantom: PhantomData<fn() -> T>,
}

impl<T: FromStr + TryFrom<String> + TryFrom<Vec<u8>>> FromStringVisitor<T> {
    //! Construction

    /// Creates a new visitor with the `expecting` message.
    pub(crate) const fn new(expecting: &'static str) -> Self {
        Self {
            expecting,
            phantom: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FromStringVisitor<T>
where
    T: FromStr + TryFrom<String> + TryFrom<Vec<u8>>,
    <T as FromStr>::Err: Display,
    <T as TryFrom<String>>::Error: Display,
    <T as TryFrom<Vec<u8>>>::Error: Display,
{
    type Value = T;

    fn expecting(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.expecting)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        T::from_str(v).map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        T::try_from(v).map_err(E::custom)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match std::str::from_utf8(v) {
            Ok(s) => self.visit_str(s),
            Err(_) => Err(E::invalid_value(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        T::try_from(v).map_err(E::custom)
    }
}
