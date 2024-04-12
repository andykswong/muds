# adts

[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/adts)](https://crates.io/crates/adts)
[![docs.rs](https://img.shields.io/docsrs/adts)](https://docs.rs/adts)
[![build](https://github.com/andykswong/muds/actions/workflows/build.yaml/badge.svg)](https://github.com/andykswong/muds/actions/workflows/build.yaml)

## Overview
`adts` is a library for abstract data type traits and implementations. Currently its main features are [Map](./src/map.rs) implementations, including [genindex](https://crates.io/crates/genindex)-based optimized collections.

## Install
```toml
[dependencies]
adts = "0.2"
```
Features:
- `std` - enables `std` support. enabled by default.
- `serde` - enables `serde` serialize/deserialize implementations of collections and indices

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
