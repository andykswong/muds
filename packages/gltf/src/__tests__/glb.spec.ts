import { isGLB, parseGLB } from '../glb';
import { GLB_FILE, GLB_BIN, GLB_GLTF_JSON, GLB_1_0_FILE, GLTF_FILE } from './test-data';

describe('isGLB', () => {
  it('should return true for GLB files', () => {
    expect(isGLB(GLB_FILE)).toBeTruthy();
  });

  it('should return false for gltf files', () => {
    expect(isGLB(GLTF_FILE)).toBeFalsy();
  });
});

describe('parseGLB', () => {
  it('should parse GLB file into glTF JSON and binary chunk', async () => {
    const { glTF, binaryChunk } = parseGLB(GLB_FILE);
    expect(glTF).toEqual(GLB_GLTF_JSON);
    expect(binaryChunk).toEqual(GLB_BIN);
  });

  it('should throw for GLB 1.0 files', async () => {
    expect(() => parseGLB(GLB_1_0_FILE)).toThrow(expect.anything());
  });

  it('should throw for non-GLB files', async () => {
    expect(() => parseGLB(GLTF_FILE)).toThrow(expect.anything());
  });
});
