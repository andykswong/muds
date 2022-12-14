import { ESMap } from 'typescript';
import { indexOf } from '../id';
import { MapCRUD } from '../types';

/** Generational index map backend by a Map. */
export class GenIdMap<V, I extends number = number>
  implements ESMap<I, V>, MapCRUD<I, V>, Iterable<[I, V]>
{
  private readonly map: Map<number, [I, V]> = new Map();

  public get size(): number {
    return this.map.size;
  }

  public clear(): void {
    this.map.clear();
  }

  public delete(id: I): boolean {
    const entry = this.map.get(indexOf(id));
    if (entry && entry[0] === id) {
      this.map.delete(indexOf(id));
      return true;
    }
    return false;
  }

  public entries(): IterableIterator<[I, V]> {
    return this.map.values();
  }

  public forEach(action: (value: V, id: I) => void): void {
    this.map.forEach((entry) => {
      action(entry[1], entry[0]);
    });
  }

  public get(id: I): V | undefined {
    const entry = this.map.get(indexOf(id));
    if (entry && entry[0] === id) {
      return entry[1];
    }
    return undefined;
  }

  public has(id: I): boolean {
    const entry = this.map.get(indexOf(id));
    return !!entry && entry[0] === id;
  }

  public * keys(): IterableIterator<I> {
    for (const entry of this.map.values()) {
      yield entry[0];
    }
  }

  public set(id: I, value: V): this {
    this.map.set(indexOf(id), [id, value]);
    return this;
  }

  public * values(): IterableIterator<V> {
    for (const entry of this.map.values()) {
      yield entry[1];
    }
  }

  public [Symbol.iterator](): IterableIterator<[I, V]> {
    return this.entries();
  }
}

/** Sparse set based map with generational index as key. */
export class SparseSetMap<V, I extends number = number>
  implements ESMap<I, V>, MapCRUD<I, V>, Iterable<[I, V]>
{
  private readonly sparse: number[] = [];
  private readonly ids: I[] = [];
  private readonly dense: V[] = [];

  public get size(): number {
    return this.dense.length;
  }

  public clear(): void {
    this.sparse.length = 0;
    this.ids.length = 0;
    this.dense.length = 0;
  }

  public delete(id: I): boolean {
    if (this.has(id)) {
      const index = indexOf(id);
      const denseIndex = this.sparse[index];

      this.sparse[indexOf(this.ids[this.size - 1])] = denseIndex;
      this.ids[denseIndex] = this.ids[this.size - 1];
      this.dense[denseIndex] = this.dense[this.size - 1];

      this.sparse[index] = -1;
      this.ids.pop();
      this.dense.pop();

      return true;
    }

    return false;
  }

  public * entries(): IterableIterator<[I, V]> {
    for (let i = 0; i < this.ids.length; ++i) {
      yield [this.ids[i], this.dense[i]];
    }
  }

  public forEach(action: (value: V, id: I) => void): void {
    this.ids.forEach((id, i) => {
      action(this.dense[i], id);
    });
  }

  public get(id: I): V | undefined {
    return this.has(id) ? this.dense[this.sparse[indexOf(id)]] : undefined;
  }

  public has(id: I): boolean {
    return (this.ids[this.sparse[indexOf(id)]] === id);
  }

  public * keys(): IterableIterator<I> {
    for (let i = 0; i < this.ids.length; ++i) {
      yield this.ids[i];
    }
  }

  public set(id: I, value: V): this {
    const denseIndex = this.sparse[indexOf(id)];
    if (!isNaN(denseIndex) && denseIndex >= 0) {
      this.ids[denseIndex] = id;
      this.dense[denseIndex] = value;
    } else {
      this.sparse[indexOf(id)] = this.ids.length;
      this.ids.push(id);
      this.dense.push(value);
    }
    return this;
  }

  public * values(): IterableIterator<V> {
    for (let i = 0; i < this.ids.length; ++i) {
      yield this.dense[i];
    }
  }

  public [Symbol.iterator](): IterableIterator<[I, V]> {
    return this.entries();
  }
}
