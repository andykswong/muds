import { decodeText, getAbsoluteUri, getBaseUrl, isDataUri } from '../utils';

const DATA_URI1 = 'data:,';
const DATA_URI2 = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI12P4//8/w38GIAXDIBKE0DHxgljNBAAO9TXL0Y4OHwAAAABJRU5ErkJggg==';
const DATA_URI3 = 'data:text/plain;charset=UTF-8;page=21,the%20data:1234,5678';
const BLOB_URI = 'blob:http://example.com/550e8400-e29b-41d4-a716-446655440000';
const BASE_URL = 'http://example.com/';
const ABSOLUTE_URL = 'http://example.com/abc.jpg';
const ABSOLUTE_HTTPS_URL = 'https://www.example.com/abc.jpg';

describe('utils', () => {
  describe('decodeText', () => {
    it('should decode buffer into string', () => {
      const hello = new Uint8Array([72, 101, 108, 108, 111]);
      expect(decodeText(hello)).toBe('Hello');

      const world = new Uint8Array([119, 111, 114, 108, 100]);
      expect(decodeText(world)).toBe('world');
    });
  });

  describe('isDataUri', () => {
    it('should return true for URI with data scheme', () => {
      expect(isDataUri(DATA_URI1)).toBeTruthy();
      expect(isDataUri(DATA_URI2)).toBeTruthy();
      expect(isDataUri(DATA_URI3)).toBeTruthy();
    });

    it('should return true for URI with blob scheme', () => {
      expect(isDataUri(BLOB_URI)).toBeTruthy();
    });

    it('should return false for URLs', () => {
      expect(isDataUri(ABSOLUTE_URL)).toBeFalsy();
    });
  });

  describe('getBaseUrl', () => {
    it('should return base part of a URL', () => {
      expect(getBaseUrl(`${BASE_URL}abc`)).toBe(BASE_URL);
      expect(getBaseUrl(`${BASE_URL}a/b/c.jpg?q=1&r=2#hash`)).toBe(`${BASE_URL}a/b/`);
    });

    it('should return base part of a relative path', () => {
      expect(getBaseUrl('./abc/def/i.jpg')).toBe('./abc/def/');
      expect(getBaseUrl('a.jpg')).toBe('');
    });

    it('should return empty string for data URI', () => {
      expect(getBaseUrl(DATA_URI3)).toBe('');
    });
  });

  describe('getAbsoluteUri', () => {
    it('should prepend base URL to a relative path', () => {
      expect(getAbsoluteUri('abc', BASE_URL)).toBe(`${BASE_URL}abc`);
    });

    it('should not modify absolute URL', () => {
      expect(getAbsoluteUri(ABSOLUTE_URL, BASE_URL)).toBe(ABSOLUTE_URL);
      expect(getAbsoluteUri(ABSOLUTE_HTTPS_URL, BASE_URL)).toBe(ABSOLUTE_HTTPS_URL);
    });

    it('should not modify data or blob URI', () => {
      expect(getAbsoluteUri(DATA_URI3, BASE_URL)).toBe(DATA_URI3);
      expect(getAbsoluteUri(BLOB_URI, BASE_URL)).toBe(BLOB_URI);
    });

    it('should return empty for empty path', () => {
      expect(getAbsoluteUri('', BASE_URL)).toBe('');
    });
  });
});
