/** Getter interface for unique key index. */
export interface UniqueIndexGetter<K, V> {
  /** Gets a value by key. */
  get(key: K): V | undefined;
}

type KeyFn<V, K> = (value: V) => K | undefined;
type JoinedValues<K, T extends UniqueIndexGetter<K, unknown>[]> = { [K in keyof T]: ReturnType<T[K]['get']> }

/** Left joins a tuple iterator with unique indices. */
export function* uniqueJoin<L extends readonly unknown[], K, I extends UniqueIndexGetter<K, unknown>[]>(
  iter: Iterator<L>, key: KeyFn<L, K>, ...indices: I
): IterableIterator<[...L, ...JoinedValues<K, I>]> {
  let left = iter.next();
  while (!left.done) {
    const joinKey = key(left.value);
    const item = [...left.value];
    for (const index of indices) {
      item.push(joinKey !== undefined ? index.get(joinKey) : undefined);
    }
    yield item as [...L, ...JoinedValues<K, I>];
    left = iter.next();
  }
  iter.return?.();
}
