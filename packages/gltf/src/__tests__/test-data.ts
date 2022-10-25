import { readFileSync } from 'fs';
import { join } from 'path';
import { URL } from 'url'; 
import { glTF2 } from '../spec';

const __dirname = new URL('.', import.meta.url).pathname;

export const GLB_FILE = readFileSync(join(__dirname, 'models/Box.glb'), null);
export const GLB_GLTF_JSON: glTF2.GlTF = JSON.parse(readFileSync(join(__dirname, 'models/Box.gltf'), 'utf8'));
export const GLB_BIN = toUint8Array(readFileSync(join(__dirname, 'models/Box0.bin'), null));

export const GLB_WITH_TEX_FILE = readFileSync(join(__dirname, 'models/BoxTextured.glb'), null);

export const GLB_1_0_FILE = readFileSync(join(__dirname, 'models/Box.1.0.glb'), null);

const gltfEmbeddedPath = join(__dirname, 'models/BoxTextured.gltf');
export const GLTF_EMBEDDED_FILE = readFileSync(gltfEmbeddedPath, null);
export const GLTF_EMBEDDED_JSON: glTF2.GlTF = JSON.parse(readFileSync(gltfEmbeddedPath, 'utf8'));

const gltfPath = join(__dirname, 'models/BoxTextured.ext.gltf');
export const GLTF_FILE = readFileSync(gltfPath, null);
export const GLTF_JSON: glTF2.GlTF = JSON.parse(readFileSync(gltfPath, 'utf8'));

function toUint8Array(buffer: Buffer): Uint8Array {
  return new Uint8Array(buffer.buffer, buffer.byteOffset, buffer.byteLength);
}
