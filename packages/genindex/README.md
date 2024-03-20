# genindex

[![license: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![npm](https://img.shields.io/npm/v/genindex.svg)](https://www.npmjs.com/package/genindex)
[![build](https://github.com/andykswong/muds/actions/workflows/build.yaml/badge.svg)](https://github.com/andykswong/muds/actions/workflows/build.yaml)

## Overview
`genindex` is a simple library for generational index, which is a number encoded as index + generation value. It is mainly used as a weak reference to array values. The generation part allows indices to be reused without suffering from [ABA problem](https://en.wikipedia.org/wiki/ABA_problem), so that data can be safely stored in a packed array.

## API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save genindex
```
```javascript
import { IndexF64 } from 'genindex';

// create gen ID with index = 0 and generation = 0
const id = IndexF64.create(10);

// get index value
console.log(IndexF64.indexOf(id)); // 10

// increment generation
console.log(IndexF64.next(id)); // 4294967306

```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
