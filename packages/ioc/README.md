# @muds/ioc
<p align="center">
  <a href="https://www.npmjs.com/package/@muds/ioc"><img src="https://img.shields.io/npm/v/@muds/ioc?label=@muds/ioc" alt="NPM @muds/ioc" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`@muds/ioc` is a minimal inversion of control (IoC) container library for [muds](https://github.com/andykswong/muds), a modular microframework for interactive data-oriented systems.

In `@muds/ioc`, the IoC container contents are defined by module classes, which declares provider functions for injectables. Modules are easily reusable, interchangeable and testable. Unlike many other JavaScript/TypeScript IoC libraries, `@muds/ioc` does not require you to add decorators nor make any change to the injectable classes, makes it easy to integrate with third-party libraries.

## API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save @muds/ioc
```

The library requires the use of TypeScript [decorators](https://www.typescriptlang.org/docs/handbook/decorators.html), or [Babel legacy decorator](https://babeljs.io/docs/en/babel-plugin-proposal-decorators#legacy) if using pure JavaScript:
```javascript
import { Container, inject, module, multi, provide, singleton, tagged } from '@muds/ioc';

const id0 = Symbol('id0'), id1 = Symbol('id1'), id2 = Symbol('id2');

// 1. declare modules
@module()
class Module {
  // each provider method must be decorated with @provide(id). they can return any value
  @provide(id0)
  getFoo() { return 'foo'; }

  // singletons are cached once created
  // use @inject to inject a dependency by id
  @provide(id1) @singleton 
  getBar(@inject(id0) foo) {
    return { bar: foo };
  }

  // multiple providers with same id is supported
  // add tags using @tagged to differentiate them
  @provide(id1) @tagged({ type: 2 }) 
  getBar2() { return { bar: 2 }; }

  // use @multi to inject an array of injectables with same id
  @provide(id2)
  getBars(@inject(id1) @multi bars) { return bars; }
}

// 2. initialize IoC container and add modules
const container = Container.create();
container.add(new Module());

// 3. use get(id, tags?) or multiGet(id, tags?) to get injectables
container.get(id0); // 'foo'
container.multiGet(id1); // [{ bar: 'foo' }, { bar: 2 }]
container.get(id2); // [{ bar: 'foo' }, { bar: 2 }]
container.get(id1, { type: 2 }); // { bar: 2 }
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
