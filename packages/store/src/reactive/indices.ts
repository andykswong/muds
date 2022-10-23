import { ReadonlyESMap } from 'typescript';

import { MUDS_DEBUG } from '../config';
import { CollectionEvents } from './collection';

/** A unique key index derived from a collection. */
export interface UniqueIndex<U, K> extends ReadonlyESMap<U, K>, Iterable<[U, K]> {
}

/** A unique key index derived from a collection. */
export class UniqueIndex<U, K> implements UniqueIndex<U, K> {
  /** Creates a UniqueIndex from an observable collection and a unique key extraction function. */
  public static fromCollection<K, V, U>(
    collection: CollectionEvents<K, V>, uniqueKey: (key: K, value: V) => U
  ): UniqueIndex<U, K> {
    const index = new Map<U, K>();
    collection.onClear.addListener(() => index.clear());
    collection.onDelete.addListener((_, key, value) => index.delete(uniqueKey(key, value)));
    collection.onAdd.addListener((_, key, value) => setUnique(index, uniqueKey, key, value));
    collection.onUpdate?.addListener((_, key, value, oldValue) => {
      index.delete(uniqueKey(key, oldValue));
      setUnique(index, uniqueKey, key, value);
    });

    return index;
  }
}

function setUnique<U, K, V>(index: Map<U, K>, uniqueKey: (key: K, value: V) => U, key: K, value: V) {
  const ukey = uniqueKey(key, value);
  MUDS_DEBUG && assertUnique(index, ukey);
  index.set(ukey, key);
}

function assertUnique<U, K>(index: Map<U, K>, key: U) {
  console.assert(!index.has(key), `Duplicate keys with unique index: ${key}`);
}
