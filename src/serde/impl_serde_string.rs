/// Implements `Serialize` and `Deserialize` for an owned type that serializes as its `Display` string.
macro_rules! impl_serde_string {
    ($ty:ident, $expecting:literal) => {
        impl ::serde::Serialize for crate::$ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.collect_str(self)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for crate::$ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer.deserialize_string(crate::serde::FromStringVisitor::new($expecting))
            }
        }
    };
}

/// Implements `Serialize` and `Deserialize` for a reference type that serializes as its `Display` string.
macro_rules! impl_serde_string_ref {
    ($ty:ident, $owned:ident, $expecting:literal) => {
        impl<'a> ::serde::Serialize for crate::$ty<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.collect_str(self)
            }
        }

        impl<'de: 'a, 'a> ::serde::Deserialize<'de> for crate::$ty<'a> {
            #[doc = concat!(
                "The string is borrowed from the input, so domain names must be lowercase and must not contain ",
                "escape sequences. Use [`",
                stringify!($owned),
                "`](crate::",
                stringify!($owned),
                ") to deserialize mixed-case or escaped input."
            )]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer.deserialize_str(crate::serde::TryFromStrVisitor::new($expecting))
            }
        }
    };
}

impl_serde_string!(Authority, "an authority string");
impl_serde_string_ref!(AuthorityRef, Authority, "a borrowed authority string");

impl_serde_string!(Domain, "a domain string");
impl_serde_string_ref!(DomainRef, Domain, "a borrowed domain string");

impl_serde_string!(Endpoint, "an endpoint string");
impl_serde_string_ref!(EndpointRef, Endpoint, "a borrowed endpoint string");

impl_serde_string!(Host, "a host string");
impl_serde_string_ref!(HostRef, Host, "a borrowed host string");

#[cfg(test)]
mod tests {
    use crate::serde::test_util::{assert_json, assert_postcard};
    use crate::{
        Authority, AuthorityRef, Domain, DomainRef, Endpoint, EndpointRef, Host, HostRef, IPv4Address, IPv6Address,
    };

    #[test]
    fn json() {
        assert_json(Domain::localhost(), "\"localhost\"");
        assert_json(Domain::example(), "\"example.com\"");
        assert_json(Domain::localhost().to_endpoint(80), "\"localhost:80\"");
        assert_json(Domain::localhost().to_host(), "\"localhost\"");
        assert_json(IPv4Address::LOCALHOST.to_host(), "\"127.0.0.1\"");
        assert_json(Domain::localhost().to_host().to_authority(80), "\"localhost:80\"");
        assert_json(IPv6Address::LOCALHOST.to_host().to_authority(80), "\"[::1]:80\"");
    }

    /// Domain-bearing types are strings in every format, binary included.
    #[test]
    fn postcard() {
        let bytes: Vec<u8> = assert_postcard(Domain::localhost());
        assert_eq!(bytes.len(), 10, "a length prefix plus the 9 name bytes");

        assert_postcard(Domain::example().to_endpoint(443));
        assert_postcard(Domain::example().to_host());
        assert_postcard(IPv4Address::LOCALHOST.to_host().to_authority(80));
    }

    #[test]
    fn borrowed_refs() {
        let json: &str = "\"localhost\"";
        let domain: DomainRef = serde_json::from_str(json).unwrap();
        assert_eq!(domain, DomainRef::LOCALHOST);
        assert_eq!(serde_json::to_string(&domain).unwrap(), json);

        let json: &str = "\"localhost:80\"";
        let endpoint: EndpointRef = serde_json::from_str(json).unwrap();
        assert_eq!(endpoint, EndpointRef::new(DomainRef::LOCALHOST, 80));
        assert_eq!(serde_json::to_string(&endpoint).unwrap(), json);

        let json: &str = "\"127.0.0.1\"";
        let host: HostRef = serde_json::from_str(json).unwrap();
        assert_eq!(host, IPv4Address::LOCALHOST.to_host_ref());
        assert_eq!(serde_json::to_string(&host).unwrap(), json);

        let json: &str = "\"[::1]:443\"";
        let authority: AuthorityRef = serde_json::from_str(json).unwrap();
        assert_eq!(authority, IPv6Address::LOCALHOST.to_host_ref().to_authority_ref(443));
        assert_eq!(serde_json::to_string(&authority).unwrap(), json);
    }

    /// The owned types normalize mixed case; the reference types cannot, because they borrow the input.
    #[test]
    fn mixed_case() {
        let domain: Domain = serde_json::from_str("\"LocalHost\"").unwrap();
        assert_eq!(domain, Domain::localhost());

        let authority: Authority = serde_json::from_str("\"LocalHost:80\"").unwrap();
        assert_eq!(authority, Domain::localhost().to_host().to_authority(80));

        assert!(serde_json::from_str::<DomainRef>("\"LocalHost\"").is_err());
        assert!(serde_json::from_str::<AuthorityRef>("\"LocalHost:80\"").is_err());
    }

    /// `h` is `h`: the owned types accept escaped input, the reference types cannot borrow through it.
    #[test]
    fn escaped_input() {
        let json: &str = "\"local\\u0068ost\"";

        let domain: Domain = serde_json::from_str(json).unwrap();
        assert_eq!(domain, Domain::localhost());

        let host: Host = serde_json::from_str(json).unwrap();
        assert_eq!(host, Domain::localhost().to_host());

        assert!(serde_json::from_str::<DomainRef>(json).is_err());
        assert!(serde_json::from_str::<HostRef>(json).is_err());
    }

    #[test]
    fn invalid_input() {
        assert!(serde_json::from_str::<Domain>("\"Local!Host\"").is_err());
        assert!(serde_json::from_str::<Endpoint>("\"localhost\"").is_err());
        assert!(serde_json::from_str::<Authority>("\"::1:80\"").is_err());
        assert!(serde_json::from_str::<Domain>("42").is_err());
    }

    /// The `expecting` message names the type, so decoder errors stay legible.
    #[test]
    fn expecting_message() {
        let error: String = serde_json::from_str::<Domain>("42").unwrap_err().to_string();
        assert!(error.contains("a domain string"), "error={}", error);

        let error: String = serde_json::from_str::<AuthorityRef>("42").unwrap_err().to_string();
        assert!(error.contains("a borrowed authority string"), "error={}", error);
    }
}
