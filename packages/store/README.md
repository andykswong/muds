# @muds/store
<p align="center">
  <a href="https://www.npmjs.com/package/@muds/store"><img src="https://img.shields.io/npm/v/@muds/store?label=@muds/store" alt="NPM @muds/store" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`@muds/store` is the core data structure store library for `muds`, the modular microframework for interactive data-oriented systems. This package is what enables `muds` to be data-oriented. It provides useful data structures to model your data store and structure your app in [Entity component system (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) architecture.

# API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save @muds/store
```

`@muds/store` is best used with an ECS architecture. `IdGenerator` can be used to generate entity IDs, and component data can be stored in maps:
```javascript
import { IdGenerator, SparseSetMap, uniqueJoin } from '@muds/store';

// Entities are simply IDs
const entityIdGenerator = new IdGenerator();

// Components are stored in maps
// SparseSetMap is good for often used components. for rare components, use GenIdMap instead
const positions = new SparseSetMap();
const velocities = new SparseSetMap();

// Systems are simply functions that operates on the data model (entity and components)
function PhysicsSystem(deltaTimeSec) {
  // Use uniqueJoin(iter, keyFn, ...maps) to perform multi-component left-join queries
  const query = uniqueJoin(velocities.entries(), ([id, vel] => id), positions);

  for ([entityId, vel, pos = [0, 0]] of query) {
    const newPos = [pos[0] + vel[0] * deltaTimeSec, pos[1] + vel[1] * deltaTimeSec];
    positions.set(entityId, newPos);
  }
}

// Create some entities
const entity1 = entityIdGenerator.add();
positions.set(entity1, [10, 10]);
velocities.set(entity1, [1, 2]);
const entity2 = entityIdGenerator.add();
// position of entity2 defaults to [0, 0]
velocities.set(entity2, [3, 4]);

PhysicsSystem(1); // Run the system
// Current positions = { [entity1]: [11, 12], [entity2]: [3, 4] }
```

Observable collections are available for reacting on collection change events. There is also a `UniqueIndex` type that automatically builds a unique key index by listening to an observable map:
```javascript
import { GenIdMap, IdGenerator, ObservableMap, UniqueIndex, onCollectionChange } from '@muds/store';

const entityIdGenerator = new IdGenerator();
const components = new ObservableMap(new GenIdMap());

// You can listen to components.onAdd/onClear/onDelete/onUpdate events
components.onAdd.addListener((components, entityId, value) => console.log(`added component key = ${entityId}, value = ${component}`));

const onChange = onCollectionChange(components); // To listen to all collection event types at once
onChange.addListener((eventType, components, entityId, value, oldValue) => ...);

// Create a reverse mapping index from component value to entityId
const index = UniqueIndex.fromCollection(components, (entityId, value) => value);

const entity1 = entityIdGenerator.add();
components.set(entity1, 1); // This triggers components.onAdd, onChange events and updates index

console.assert(entity1 === index.get(1)); // true
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
