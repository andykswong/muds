/** GlTF resource loader implementation using fetch. */
export function glTFFetchLoader(uri: string, type: 'bin'): Promise<Uint8Array>;
export function glTFFetchLoader(uri: string, type: 'img'): Promise<TexImageSource>;
export async function glTFFetchLoader(uri: string, type: 'bin' | 'img'): Promise<Uint8Array | TexImageSource> {
  if (type === 'img') {
    return await loadImage(uri);
  }

  // else return binary
  const data = await fetch(uri);
  const buffer = await data.arrayBuffer();
  return new Uint8Array(buffer);
}

function loadImage(uri: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onerror = () => reject(new Error(`Failed to load: ${uri}`));
    img.onload = () => resolve(img);
    img.src = uri;
  });
}
