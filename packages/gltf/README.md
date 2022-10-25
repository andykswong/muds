# @muds/gltf
<p align="center">
  <a href="https://www.npmjs.com/package/@muds/gltf"><img src="https://img.shields.io/npm/v/@muds/gltf?label=@muds/gltf" alt="NPM @muds/gltf" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`@muds/gltf` is the minimal glTF 2.0 file loader for [muds](https://github.com/andykswong/muds), a modular microframework for interactive data-oriented systems.

## API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save @muds/gltf
```

To load a glTF 2.0 model (both .gltf and .glb formats are supported):
```typescript
import { loadGlTF, glTF2 } from '@muds/gltf';

const uri = 'https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/BoxTextured/glTF-Binary/BoxTextured.glb';
const {
  glTF, // glTF model in JSON
  buffers, // array of Uint8Array for buffers declared in the model
  images // array of Image for images declared in the model
} = loadGlTF(uri);

// For TypeScript, the glTF model typing is available as glTF2.GlTF
// It is auto-generated from the official glTF 2.0 JSON schema
const typedGlTF: glTF2.GlTF = glTF;
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
