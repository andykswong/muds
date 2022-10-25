import fetchMock from 'jest-fetch-mock';
fetchMock.enableMocks();

import { glTFFetchLoader } from '../fetch';

const LOAD_SUCCESS_SRC = 'img';
const LOAD_FAILURE_SRC = 'fail';

describe('glTFFetchLoader', () => {
  beforeAll(() => {
    Object.defineProperty(global.Image.prototype, 'src', {
      get() {
        return this._src;
      },
      set(src) {
        this._src = src;
        if (src === LOAD_FAILURE_SRC) {
          setTimeout(() => this.onerror(new Error()));
        } else if (src === LOAD_SUCCESS_SRC) {
          setTimeout(() => this.onload());
        }
      },
    });
  });

  it('should fetch binary data from uri', async () => {
    const uri = 'URI';
    const mockData = 'Hello';
    const expected = new Uint8Array([72, 101, 108, 108, 111]);

    fetchMock.doMockOnceIf(uri, mockData);

    expect(await glTFFetchLoader(uri, 'bin')).toEqual(expected);
  });

  it('should fetch image from uri', async () => {
    const image = await glTFFetchLoader(LOAD_SUCCESS_SRC, 'img') as HTMLImageElement;
    expect(image.src).toBe(LOAD_SUCCESS_SRC);
  });

  it('should throw when failing to fetch image', async () => {
    expect.assertions(1);
    await expect(glTFFetchLoader(LOAD_FAILURE_SRC, 'img')).rejects.toBeInstanceOf(Error);
  });
});
