/** A readonly list. */
export interface ReadonlyList<T> extends Iterable<T> {
  /** Size of this container. */
  readonly size: number;

  /** Gets a value by index. Returns undefined if there is no such entry. */
  get(index: number): T | undefined;

  /** Returns an Iterator for all entries in the list. */
  entries(): Iterator<[number, T]>;

  /** Returns an Iterator for all values in the list. */
  values(): Iterator<T>;

  /** Calls `action` once for each entry in the arena. */
  forEach(action: (value: T, index: number) => void): void;
}

/** A stack. */
export interface Stack<T> {
  /** Size of this container. */
  readonly size: number;

  /** The last element. */
  readonly last: T | undefined;

  /** Clears the container. */
  clear(): void;

  /** Removes and returns the last element. */
  pop(): T | undefined;

  /** Adds an element to the end. */
  push(value: T): void;
}

/** A queue. */
export interface Queue<T> {
  /** Size of this container. */
  readonly size: number;

  /** The first element. */
  readonly first: T | undefined;

  /** Clears the container. */
  clear(): void;

  /** Removes and returns the first element. */
  shift(): T | undefined;

  /** Adds an element to the end. */
  push(value: T): void;
}

/** A list. */
export interface List<T> extends ReadonlyList<T>, Stack<T> {
  /** Clears the container. */
  clear(): void;

  /** Sets the value at given index. */
  set(index: number, value: T): void;
}

/** Array implementation of a list. */
export class ArrayList<T> implements List<T> {
  private data: T[] = [];

  public get size(): number {
    return this.data.length;
  }

  public get last(): T | undefined {
    return this.data[this.data.length - 1];
  }

  public get(index: number): T | undefined {
    return this.data[index];
  }

  public pop(): T | undefined {
    return this.data.pop();
  }

  public push(value: T): void {
    this.data.push(value);
  }

  public clear(): void {
    this.data.length = 0;
  }

  /** Sets the value at given index and returns this. */
  public set(index: number, value: T): this {
    this.data[index] = value;
    return this;
  }

  public entries(): IterableIterator<[number, T]> {
    return this.data.entries();
  }

  public values(): IterableIterator<T> {
    return this.data.values();
  }

  public forEach(action: (value: T, index: number) => void): void {
    return this.data.forEach(action);
  }

  public [Symbol.iterator](): IterableIterator<T> {
    return this.data.values();
  }
}
