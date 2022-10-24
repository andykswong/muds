# @muds/node
<p align="center">
  <a href="https://www.npmjs.com/package/@muds/node"><img src="https://img.shields.io/npm/v/@muds/node?label=@muds/node" alt="NPM @muds/node" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`@muds/node` is a minimal 2D/3D scene node graph library for [muds](https://github.com/andykswong/muds), a modular microframework for interactive data-oriented systems.

# API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save @muds/node
```

To create a node hierarchy:
```javascript
import { Node, setParent, preorderTraverse } from '@muds/node';

const nodes = new Map();

const root = 1, child = 2;
nodes.set(child, new Node());
nodes.set(root, new Node());
setParent(nodes, child, root);
console.assert(nodes.get(root).lastChild === child && nodes.get(child).parent === root);

for (const [nodeId, node] of preorderTraverse(nodes, root)) {
  console.log(nodeId);
}
// prints: 1 2
```


Using the 3D transform component:
```javascript
import { Transform } from '@muds/node';

const transform = new Transform();
transform.translation = [1, 2, 3]; // x-y-z translation
transform.rotation = [0, 0, 0, 1]; // quaternion rotation
transform.scale = [5, 6, 7]; // x-y-z scaling

transform.update(true); // if false (default), it sets transform.needUpdate = true only without updating the matrix.
console.log(transform.matrix); // prints the 4x4 local transformation matrix = T * R * S
```

To create a 3D transform hierarchy and calculate world transformations:
```javascript
import { Node, Transform, setParent, updateWorldTransforms } from '@muds/node';

const nodes = new Map();
const transforms = new Map();
const worldTransforms /* : Map<number, Mat4> type */ = new Map();

const root = 1, child = 2;
nodes.set(root, new Node());
transforms.set(root, new Transform());
nodes.set(child, new Node());
transforms.set(child, new Transform());
setParent(nodes, child, root);
// set root/child transform logic omitted

updateWorldTransforms(nodes, transforms, worldTransforms, root);
console.log(worldTransforms.get(child)); // prints the 4x4 world matrix = (parent transform) * (child transform)
```

There are also 2D versions of above and the APIs are similar except for being 2D:
```javascript
import { Transform2D, updateWorldTransforms2D } from '@muds/node';
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
