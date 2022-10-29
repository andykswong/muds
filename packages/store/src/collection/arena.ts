import { ESMap } from 'typescript';
import { indexOf } from '../id';
import { IdGenerator } from './generator';
import { MapGetSet } from '../types';

/** An arena holds values that can be accessed by numerical keys. */
export interface Arena<T, I extends number = number>
  extends ESMap<I, T>, MapGetSet<I, T>, Iterable<[I, T]>
{
  /** Adds a value to the arena and returns its key. */
  add(value: T): I;

  /** Updates a value on the arena. Does nothing if the key does not exist. */
  set(key: I, value: T): this;
}

/** An arena that uses generational index as key. */
export class GenerationalArena<T, I extends number = number> implements Arena<T, I> {
  private ids: IdGenerator<I> = new IdGenerator();
  private readonly data: T[] = [];

  public get size(): number {
    return this.ids.size;
  }

  public add(value: T): I {
    const id = this.ids.add();
    this.data[indexOf(id)] = value;
    return id;
  }

  public clear(): void {
    this.ids.clear();
    this.data.length = 0;
  }

  public delete(id: I): boolean {
    if (this.ids.delete(id)) {
      delete this.data[indexOf(id)];
      return true;
    }
    return false;
  }

  public * entries(): IterableIterator<[I, T]> {
    for (const id of this.ids.values()) {
      yield [id, this.data[indexOf(id)]];
    }
  }

  public forEach(action: (value: T, key: I) => void): void {
    this.ids.forEach((id) => {
      action(this.data[indexOf(id)], id);
    });
  }

  public get(id: I): T | undefined {
    return this.ids.has(id) ? this.data[indexOf(id)] : undefined;
  }

  public has(id: I): boolean {
    return this.ids.has(id);
  }

  public keys(): IterableIterator<I> {
    return this.ids.values();
  }

  public set(id: I, value: T): this {
    if (this.ids.has(id)) {
      this.data[indexOf(id)] = value;
    }
    return this;
  }

  public * values(): IterableIterator<T> {
    for (const id of this.ids.values()) {
      yield this.data[indexOf(id)];
    }
  }

  public [Symbol.iterator](): IterableIterator<[I, T]> {
    return this.entries();
  }
}
