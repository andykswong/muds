import { glTF2 } from './spec';

/**
 * A parsed glTF 2.0 asset and its resources.
 */
export interface GlTFAsset {
  /** The glTF JSON content. */
  glTF: glTF2.GlTF;

  /** Optional binary chunk for GLB. */
  binaryChunk?: Uint8Array;

  /** Base URL of this asset. */
  baseUrl?: string;

  /** Resolved buffer data. */
  buffers?: Uint8Array[];

  /** Resolved images. */
  images?: TexImageSource[];
}

/**
 * A GlTF resource loader function.
 */
export interface GlTFResourceLoader {
  /**
   * Loads a file as binary buffer.
   * @param uri URI to load. This can be a relative, absolute, data or blob URI
   * @returns a promise containing a Uint8Array of the file content
   */
  (uri: string, type: 'bin'): Promise<Uint8Array>;

  /**
   * Loads a file as TexImageSource.
   * @param uri URI to load. This can be a relative, absolute, data or blob URI
   * @returns a promise containing the image data
   */
  (uri: string, type: 'img'): Promise<TexImageSource>;
}
