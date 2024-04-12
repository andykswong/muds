# genindex

[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/genindex)](https://crates.io/crates/genindex)
[![docs.rs](https://img.shields.io/docsrs/genindex)](https://docs.rs/genindex)
[![build](https://github.com/andykswong/muds/actions/workflows/build.yaml/badge.svg)](https://github.com/andykswong/muds/actions/workflows/build.yaml)

## Overview
`genindex` is a no-std library for generational index, which is a number encoded as index + generation value. It is mainly used as a weak reference to array values. The generation part allows indices to be reused without suffering from [ABA problem](https://en.wikipedia.org/wiki/ABA_problem), so that data can be safely stored in a packed array.

## Install
```toml
[dependencies]
genindex = "0.2"
```
Features:
- `serde` - enables `serde` serialize/deserialize implementations of collections and indices

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
