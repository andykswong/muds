import { Stack, Queue, List } from './list';

/** A double-ended queue with constant-time push/pop on both ends. */
export interface Deque<T> extends Queue<T>, Stack<T> {
  /** Adds an element to the beginning of the deque. */
  unshift(value: T): void;
}

/** A dual array based double-ended queue. */
export class ArrayDeque<T> implements Deque<T>, List<T> {
  private readonly minLoadFactor: number;
  private readonly frontStack: (T | undefined)[] = [];
  private readonly backStack: (T | undefined)[] = [];
  private frontOffset = 0;
  private backOffset = 0;

  public constructor(minLoadFactor = 0.5) {
    this.minLoadFactor = Math.max(0, Math.min(minLoadFactor, 1));
  }

  public get size(): number {
    return (this.frontStack.length - this.frontOffset) + (this.backStack.length - this.backOffset);
  }

  public get first(): T | undefined {
    if (this.size === 0) {
      return undefined;
    }

    if (this.frontStack.length) {
      return this.frontStack[this.frontStack.length - 1];
    }

    return this.backStack[this.backOffset];
  }

  public get last(): T | undefined {
    if (this.size === 0) {
      return undefined;
    }

    if (this.backStack.length) {
      return this.backStack[this.backStack.length - 1];
    }
    return this.frontStack[this.frontOffset];
  }

  public clear(): void {
    this.backStack.length = this.frontStack.length = 0;
    this.backOffset = this.frontOffset = 0;
  }

  public unshift(value: T): this {
    if (this.backOffset) {
      this.backStack[this.backOffset--] = value;
    } else {
      this.frontStack.push(value);
    }
    return this;
  }

  public push(value: T): this {
    if (this.frontOffset) {
      this.frontStack[this.frontOffset--] = value;
    } else {
      this.backStack.push(value);
    }
    return this;
  }

  public shift(): T | undefined {
    if (this.frontStack.length - this.frontOffset) {
      return this.frontStack.pop();
    }

    if (this.backStack.length - this.backOffset) {
      const [first, backOffset] = dequeueAndResize(this.backStack, this.backOffset, this.minLoadFactor);
      this.backOffset = backOffset;
      return first;
    }

    return undefined;
  }

  public pop(): T | undefined {
    if (this.backStack.length - this.backOffset) {
      return this.backStack.pop();
    }

    if (this.frontStack.length - this.frontOffset) {
      const [last, frontOffset] = dequeueAndResize(this.frontStack, this.frontOffset, this.minLoadFactor);
      this.frontOffset = frontOffset;
      return last;
    }

    return undefined;
  }

  public get(index: number): T | undefined {
    const frontSize = this.frontStack.length - this.frontOffset;
    if (index >= frontSize) {
      return this.backStack[this.backOffset + index - frontSize];
    } else {
      return this.frontStack[this.frontStack.length - 1 - index];
    }
  }

  public has(index: number): boolean {
    return Number.isInteger(index) && index >= 0 && index < this.size;
  }

  public set(index: number, value: T): void {
    const frontSize = this.frontStack.length - this.frontOffset;
    if (index >= frontSize) {
      this.backStack[this.backOffset + index - frontSize] = value;
    } else {
      this.frontStack[this.frontStack.length - 1 - index] = value;
    }
  }

  public * keys(): IterableIterator<number> {
    const size = this.size;
    for (let i = 0; i < size; ++i) {
      yield i;
    }
  }

  public * values(): IterableIterator<T> {
    for (let i = this.frontStack.length - 1; i >= this.frontOffset; --i) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      yield this.frontStack[i]!;
    }
    for (let i = this.backOffset; i < this.backStack.length; ++i) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      yield this.backStack[i]!;
    }
  }

  public * entries(): IterableIterator<[number, T]> {
    let index = 0;
    for (const value of this.values()) {
      yield [index++, value];
    }
  }

  public forEach(action: (value: T, index: number) => void): void {
    let index = 0;
    for (const value of this.values()) {
      action(value, index++);
    }
  }

  public [Symbol.iterator](): IterableIterator<T> {
    return this.values();
  }
}

function dequeueAndResize<T>(
  stack: (T | undefined)[], offset: number, minLoadFactor: number
): [front: T | undefined, newOffset: number] {
  const first = stack[offset++];

  const loadFactor = 1 - offset / stack.length;
  if (loadFactor < minLoadFactor) {
    stack.copyWithin(0, offset);
    stack.length -= offset;
    offset = 0;
  } else {
    stack[offset - 1] = undefined;
  }

  return [first, offset];
}
