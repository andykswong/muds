const UTF8_DECODER = new TextDecoder('utf-8');

/**
 * Decodes a buffer into string.
 * This uses TextDecoder, which can be polyfilled as per:
 * https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder#polyfill
 */
export function decodeText(data: BufferSource): string {
  return UTF8_DECODER.decode(data);
}

/**
 * Check if URI is a data or blob URI.
 */
export function isDataUri(uri: string): boolean {
  return /^data:.*,.*$/i.test(uri) /* Data URI */ ||
    /^blob:.*$/i.test(uri) /* Blob URI */;
}

/**
 * Extract the base part of a URL.
 */
export function getBaseUrl(url: string): string {
  if (isDataUri(url)) {
    return '';
  }

  const parts = url.split(/[?#]/)[0].split('/');
  parts.pop();
  return parts.length ? parts.join('/') + '/' : '';
}

/**
 * Resolve URIs into absolute path.
 */
export function getAbsoluteUri(uri: string, baseUrl: string): string {
  if (uri === '') {
    return '';
  }

  if (
    /^(https?:)?\/\//i.test(uri) /* Absolute HTTP URL */ ||
    isDataUri(uri) /* Data URI */
  ) {
    return uri;
  }

  // Turn relative path to absolute
  return baseUrl + uri;
}
