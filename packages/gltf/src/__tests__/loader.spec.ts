import { jest } from '@jest/globals';
import { parseGLB } from '../glb';
import { loadGlTF, loadGlTFResources } from '../loader';
import { GlTFResourceLoader } from '../types';
import {
  GLB_FILE, GLB_BIN, GLB_GLTF_JSON, GLB_WITH_TEX_FILE, GLTF_FILE, GLTF_JSON, GLTF_EMBEDDED_JSON
} from './test-data';

const BASE_URL = 'http://example.com/';
const FILE_URL = `${BASE_URL}file`;

const BUFFER = new Uint8Array([0, 1, 2]);
const IMAGE = new Image();
IMAGE.src = 'test.png';

describe('loadGlTF', () => {
  it('should load and parse GLB file from URI', async () => {
    const mockLoader = jest.fn();
    mockLoader.mockReturnValueOnce(Promise.resolve(GLB_FILE));

    const asset = await loadGlTF(FILE_URL, false, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF: GLB_GLTF_JSON,
      binaryChunk: GLB_BIN,
      baseUrl: BASE_URL,
    });
    expect(mockLoader).toHaveBeenCalledWith(FILE_URL, 'bin');
  });

  it('should load and parse glTF file from URI', async () => {
    const mockLoader = jest.fn();
    mockLoader.mockReturnValueOnce(Promise.resolve(GLTF_FILE));

    const asset = await loadGlTF(FILE_URL, false, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF: GLTF_JSON,
      baseUrl: BASE_URL,
    });
    expect(mockLoader).toHaveBeenCalledWith(FILE_URL, 'bin');
  });

  it('should load glTF buffers and images', async () => {
    const mockLoader = jest.fn();
    mockLoader
      .mockReturnValueOnce(Promise.resolve(GLTF_FILE))
      .mockReturnValueOnce(Promise.resolve(BUFFER))
      .mockReturnValueOnce(Promise.resolve(IMAGE));

    const asset = await loadGlTF(FILE_URL, true, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF: GLTF_JSON,
      baseUrl: BASE_URL,
      buffers: [BUFFER],
      images: [IMAGE],
    });
    expect(mockLoader).toHaveBeenNthCalledWith(1, FILE_URL, 'bin');
    expect(mockLoader).toHaveBeenNthCalledWith(2, `${BASE_URL}BoxTextured0.bin`, 'bin');
    expect(mockLoader).toHaveBeenNthCalledWith(3, `${BASE_URL}CesiumLogoFlat.png`, 'img');
  });
});

describe('loadGlTFResources', () => {
  it('should load external buffers and images', async () => {
    const mockLoader = jest.fn();
    mockLoader
      .mockReturnValueOnce(Promise.resolve(BUFFER))
      .mockReturnValueOnce(Promise.resolve(IMAGE));

    const asset = await loadGlTFResources({
      glTF: GLTF_JSON,
      baseUrl: BASE_URL,
    }, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF: GLTF_JSON,
      baseUrl: BASE_URL,
      buffers: [BUFFER],
      images: [IMAGE],
    });
    expect(mockLoader).toHaveBeenNthCalledWith(1, `${BASE_URL}BoxTextured0.bin`, 'bin');
    expect(mockLoader).toHaveBeenNthCalledWith(2, `${BASE_URL}CesiumLogoFlat.png`, 'img');
  });

  it('should load embedded buffers and images', async () => {
    const mockLoader = jest.fn();
    mockLoader
      .mockReturnValueOnce(Promise.resolve(BUFFER))
      .mockReturnValueOnce(Promise.resolve(IMAGE));

    const asset = await loadGlTFResources({
      glTF: GLTF_EMBEDDED_JSON,
      baseUrl: BASE_URL,
    }, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF: GLTF_EMBEDDED_JSON,
      baseUrl: BASE_URL,
      buffers: [BUFFER],
      images: [IMAGE],
    });
    expect(mockLoader).toHaveBeenNthCalledWith(1, GLTF_EMBEDDED_JSON.buffers?.[0].uri, 'bin');
    expect(mockLoader).toHaveBeenNthCalledWith(2, GLTF_EMBEDDED_JSON.images?.[0].uri, 'img');
  });

  it('should load GLB binary chunk as buffer', async () => {
    const mockLoader = jest.fn();

    const asset = await loadGlTFResources({
      glTF: GLB_GLTF_JSON,
      binaryChunk: GLB_BIN,
      baseUrl: BASE_URL,
    }, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF: GLB_GLTF_JSON,
      binaryChunk: GLB_BIN,
      baseUrl: BASE_URL,
      buffers: [GLB_BIN],
      images: [],
    });
    expect(mockLoader).not.toHaveBeenCalled();
  });

  it('should load GLB embedded image', async () => {
    const blobUri = 'blob:testing123';
    const mockCreateObjectURL = jest.fn();
    const mockRevokeObjectURL = jest.fn();
    const mockLoader = jest.fn();

    (global.URL.createObjectURL as unknown) = mockCreateObjectURL;
    (global.URL.revokeObjectURL as unknown) = mockRevokeObjectURL;

    mockCreateObjectURL.mockReturnValueOnce(blobUri);
    mockLoader.mockReturnValueOnce(Promise.resolve(IMAGE));

    const { glTF, binaryChunk } = parseGLB(GLB_WITH_TEX_FILE);

    const asset = await loadGlTFResources({
      glTF,
      binaryChunk,
      baseUrl: BASE_URL,
    }, mockLoader as GlTFResourceLoader);

    expect(asset).toEqual({
      glTF,
      binaryChunk,
      baseUrl: BASE_URL,
      buffers: [binaryChunk],
      images: [IMAGE],
    });
    expect(mockCreateObjectURL).toHaveBeenCalled();
    expect(mockRevokeObjectURL).toHaveBeenCalledWith(blobUri);
    expect(mockLoader).toHaveBeenCalledWith(blobUri, 'img');
  });
});
