# Issues

## Cargo

- Decide whether to declare an MSRV: add `rust-version` to `Cargo.toml`, measure it, and enforce it in CI.

## Formatting

- Decide whether the Markdown should wrap at 100; `README.md` & `ISSUES.md` are still at 120.

## Portability

- Decide whether to support `no_std` + `alloc`: the crate uses nothing std-only.

## API

- Decide whether `TryFrom<Vec<u8>>` should become a named parser like `parse_text`.
- Decide whether to impl `std::net::ToSocketAddrs` for `Authority` & `AuthorityRef`.
- Return `Result<_, Self>` from `Authority::to_endpoint` & `Host::to_domain`; they consume the value & drop it.
- Decide whether the `idna` feature should extend `Host`, `Endpoint`, & `Authority`; only `Domain` parses Unicode.
- Reconcile the `Host` variant names (`Name` & `Address`) with the `is_domain` & `is_ip` method names.

## Parsing

- Extract the shared host classification; `Authority::parse_text` & `TryFrom<Vec<u8>>` each state the parse order.

## Validation

- Decide whether `Domain` should reject an all-numeric final label (e.g. `999.1.1.1`).
- Decide whether `Domain` should reject IPv4 text (e.g. `127.0.0.1`).
- Make the validation helpers private; only `classify_name` is used outside `validation.rs`.

## Testing

- Decide whether to gate CI on `cargo miri test`; it passes today under `-Zmiri-strict-provenance`.
- Get rid of pedantic tests like testing display formatting.
- Consolidate each type's parse cases into one table looped through every entry point; they now differ per path.

## Performance

- Serialize `Domain`, `DomainRef`, & the `Host` `Name` variants with `serialize_str` instead of `collect_str`.
- Revisit the `parse` byte -> str conversions with `[u8]::as_ascii()` when nightly `ascii_char` (rust#110998).
