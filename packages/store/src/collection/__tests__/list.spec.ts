import { ArrayList } from '../list';

describe('ArrayList', () => {
  test('clear() should empty the container', () => {
    const list = new ArrayList<string>();
    list.push('1');
    list.push('2');

    expect(list.size).toBe(2);

    list.clear();
    expect(list.size).toBe(0);
  });

  test('pop() should remove and return last value', () => {
    const list = new ArrayList<string>();
    const value = 'hello';
    list.push(value);

    expect(list.pop()).toBe(value);
    expect(list.size).toBe(0);
  });

  test('push() should append value to list', () => {
    const list = new ArrayList<string>();
    expect(list.size).toBe(0);

    list.push('hello');
    const value = 'world';
    list.push(value);
    expect(list.size).toBe(2);
    expect(list.last).toBe(value);
    expect(list.get(1)).toBe(value);
    expect(list.has(1)).toBeTruthy();
  });

  test('set() should update value of existing key', () => {
    const list = new ArrayList<string>();
    list.push('hello');

    const newValue = 'new';
    list.set(0, newValue);

    expect(list.get(0)).toBe(newValue);
  });

  test('get() should return undefined for non-existent key', () => {
    const list = new ArrayList<string>();
    expect(list.get(123)).toBeUndefined();
  });

  test('has() should false for non-existent key', () => {
    const list = new ArrayList<string>();
    expect(list.has(123)).toBeFalsy();
  });

  test('foreach() should loop through all id-values', () => {
    const list = new ArrayList<string>();
    const value1 = 'hello', value2 = 'world';
    list.push(value1);
    list.push(value2);

    const results: [number, string][] = [];
    list.forEach((value, id) => results.push([id, value]));

    expect(results).toEqual([[0, value1], [1, value2]]);
  });

  test('entries() should iterate through all id-values', () => {
    const list = new ArrayList<string>();
    const value1 = 'hello', value2 = 'world';
    list.push(value1);
    list.push(value2);

    const results: [number, string][] = [];
    for (const entry of list.entries()) {
      results.push(entry);
    }

    expect(results).toEqual([[0, value1], [1, value2]]);
  });

  test('[Symbol.iterator]() should iterate through all id-values', () => {
    const list = new ArrayList<string>();
    const value1 = 'hello', value2 = 'world';
    list.push(value1);
    list.push(value2);

    const results: string[] = [];
    for (const entry of list) {
      results.push(entry);
    }

    expect(results).toEqual([value1, value2]);
  });

  test('keys() should iterate through all ids', () => {
    const list = new ArrayList<string>();
    const value1 = 'hello', value2 = 'world';
    list.push(value1);
    list.push(value2);

    const results: number[] = [];
    for (const key of list.keys()) {
      results.push(key);
    }

    expect(results).toEqual([0, 1]);
  });

  test('values() should iterate through all values', () => {
    const list = new ArrayList<string>();
    const value1 = 'hello', value2 = 'world';
    list.push(value1);
    list.push(value2);

    const results: string[] = [];
    for (const value of list.values()) {
      results.push(value);
    }

    expect(results).toEqual([value1, value2]);
  });
});
