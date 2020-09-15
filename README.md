# Rust-Handwritten-JSON

[![License]](#license)
[![Travis CI]](https://travis-ci.com/yangby-cryptape/rust-handwritten-json)
[![Crate Badge]](https://crates.io/crates/handwritten-json)
 [![Crate Doc]](https://docs.rs/handwritten-json)

Convert a non-standard JSON string into a normalized JSON string.

[License]: https://img.shields.io/badge/License-Apache--2.0%20OR%20MIT-blue.svg
[Travis CI]: https://img.shields.io/travis/com/yangby-cryptape/rust-handwritten-json.svg
[Crate Badge]: https://img.shields.io/crates/v/handwritten-json.svg
[Crate Doc]: https://docs.rs/handwritten-json/badge.svg

## The Difference between Handwritten-JSON and JSON

- Allow extra commas in the end of arrays and objects.
- If a name string in a name/value pair only contains alphabetic characters,
  numeric characters and underscores, the double quotes can be omitted.

## License

Licensed under either of [Apache License, Version 2.0] or [MIT License], at
your option.

[Apache License, Version 2.0]: LICENSE-APACHE
[MIT License]: LICENSE-MIT
