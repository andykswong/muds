import { Collection, ReadonlySet } from 'typescript';
import { create as id, generationOf, indexOf, MAX_SAFE_GENERATION } from '../id';

/** Generator of values. */
export interface Generator<T> extends Collection<T>, ReadonlySet<T>, Iterable<T> {
  /** Creates a new value. */
  add(): T;

  /** Deletes a value and returns if the value originally exists */
  delete(value: T): boolean;
}

/** Generator of generational index IDs. */
export class IdGenerator<T extends number = number> implements Generator<T> {
  private readonly generations: number[] = [];
  private readonly freeList: number[] = [];

  public get size(): number {
    return this.generations.length - this.freeList.length;
  }

  public clear(): void {
    this.generations.length = 0;
    this.freeList.length = 0;
  }

  public add(): T {
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

  public delete(id: T): boolean {
    if (!this.has(id)) {
      return false;
    }

    const index = indexOf(id);
    let generation = (this.generations[index] + 1) & MAX_SAFE_GENERATION;
    if (!index && !generation) {
      ++generation; // avoids 0 Id as it is reserved for null.
    }

    // Use negative sign to indicate free space
    this.generations[index] = -generation;
    this.freeList.push(index);

    return true;
  }

  public forEach(action: (value: T, key: T) => void): void {
    for (const id of this.values()) {
      action(id, id);
    }
  }

  public has(id: T): boolean {
    return (indexOf(id) < this.generations.length
      && generationOf(id) === this.generations[indexOf(id)]);
  }

  public keys(): IterableIterator<T> {
    return this.values();
  }

  public * values(): IterableIterator<T> {
    for (let i = 0; i < this.generations.length; ++i) {
      const generation = this.generations[i];
      if (this.generations[i] >= 0) {
        yield id(i, generation);
      }
    }
  }

  public * entries(): IterableIterator<[T, T]> {
    for (const value of this.values()) {
      yield [value, value];
    }
  }

  public [Symbol.iterator](): IterableIterator<T> {
    return this.values();
  }
}
