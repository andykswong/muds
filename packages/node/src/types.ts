/**
 * Getter interface for unique key index.
 * @internal
 */
export interface UniqueIndexGet<K, V> {
  /** Gets a value by key. */
  get(key: K): V | undefined;
}

/**
 * Getter and setter interface for unique key index.
 * @internal
 */
export interface UniqueIndexGetSet<K, V> extends UniqueIndexGet<K, V> {
  /** Sets a key-value pair. */
  set(key: K, value: V): void;
}
