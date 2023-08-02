# Contributing to AgIsoStack-rs

We warmly welcome you to AgIsoStack-rs!

Contributing to our open source repository implementing the ISOBUS (ISO11783) standard in the
agricultural industry can involve anything from adding new features, improving existing code, fixing
bugs, or even just helping to test and document the project. We greatly appreciate any contributions
you can make to help drive progress and innovation in this field. Thank you for your interest and
support!

We accept all public contributions that adhere to the [code of
conduct](https://github.com/Open-Agriculture/AgIsoStack-plus-plus/blob/main/CODE_OF_CONDUCT.md)
defined by our sibling project
[AgIsoStack++](https://github.com/Open-Agriculture/AgIsoStack-plus-plus). Additionally, for PR's we
require the pass of all automated pre-merge checks, and a manual code review by a repository
maintainer to ensure that our high code quality and project standards are maintained.

## What are our guidelines?

* Contributions must follow the usual `rustc` and `clippy` lints, and the default `rustfmt`
  settings. Exceptions to lints are allowed, but should be defined in the project's `lib.rs` file

  You can check these settings with `cargo check`, `cargo clippy`, and `cargo fmt`
* The code should compile with no warnings in the CI pipeline using `RUSTFLAGS=-Dwarnings`
* `rustdoc` documentation should compile without warnings in the CI pipeline using
  `RUSTDOCFLAGS=-Dwarnings`
* No code should be added under a more strict license than MIT, or which has not had conditions met
  to be distributed under our license
* There must be a copyright notice in every source file
* Contributions must pass the CI pipeline
* Aim for ~80% code coverage on new code, but prioritize high quality tests over gaming code
  coverage percentages

## Minimum supported Rust version

**TODO:** Define a MSRV

## Setting up a development environment

## Copyright

AgIsoStack-rs is sponsored by Raven Industries inc. as an open source project under the MIT license
started during an Innovation Sprint under the condition that Raven maintains copyright over the
project, including future contributions. See the [COPYRIGHT](./COPYRIGHT) file for details.

In addition to the `COPYRIGHT` file, every source file should include the following copyright notice

```rust
// Copyright 2023 Raven Industries, inc.
```
