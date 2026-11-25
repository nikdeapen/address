# Contributing

This guide outlines how contributions should be made to the codebase.

## Pull Requests

The `master` branch is protected, so every change is made on a branch & merged with a pull request.

- Each pull request is one coherent change.
- The build workflow must pass. It runs the same checks as:

```sh
cargo fmt --all -- --check
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

## Commits

Each commit is one coherent change. Its message is a tag & one sentence.

```text
[domain] Added support for IDNs.
```

There shouldn't be a description.

### Tags

The tag denotes the part of the codebase the commit is relevant to.

- `[cargo]`: the project, typically anything focused outside of `src/`.
- `[ip]`: the IP address types.
- `[socket]`: the socket address types.
- `[domain]`: the domain types.
- `[endpoint]`: the endpoint types.
- `[host]`: the host types.
- `[authority]`: the authority types.
- `[parse]`: the parsing & validation of every address type.
- `[serde]`: the `serde` implementations & the wire contract.
- `[*]`: a change that spans many of the above.

A change that spans two of the above can use both tags, like `[ip][socket]`.

## Versions

- Breaking changes bump the minor version while the crate is below `1.0`.
