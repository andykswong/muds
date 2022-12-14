import { MapGet } from './types';

/**
 * The RHS values of a unique join.
 * @internal
 */
export type UniqueJoinRHS<K, T extends MapGet<K, unknown>[]> = { [K in keyof T]: ReturnType<T[K]['get']> }

/** Left joins a tuple iterator with unique indices. */
export function* uniqueJoin<L extends readonly unknown[], K, I extends MapGet<K, unknown>[]>(
  iter: Iterator<L>, key: (lhs: L) => K | undefined, ...indices: I
): IterableIterator<[...L, ...UniqueJoinRHS<K, I>]> {
  let left = iter.next();
  while (!left.done) {
    const joinKey = key(left.value);
    const item = [...left.value];
    for (const index of indices) {
      item.push(joinKey !== undefined ? index.get(joinKey) : undefined);
    }
    yield item as [...L, ...UniqueJoinRHS<K, I>];
    left = iter.next();
  }
  iter.return?.();
}
