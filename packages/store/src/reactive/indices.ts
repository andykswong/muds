import { ReadonlyESMap } from 'typescript';
import { MUDS_DEBUG } from '../config';
import { MapGet, MapGetSet, MapCRUD } from '../types';
import { CollectionEvents } from './collection';

/** A unique key index derived from a collection. */
export interface UniqueIndex<U, K> extends ReadonlyESMap<U, K>, MapGet<U, K>, Iterable<[U, K]> {
}

/** Function to collect items from collection into an index. */
export interface UniqueIndexCollector<K, V, U> {
  (index: MapCRUD<U, K>, collection: CollectionEvents<K, V>, uniqueKey: (key: K, value: V) => U): void;
}

/** A unique key index derived from a collection. */
export class UniqueIndex<U, K> implements UniqueIndex<U, K> {
  /** Creates a UniqueIndex from an observable collection. */
  public static fromCollection<K, V, U>(
    collection: CollectionEvents<K, V>,
    uniqueKey: (key: K, value: V) => U,
    collector: UniqueIndexCollector<K, V, U> = defaultCollector,
  ): UniqueIndex<U, K> {
    const index = new Map<U, K>();
    collection.onClear.addListener(() => index.clear());
    collection.onDelete.addListener((_, key, value) => index.delete(uniqueKey(key, value)));
    collection.onAdd.addListener((_, key, value) => setUnique(index, uniqueKey, key, value));
    collection.onUpdate?.addListener((_, key, value, oldValue) => {
      index.delete(uniqueKey(key, oldValue));
      setUnique(index, uniqueKey, key, value);
    });

    collector(index, collection, uniqueKey);

    return index;
  }
}

function defaultCollector<K, V, U>(
  index: MapGetSet<U, K>, collection: CollectionEvents<K, V>, uniqueKey: (key: K, value: V) => U
) {
  if ((collection as unknown as Entries<K, V>).entries) {
    for (
      let iter = (collection as unknown as Entries<K, V>).entries(), result = iter.next();
      !result.done;
      result = iter.next()
    ) {
      setUnique(index, uniqueKey, ...result.value);
    }
  }
}

function setUnique<U, K, V>(index: MapGetSet<U, K>, uniqueKey: (key: K, value: V) => U, key: K, value: V) {
  const ukey = uniqueKey(key, value);
  MUDS_DEBUG && assertUnique(index, ukey);
  index.set(ukey, key);
}

function assertUnique<U, K>(index: MapGetSet<U, K>, key: U) {
  console.assert(index.get(key) === undefined, `Duplicate keys with unique index: ${key}`);
}

/** Interface with entries() method. */
interface Entries<K, V> {
  entries(): Iterator<[K, V]>;
}
