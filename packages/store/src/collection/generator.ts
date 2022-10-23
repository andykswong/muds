import { Collection } from 'typescript';
import { create as id, generationOf, indexOf, MAX_SAFE_GENERATION } from '../id';

/** Generator of values. */
export interface Generator<T> extends Collection<T>, Iterable<T> {
  /** Creates a new value. */
  add(): T;

  /** Deletes a value and returns if the value originally exists */
  delete(value: T): boolean;

  /** Returns an Iterator for all active values. */
  values(): Iterator<T>;

  /** Calls `action` once for each active entry. */
  forEach(action: (value: T) => void): void;
}

/** Generator of generational index IDs. */
export class IdGenerator implements Generator<number> {
  private readonly generations: number[] = [];
  private readonly freeList: number[] = [];

  public get size(): number {
    return this.generations.length - this.freeList.length;
  }

  public clear(): void {
    this.generations.length = 0;
    this.freeList.length = 0;
  }

  public add(): number {
    let index: number;
    let generation: number;

    if (this.freeList.length > 0) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      index = this.freeList.pop()!;
      generation = Math.abs(this.generations[index]);
      this.generations[index] = generation;
    } else {
      index = this.generations.length;
      generation = index ? 0 : 1; // avoids 0 Id as it is usually reserved for null.
      this.generations.push(generation);
    }

    return id(index, generation);
  }

  public delete(id: number): boolean {
    if (!this.has(id)) {
      return false;
    }

    const index = indexOf(id);
    // Use negative sign to indicate free space
    let generation = -((this.generations[index] + 1) & MAX_SAFE_GENERATION);
    if (!index && !generation) {
      ++generation; // avoids 0 Id as it is reserved for null.
    }

    this.generations[index] = generation;
    this.freeList.push(index);

    return true;
  }

  public forEach(action: (id: number) => void): void {
    for (const id of this.values()) {
      action(id);
    }
  }

  public has(id: number): boolean {
    return (indexOf(id) < this.generations.length
      && generationOf(id) === this.generations[indexOf(id)]);
  }

  public keys(): IterableIterator<number> {
    return this.values();
  }

  public * values(): IterableIterator<number> {
    for (let i = 0; i < this.generations.length; ++i) {
      const generation = this.generations[i];
      if (this.generations[i] >= 0) {
        yield id(i, generation);
      }
    }
  }

  public [Symbol.iterator](): IterableIterator<number> {
    return this.values();
  }
}
