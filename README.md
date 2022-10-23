<h1 align="center">〰ᗰᑌᗪᔕ〰</h1>
<h2 align="center">muds - Modular microframework for interactive data-oriented systems</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`muds` is a modular microframework for building interactive systems, like web apps, games and simulations, in a data-oriented design.

[Data-oriented design](https://en.wikipedia.org/wiki/Data-oriented_design) promotes separation of code (behavior) from data. In the core `muds` library, [`@muds/store`](./packages/store) provides the generic data structures to model your data store, and [`@muds/event`](./packages/event) provides a way for your code logic to act on changes in the data store.

Because `muds` is modular, it can be used within any architecture or framework. However, [Entity component system architecture](https://en.wikipedia.org/wiki/Entity_component_system) is recommended if you are starting a new project from scratch. See [`@muds/store`](./packages/store) for examples usage.

## Packages

|Package|NPM|Description|
|-------|---|-----------|
|[`@muds/store`](./packages/store)|<a href="https://www.npmjs.com/package/@muds/store"><img src="https://img.shields.io/npm/v/@muds/store?label=@muds/store" alt="NPM @muds/store" /></a>|Core data structure store library|
|[`@muds/event`](./packages/event)|<a href="https://www.npmjs.com/package/@muds/event"><img src="https://img.shields.io/npm/v/@muds/event?label=@muds/event" alt="NPM @muds/event" /></a>|Core event signal library|
|[`@muds/react`](./packages/react)|<a href="https://www.npmjs.com/package/@muds/react"><img src="https://img.shields.io/npm/v/@muds/react?label=@muds/react" alt="NPM @muds/react" /></a> |React integration utils|

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
