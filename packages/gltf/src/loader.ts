import { glTFFetchLoader } from './fetch';
import { isGLB, parseGLB } from './glb';
import { glTF2 } from './spec';
import { GlTFAsset, GlTFResourceLoader } from './types';
import { decodeText, getBaseUrl, getAbsoluteUri } from './utils';

/**
 * Loads a GlTF file (JSON or GLB), and optionally its external resources (buffers and images).
 */
export async function loadGlTF(
  uri: string, loadResources = true, loader: GlTFResourceLoader = glTFFetchLoader
): Promise<GlTFAsset> {
  const baseUrl = getBaseUrl(uri);
  let asset: GlTFAsset;

  const binContent = await loader(uri, 'bin');
  if (isGLB(binContent)) {
    asset = parseGLB(binContent);
  } else {
    asset = { glTF: JSON.parse(decodeText(binContent)) };
  }

  asset.baseUrl = baseUrl;

  if (loadResources) {
    asset = await loadGlTFResources(asset, loader);
  }

  return asset;
}

/**
 * Loads the external resources (buffers and images) for a glTF asset.
 */
export async function loadGlTFResources(
  asset: GlTFAsset, loader: GlTFResourceLoader = glTFFetchLoader
): Promise<GlTFAsset> {
  asset = await loadBuffers(asset, loader);
  return await loadImages(asset, loader);
}

async function loadBuffers(asset: GlTFAsset, loader: GlTFResourceLoader): Promise<GlTFAsset> {
  const buffers = asset.buffers || [];

  if (asset.glTF.buffers) {
    for (let i = 0; i < asset.glTF.buffers.length; ++i) {
      if (buffers[i]) {
        continue;
      }

      const buffer = asset.glTF.buffers[i];
      if (!buffer.uri) {
        // uri of buffer 0 of GLB may be undefined; in this case it uses the binary chunk
        if (i !== 0 || !asset.binaryChunk) {
          throw new Error(`Invalid glTF: missing uri for buffer ${i}`);
        }
        buffers[i] = asset.binaryChunk;
      } else {
        buffers[i] = await loader(getAbsoluteUri(buffer.uri, asset.baseUrl || ''), 'bin');
      }
    }
  }

  return { ...asset, buffers };
}

async function loadImages(asset: GlTFAsset, loader: GlTFResourceLoader): Promise<GlTFAsset> {
  const { glTF, buffers = [], images = [] } = asset;

  if (glTF.images) {
    for (let i = 0; i < glTF.images.length; ++i) {
      if (images[i]) {
        continue;
      }

      const image: glTF2.Image = glTF.images[i];
      let isObjectURL = false;
      let uri = image.uri;

      if (image.bufferView || image.bufferView === 0) {
        const bufferView = glTF.bufferViews?.[image.bufferView];
        if (!bufferView) {
          throw new Error(`Invalid glTF: invalid bufferView for image ${i}`);
        }

        const blob = new Blob(
          [getBufferViewData(buffers, bufferView)],
          { type: image.mimeType }
        );
        uri = URL.createObjectURL(blob);
        isObjectURL = true;
      }

      if (!uri) {
        throw new Error(`Invalid glTF: missing uri or bufferView for image ${i}`);
      }

      try {
        images[i] = await loader(getAbsoluteUri(uri, asset.baseUrl || ''), 'img');
      } finally {
        if (isObjectURL) {
          URL.revokeObjectURL(uri);
        }
      }
    }
  }

  return { ...asset, images };
}

function getBufferViewData(buffers: Uint8Array[], bufferView: glTF2.BufferView): Uint8Array {
  const buffer = buffers[bufferView.buffer];
  if (!buffer) {
    return new Uint8Array();
  }
  return new Uint8Array(buffer.buffer, buffer.byteOffset + (bufferView.byteOffset || 0), bufferView.byteLength);
}
