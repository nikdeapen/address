pub use domain_invalid_domain_name::*;
pub use invalid_address::*;
pub use parse_error::*;

pub(crate) use impl_parse::*;
pub(crate) use parse_port::*;
pub(crate) use strip_brackets::*;
pub(crate) use strip_zone::*;

mod domain_invalid_domain_name;
mod impl_parse;
mod invalid_address;
mod parse_error;
mod parse_port;
mod strip_brackets;
mod strip_zone;

mod authority;
mod authority_ref;
mod domain;
mod domain_ref;
mod domain_validation;
mod endpoint;
mod endpoint_ref;
mod host;
mod host_ref;
mod ip_address;
mod ipv4_address;
mod ipv6_address;
mod socket_address;
mod socket_address_v4;
mod socket_address_v6;

#[cfg(feature = "idna")]
mod domain_idna;
