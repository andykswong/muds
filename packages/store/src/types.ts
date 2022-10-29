/**
 * Getter interface for a key-value map.
 */
export interface MapGet<K, V> {
  /** Gets a value by key. */
  get(key: K): V | undefined;
}

/**
 * Getter and setter interface for a key-value map.
 */
export interface MapGetSet<K, V> extends MapGet<K, V> {
  /** Sets a key-value pair. */
  set(key: K, value: V): void;
}
