pub(in crate::serde) use from_str_visitor::*;
pub(in crate::serde) use from_string_visitor::*;
pub(in crate::serde) use try_from_str_visitor::*;

mod from_str_visitor;
mod from_string_visitor;
mod try_from_str_visitor;

mod impl_serde_string;
mod impl_serde_string_or_binary;
mod ip_address;

#[cfg(test)]
pub(in crate::serde) mod test_util {
    use serde::Serialize;
    use serde::de::DeserializeOwned;
    use std::fmt::Debug;

    /// Asserts the value serializes to the `expected` JSON & deserializes back to an equal value.
    pub(in crate::serde) fn assert_json<T>(value: T, expected: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let json: String = serde_json::to_string(&value).unwrap();
        assert_eq!(json, expected, "value={:?}", value);

        let parsed: T = serde_json::from_str(json.as_str()).unwrap();
        assert_eq!(parsed, value, "value={:?}", value);
    }

    /// Asserts the value survives a postcard round trip & returns its encoded bytes.
    pub(in crate::serde) fn assert_postcard<T>(value: T) -> Vec<u8>
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let bytes: Vec<u8> = postcard::to_allocvec(&value).unwrap();

        let parsed: T = postcard::from_bytes(bytes.as_slice()).unwrap();
        assert_eq!(parsed, value, "value={:?}", value);

        bytes
    }
}
