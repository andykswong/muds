<h1 align="center">〰ᗰᑌᗪᔕ〰</h1>
<h2 align="center">muds - Modular microframework for interactive data-oriented simulations</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
  <a href="https://github.com/andykswong/muds/actions/workflows/build.yaml"><img src="https://github.com/andykswong/muds/actions/workflows/build.yaml/badge.svg" alt="build" /></a>
</p>

## Overview
`muds` is a modular microframework for building interactive applications, particularly for simulations and games, in a data-oriented design.

[Data-oriented design](https://en.wikipedia.org/wiki/Data-oriented_design) promotes separation of data from business logic. In the core `muds` library, [`@muds/store`](./packages/store) provides the generic data structures to model your data store; [`@muds/event`](./packages/event) provides a way for your code logic to act on changes in the data store; and [`@muds/ioc`](./packages/ioc) makes it easy to wire up your data stores to your business logic code.

`muds` can be used with any architecture or framework. However, it is best used with a data-oriented design, like the [Entity component system](https://en.wikipedia.org/wiki/Entity_component_system) architecture. See [`@muds/store`](./packages/store) for example usage.

## API Documentation
http://andykswong.github.io/muds

## Packages

|Package|NPM|Category|Description|
|-------|---|--------|-----------|
|[`@muds/ecs`](./packages/ecs)|<a href="https://www.npmjs.com/package/@muds/ecs"><img src="https://img.shields.io/npm/v/@muds/ecs?label=@muds/ecs" alt="NPM @muds/ecs" /></a>|core|Entity-component-system library|
|[`@muds/event`](./packages/event)|<a href="https://www.npmjs.com/package/@muds/event"><img src="https://img.shields.io/npm/v/@muds/event?label=@muds/event" alt="NPM @muds/event" /></a>|core|Event signal library|
|[`@muds/ioc`](./packages/ioc)|<a href="https://www.npmjs.com/package/@muds/ioc"><img src="https://img.shields.io/npm/v/@muds/ioc?label=@muds/ioc" alt="NPM @muds/ioc" /></a>|core|Inversion of control (IoC) container|
|[`@muds/store`](./packages/store)|<a href="https://www.npmjs.com/package/@muds/store"><img src="https://img.shields.io/npm/v/@muds/store?label=@muds/store" alt="NPM @muds/store" /></a>|core|Data structure store library|
|[`@muds/node`](./packages/node)|<a href="https://www.npmjs.com/package/@muds/node"><img src="https://img.shields.io/npm/v/@muds/node?label=@muds/node" alt="NPM @muds/node" /></a>|graphics|Scene node graph library|
|[`@muds/gltf`](./packages/gltf)|<a href="https://www.npmjs.com/package/@muds/gltf"><img src="https://img.shields.io/npm/v/@muds/gltf?label=@muds/gltf" alt="NPM @muds/gltf" /></a>|graphics|glTF 2.0 file loader|
|[`mugl`](https://github.com/andykswong/mugl)|<a href="https://www.npmjs.com/package/mugl"><img src="https://img.shields.io/npm/v/mugl?label=mugl" alt="NPM mugl" /></a> |graphics|WebGL 2.0 low-level 3D graphics library|
|[`munum`](https://github.com/andykswong/munum)|<a href="https://www.npmjs.com/package/munum"><img src="https://img.shields.io/npm/v/munum?label=munum" alt="NPM munum" /></a> |graphics|3D math library|
|[`@muds/react`](./packages/react)|<a href="https://www.npmjs.com/package/@muds/react"><img src="https://img.shields.io/npm/v/@muds/react?label=@muds/react" alt="NPM @muds/react" /></a> |extension|React integration utils|

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
