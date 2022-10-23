import { Collection } from 'typescript';
import { indexOf } from '../id';
import { IdGenerator } from './generator';

/** An arena holds values that can be accessed by numerical keys. */
export interface Arena<T> extends Collection<number>, Iterable<[number, T]> {
  /** Adds a value to the arena and returns its key. */
  add(value: T): number;

  /** Gets a value by key. Returns undefined if there is no such entry. */
  get(key: number): T | undefined;

  /** Returns an Iterator for all entries in the arena. */
  entries(): Iterator<[number, T]>;

  /** Returns an Iterator for all values in the arena. */
  values(): Iterator<T>;

  /** Calls `action` once for each entry in the arena. */
  forEach(action: (value: T, key: number) => void): void;
}

/** An arena that uses generational index as key. */
export class GenerationalArena<T> implements Arena<T> {
  private readonly allocator: IdGenerator = new IdGenerator();
  private readonly data: T[] = [];

  public get size(): number {
    return this.allocator.size;
  }

  public add(value: T): number {
    const id = this.allocator.add();
    this.data[indexOf(id)] = value;
    return id;
  }

  public clear(): void {
    this.allocator.clear();
    this.data.length = 0;
  }

  public delete(id: number): boolean {
    if (this.allocator.delete(id)) {
      delete this.data[indexOf(id)];
      return true;
    }
    return false;
  }

  public * entries(): IterableIterator<[number, T]> {
    for (const id of this.allocator.values()) {
      yield [id, this.data[indexOf(id)]];
    }
  }

  public forEach(action: (value: T, key: number) => void): void {
    this.allocator.forEach((id) => {
      action(this.data[indexOf(id)], id);
    });
  }

  public get(id: number): T | undefined {
    return this.has(id) ? this.data[indexOf(id)] : undefined;
  }

  public has(id: number): boolean {
    return this.allocator.has(id);
  }

  public keys(): IterableIterator<number> {
    return this.allocator.values();
  }

  public * values(): IterableIterator<T> {
    for (const id of this.allocator.values()) {
      yield this.data[indexOf(id)];
    }
  }

  public [Symbol.iterator](): IterableIterator<[number, T]> {
    return this.entries();
  }
}
