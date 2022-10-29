import { ArrayDeque, ArrayList, GenerationalArena, IdGenerator } from '../../collection';
import { ObservableArena, ObservableDeque, ObservableGenerator, ObservableList, ObservableMap, ObservableSet } from '../observables';

describe('ObservableArena', () => {
  test('add() should add value to container and trigger onAdd', () => {
    expect.assertions(5);

    const container = new ObservableArena(new GenerationalArena<string>());
    const value = 'hello';
    let eventId!: number;

    container.onAdd.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(value);
      eventId = k;
    });

    expect(container.size).toBe(0);
    const id = container.add(value);
    expect(id).toBe(eventId);
    expect(container.size).toBe(1);

  });

  test('clear() should empty the container and trigger onClear', () => {
    expect.assertions(3);

    const container = new ObservableArena(new GenerationalArena<string>());
    container.onClear.addListener((thisArg) => expect(thisArg).toBe(container));
    container.add('1');
    container.add('2');

    expect(container.size).toBe(2);
    container.clear();
    expect(container.size).toBe(0);
  });

  test('delete() should remove value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableArena(new GenerationalArena<string>());
    const value = 'hello';
    let eventId!: number;

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(value);
      eventId = k;
    });
    const id = container.add(value);

    expect(container.delete(id)).toBeTruthy();
    expect(id).toBe(eventId);
    expect(container.size).toBe(0);
  });

  test('delete() should do nothing and return false for non-existent id', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    container.onDelete.addListener(() => {
      throw new Error('onDelete should not be triggered');
    });
    container.add('hello');
    expect(container.delete(999)).toBeFalsy();
  });

  test('set() should update value and trigger onUpdate', () => {
    expect.assertions(5);

    const container = new ObservableArena(new GenerationalArena<string>());
    const value = 'hello';
    const newValue = 'new';
    let eventId!: number;

    container.onUpdate.addListener((thisArg, k, v, pv) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(newValue);
      expect(pv).toBe(value);
      eventId = k;
    });

    const id = container.add('hello');
    container.set(id, newValue);

    expect(id).toBe(eventId);
    expect(container.get(id)).toBe(newValue);
  });

  test('set() should do nothing for non-existent id', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    container.onUpdate.addListener(() => {
      throw new Error('onUpdate should not be triggered');
    });

    const value = 'hello';
    const id = container.add(value);

    container.set(999, 'unreachable');

    expect(container.get(id)).toBe(value);
  });

  test('size should return container size', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    container.add('hello');
    container.add('world');
    expect(container.size).toBe(2);
  });

  test('get() should return existing value', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const value = 'hello';
    const id = container.add(value);
    expect(container.get(id)).toBe(value);
    expect(container.has(id)).toBeTruthy();
  });

  test('has() should return true for existing id', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const id = container.add('hello');
    expect(container.has(id)).toBeTruthy();
  });

  test('foreach() should loop through all id-values', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const value1 = 'hello', value2 = 'world';
    const id1 = container.add(value1);
    const id2 = container.add(value2);

    const results: [number, string][] = [];
    container.forEach((value, id) => results.push([id, value]));

    expect(results).toEqual([[id1, value1], [id2, value2]]);
  });

  test('entries() should iterate through all id-values', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const value1 = 'hello', value2 = 'world';
    const id1 = container.add(value1);
    const id2 = container.add(value2);

    const results: [number, string][] = [];
    for (const entry of { [Symbol.iterator]: () => container.entries() }) {
      results.push(entry);
    }

    expect(results).toEqual([[id1, value1], [id2, value2]]);
  });

  test('[Symbol.iterator]() should iterate through all id-values', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const value1 = 'hello', value2 = 'world';
    const id1 = container.add(value1);
    const id2 = container.add(value2);

    const results: [number, string][] = [];
    for (const entry of container) {
      results.push(entry);
    }

    expect(results).toEqual([[id1, value1], [id2, value2]]);
  });

  test('keys() should iterate through all ids', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const value1 = 'hello', value2 = 'world';
    const id1 = container.add(value1);
    const id2 = container.add(value2);

    const results: number[] = [];
    for (const key of { [Symbol.iterator]: () => container.keys() }) {
      results.push(key);
    }

    expect(results).toEqual([id1, id2]);
  });

  test('values() should iterate through all values', () => {
    const container = new ObservableArena(new GenerationalArena<string>());
    const value1 = 'hello', value2 = 'world';
    container.add(value1);
    container.add(value2);

    const results: string[] = [];
    for (const value of { [Symbol.iterator]: () => container.values() }) {
      results.push(value);
    }

    expect(results).toEqual([value1, value2]);
  });
});

describe('ObservableDeque', () => {
  test('first should return first element', () => {
    const container = new ObservableDeque(new ArrayDeque<string>());
    const value = '1';
    container.push(value);
    expect(container.first).toBe(value);
  });

  test('last should return last element', () => {
    const container = new ObservableDeque(new ArrayDeque<string>());
    const value = '1';
    container.unshift(value);
    expect(container.last).toBe(value);
  });

  test('clear() should empty the container and trigger onClear', () => {
    expect.assertions(3);

    const container = new ObservableDeque(new ArrayDeque<string>());
    container.onClear.addListener((thisArg) => expect(thisArg).toBe(container));
    container.push('1');
    container.push('2');

    expect(container.size).toBe(2);
    container.clear();
    expect(container.size).toBe(0);
  });

  test('push() should append value to container and trigger onAdd', () => {
    expect.assertions(5);

    const container = new ObservableDeque(new ArrayDeque<string>());
    const value = 'hello';
    let eventId!: number;

    container.onAdd.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(value);
      eventId = k;
    });

    expect(container.size).toBe(0);
    container.push(value);
    expect(eventId).toBe(0);
    expect(container.size).toBe(1);

  });

  test('pop() should remove and return last value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableDeque(new ArrayDeque<string>());
    const value = 'hello';
    let eventId!: number;

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(value);
      eventId = k;
    });
    container.push(value);

    expect(container.pop()).toBe(value);
    expect(eventId).toBe(0);
    expect(container.size).toBe(0);
  });

  test('unshift() should preppend value to container and trigger onAdd', () => {
    expect.assertions(5);

    const container = new ObservableDeque(new ArrayDeque<string>());
    const value = 'hello';
    let eventId!: number;

    container.onAdd.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(value);
      eventId = k;
    });

    expect(container.size).toBe(0);
    container.unshift(value);
    expect(eventId).toBe(0);
    expect(container.size).toBe(1);

  });

  test('shift() should remove and return first value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableDeque(new ArrayDeque<string>());
    const value = 'hello';
    let eventId!: number;

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(value);
      eventId = k;
    });
    container.push(value);

    expect(container.shift()).toBe(value);
    expect(eventId).toBe(0);
    expect(container.size).toBe(0);
  });

  test('first, last, shift(), pop() should return undefined and not trigger onDelete for empty container', () => {
    const container = new ObservableDeque(new ArrayDeque<string>());
    container.onDelete.addListener(() => {
      throw new Error('onDelete should not be triggered');
    });
    expect(container.first).toBeUndefined();
    expect(container.last).toBeUndefined();
    expect(container.shift()).toBeUndefined();
    expect(container.pop()).toBeUndefined();
  });

  test('size should return container size', () => {
    const container = new ObservableDeque(new ArrayDeque<string>());
    container.push('hello');
    container.push('world');
    expect(container.size).toBe(2);
  });
});

describe('ObservableGenerator', () => {
  test('add() should generate new id and trigger onAdd', () => {
    expect.assertions(5);

    const container = new ObservableGenerator<number>(new IdGenerator());
    let eventId!: number;

    container.onAdd.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(k);
      eventId = k;
    });

    expect(container.size).toBe(0);
    const id = container.add();
    expect(id).toBe(eventId);
    expect(container.size).toBe(1);

  });

  test('clear() should empty the container and trigger onClear', () => {
    expect.assertions(3);

    const container = new ObservableGenerator<number>(new IdGenerator());
    container.onClear.addListener((thisArg) => expect(thisArg).toBe(container));
    container.add();
    container.add();

    expect(container.size).toBe(2);
    container.clear();
    expect(container.size).toBe(0);
  });

  test('delete() should remove value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableGenerator<number>(new IdGenerator());
    let eventId!: number;

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(v).toBe(k);
      eventId = k;
    });
    const id = container.add();

    expect(container.delete(id)).toBeTruthy();
    expect(id).toBe(eventId);
    expect(container.size).toBe(0);
  });

  test('delete() should do nothing and return false for non-existent id', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    container.onDelete.addListener(() => {
      throw new Error('onDelete should not be triggered');
    });
    container.add();
    expect(container.delete(999)).toBeFalsy();
  });

  test('size should return container size', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    container.add();
    container.add();
    expect(container.size).toBe(2);
  });

  test('has() should return true for existing id', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    const id = container.add();
    expect(container.has(id)).toBeTruthy();
  });

  test('foreach() should loop through all ids', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    const id1 = container.add();
    const id2 = container.add();

    const results: [number, number][] = [];
    container.forEach((value, id) => results.push([id, value]));

    expect(results).toEqual([[id1, id1], [id2, id2]]);
  });

  test('entries() should iterate through all ids', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    const id1 = container.add();
    const id2 = container.add();

    const results: [number, number][] = [];
    for (const entry of { [Symbol.iterator]: () => container.entries() }) {
      results.push(entry);
    }

    expect(results).toEqual([[id1, id1], [id2, id2]]);
  });

  test('[Symbol.iterator]() should iterate through all ids', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    const id1 = container.add();
    const id2 = container.add();

    const results: number[] = [];
    for (const entry of container) {
      results.push(entry);
    }

    expect(results).toEqual([id1, id2]);
  });

  test('keys() should iterate through all ids', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    const id1 = container.add();
    const id2 = container.add();

    const results: number[] = [];
    for (const key of { [Symbol.iterator]: () => container.keys() }) {
      results.push(key);
    }

    expect(results).toEqual([id1, id2]);
  });

  test('values() should iterate through all ids', () => {
    const container = new ObservableGenerator<number>(new IdGenerator());
    const id1 = container.add();
    const id2 = container.add();

    const results: number[] = [];
    for (const value of { [Symbol.iterator]: () => container.values() }) {
      results.push(value);
    }

    expect(results).toEqual([id1, id2]);
  });
});

describe('ObservableList', () => {
  test('push() should append value to container and trigger onAdd', () => {
    expect.assertions(6);

    const container = new ObservableList(new ArrayList<string>());
    const value = 'hello';

    container.onAdd.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(0);
      expect(v).toBe(value);
    });

    expect(container.size).toBe(0);
    container.push(value);
    expect(container.last).toBe(value);
    expect(container.size).toBe(1);

  });

  test('clear() should empty the container and trigger onClear', () => {
    expect.assertions(3);

    const container = new ObservableList(new ArrayList<string>());
    container.onClear.addListener((thisArg) => expect(thisArg).toBe(container));
    container.push('1');
    container.push('2');

    expect(container.size).toBe(2);
    container.clear();
    expect(container.size).toBe(0);
  });

  test('pop() should remove last value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableList(new ArrayList<string>());
    const value = 'hello';

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(0);
      expect(v).toBe(value);
    });
    container.push(value);

    expect(container.pop()).toBe(value);
    expect(container.size).toBe(0);
  });

  test('pop() should do nothing and return undefined for empty container', () => {
    const container = new ObservableList(new ArrayList<string>());
    container.onDelete.addListener(() => {
      throw new Error('onDelete should not be triggered');
    });
    expect(container.pop()).toBeUndefined();
  });

  test('set() should update value and trigger onUpdate', () => {
    expect.assertions(5);

    const container = new ObservableList(new ArrayList<string>());
    const value = 'hello';
    const newValue = 'new';

    container.onUpdate.addListener((thisArg, k, v, pv) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(0);
      expect(v).toBe(newValue);
      expect(pv).toBe(value);
    });

    container.push('hello');
    container.set(0, newValue);

    expect(container.get(0)).toBe(newValue);
  });

  test('set() should trigger onAdd for non-existent id', () => {
    expect.assertions(4);

    const value = 'world';
    const container = new ObservableList(new ArrayList<string>());
    container.push('hello');
    container.onAdd.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(1);
      expect(v).toBe(value);
    });

    container.set(1, value);
    expect(container.get(1)).toBe(value);
  });

  test('size should return container size', () => {
    const container = new ObservableList(new ArrayList<string>());
    container.push('hello');
    container.push('world');
    expect(container.size).toBe(2);
  });

  test('get() should return existing value', () => {
    const container = new ObservableList(new ArrayList<string>());
    const value = 'hello';
    container.push(value);
    expect(container.get(0)).toBe(value);
  });

  test('has() should return true for existing id', () => {
    const container = new ObservableList(new ArrayList<string>());
    container.push('hello');
    expect(container.has(0)).toBeTruthy();
  });

  test('foreach() should loop through all id-values', () => {
    const container = new ObservableList(new ArrayList<string>());
    const value1 = 'hello', value2 = 'world';
    container.push(value1);
    container.push(value2);

    const results: [number, string][] = [];
    container.forEach((value, id) => results.push([id, value]));

    expect(results).toEqual([[0, value1], [1, value2]]);
  });

  test('entries() should iterate through all id-values', () => {
    const container = new ObservableList(new ArrayList<string>());
    const value1 = 'hello', value2 = 'world';
    container.push(value1);
    container.push(value2);

    const results: [number, string][] = [];
    for (const entry of { [Symbol.iterator]: () => container.entries() }) {
      results.push(entry);
    }

    expect(results).toEqual([[0, value1], [1, value2]]);
  });

  test('[Symbol.iterator]() should iterate through all id-values', () => {
    const container = new ObservableList<string>(new ArrayList<string>());
    const value1 = 'hello', value2 = 'world';
    container.push(value1);
    container.push(value2);

    const results: string[] = [];
    for (const entry of container) {
      results.push(entry);
    }

    expect(results).toEqual([value1, value2]);
  });

  test('keys() should iterate through all ids', () => {
    const container = new ObservableList(new ArrayList<string>());
    const value1 = 'hello', value2 = 'world';
    container.push(value1);
    container.push(value2);

    const results: number[] = [];
    for (const key of { [Symbol.iterator]: () => container.keys() }) {
      results.push(key);
    }

    expect(results).toEqual([0, 1]);
  });

  test('values() should iterate through all values', () => {
    const container = new ObservableList(new ArrayList<string>());
    const value1 = 'hello', value2 = 'world';
    container.push(value1);
    container.push(value2);

    const results: string[] = [];
    for (const value of { [Symbol.iterator]: () => container.values() }) {
      results.push(value);
    }

    expect(results).toEqual([value1, value2]);
  });
});

describe('ObservableMap', () => {
  test('clear() should empty the container and trigger onClear', () => {
    expect.assertions(3);

    const container = new ObservableMap<number, string>(new Map<number, string>());
    container.onClear.addListener((thisArg) => expect(thisArg).toBe(container));
    container.set(1, '1');
    container.set(2, '2');

    expect(container.size).toBe(2);
    container.clear();
    expect(container.size).toBe(0);
  });

  test('delete() should remove value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id = 1;
    const value = 'hello';

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(id);
      expect(v).toBe(value);
    });
    container.set(id, value);

    expect(container.delete(id)).toBeTruthy();
    expect(container.size).toBe(0);
  });

  test('delete() should do nothing and return false for non-existent id', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    container.onDelete.addListener(() => {
      throw new Error('onDelete should not be triggered');
    });
    container.set(1, 'hello');
    expect(container.delete(999)).toBeFalsy();
  });

  test('set() should update value and trigger onUpdate', () => {
    expect.assertions(5);

    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id = 1;
    const value = 'hello';
    const newValue = 'new';

    container.onUpdate.addListener((thisArg, k, v, pv) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(id);
      expect(v).toBe(newValue);
      expect(pv).toBe(value);
    });

    container.set(id, 'hello');
    container.set(id, newValue);
    expect(container.get(id)).toBe(newValue);
  });

  test('set() should do nothing for non-existent id', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    container.onUpdate.addListener(() => {
      throw new Error('onUpdate should not be triggered');
    });

    const value = 'hello';
    container.set(1, value);
    container.set(999, 'unreachable');
  });

  test('size should return container size', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    container.set(1, 'hello');
    container.set(2, 'world');
    expect(container.size).toBe(2);
  });

  test('get() should return existing value', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const value = 'hello';
    const id = 1;
    container.set(id, value);
    expect(container.get(id)).toBe(value);
    expect(container.has(id)).toBeTruthy();
  });

  test('has() should return true for existing id', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id = 1;
    container.set(id, 'hello');
    expect(container.has(id)).toBeTruthy();
  });

  test('foreach() should loop through all id-values', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id1 = 1, id2 = 2;
    const value1 = 'hello', value2 = 'world';
    container.set(id1, value1);
    container.set(id2, value2);

    const results: [number, string][] = [];
    container.forEach((value, id) => results.push([id, value]));

    expect(results).toEqual([[id1, value1], [id2, value2]]);
  });

  test('entries() should iterate through all id-values', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id1 = 1, id2 = 2;
    const value1 = 'hello', value2 = 'world';
    container.set(id1, value1);
    container.set(id2, value2);

    const results: [number, string][] = [];
    for (const entry of { [Symbol.iterator]: () => container.entries() }) {
      results.push(entry);
    }

    expect(results).toEqual([[id1, value1], [id2, value2]]);
  });

  test('[Symbol.iterator]() should iterate through all id-values', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id1 = 1, id2 = 2;
    const value1 = 'hello', value2 = 'world';
    container.set(id1, value1);
    container.set(id2, value2);

    const results: [number, string][] = [];
    for (const entry of container) {
      results.push(entry);
    }

    expect(results).toEqual([[id1, value1], [id2, value2]]);
  });

  test('keys() should iterate through all ids', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id1 = 1, id2 = 2;
    const value1 = 'hello', value2 = 'world';
    container.set(id1, value1);
    container.set(id2, value2);

    const results: number[] = [];
    for (const key of { [Symbol.iterator]: () => container.keys() }) {
      results.push(key);
    }

    expect(results).toEqual([id1, id2]);
  });

  test('values() should iterate through all values', () => {
    const container = new ObservableMap<number, string>(new Map<number, string>());
    const id1 = 1, id2 = 2;
    const value1 = 'hello', value2 = 'world';
    container.set(id1, value1);
    container.set(id2, value2);

    const results: string[] = [];
    for (const value of { [Symbol.iterator]: () => container.values() }) {
      results.push(value);
    }

    expect(results).toEqual([value1, value2]);
  });
});

describe('ObservableSet', () => {
  test('clear() should empty the container and trigger onClear', () => {
    expect.assertions(3);

    const container = new ObservableSet<string>(new Set<string>());
    container.onClear.addListener((thisArg) => expect(thisArg).toBe(container));
    container.add('1');
    container.add('2');

    expect(container.size).toBe(2);
    container.clear();
    expect(container.size).toBe(0);
  });

  test('delete() should remove value and trigger onDelete', () => {
    expect.assertions(5);

    const container = new ObservableSet<string>(new Set<string>());
    const value = 'hello';

    container.onDelete.addListener((thisArg, k, v) => {
      expect(thisArg).toBe(container);
      expect(k).toBe(value);
      expect(v).toBe(value);
    });
    container.add(value);

    expect(container.delete(value)).toBeTruthy();
    expect(container.size).toBe(0);
  });

  test('delete() should do nothing and return false for non-existent id', () => {
    const container = new ObservableSet<string>(new Set<string>());
    container.onDelete.addListener(() => {
      throw new Error('onDelete should not be triggered');
    });
    container.add('hello');
    expect(container.delete('world')).toBeFalsy();
  });

  test('size should return container size', () => {
    const container = new ObservableSet<string>(new Set<string>());
    container.add('hello');
    container.add('world');
    expect(container.size).toBe(2);
  });

  test('has() should return true for existing value', () => {
    const container = new ObservableSet<string>(new Set<string>());
    const value = 'hello';
    container.add(value);
    expect(container.has(value)).toBeTruthy();
  });

  test('foreach() should loop through all values', () => {
    const container = new ObservableSet<string>(new Set<string>());
    const value1 = 'hello', value2 = 'world';
    container.add(value1);
    container.add(value2);

    const results: [string, string][] = [];
    container.forEach((value, id) => results.push([id, value]));

    expect(results).toEqual([[value1, value1], [value2, value2]]);
  });

  test('entries() should iterate through all id-values', () => {
    const container = new ObservableSet<string>(new Set<string>());
    const value1 = 'hello', value2 = 'world';
    container.add(value1);
    container.add(value2);

    const results: [string, string][] = [];
    for (const entry of { [Symbol.iterator]: () => container.entries() }) {
      results.push(entry);
    }

    expect(results).toEqual([[value1, value1], [value2, value2]]);
  });

  test('[Symbol.iterator]() should iterate through all id-values', () => {
    const container = new ObservableSet<string>(new Set<string>());
    const value1 = 'hello', value2 = 'world';
    container.add(value1);
    container.add(value2);

    const results: string[] = [];
    for (const entry of container) {
      results.push(entry);
    }

    expect(results).toEqual([value1, value2]);
  });

  test('keys() should iterate through all ids', () => {
    const container = new ObservableSet<string>(new Set<string>());
    const value1 = 'hello', value2 = 'world';
    container.add(value1);
    container.add(value2);

    const results: string[] = [];
    for (const key of { [Symbol.iterator]: () => container.keys() }) {
      results.push(key);
    }

    expect(results).toEqual([value1, value2]);
  });

  test('values() should iterate through all values', () => {
    const container = new ObservableSet<string>(new Set<string>());
    const value1 = 'hello', value2 = 'world';
    container.add(value1);
    container.add(value2);

    const results: string[] = [];
    for (const value of { [Symbol.iterator]: () => container.values() }) {
      results.push(value);
    }

    expect(results).toEqual([value1, value2]);
  });
});
