import { Event, EventMultiplexer } from '@muds/event';

/** Collection change events. */
export interface CollectionEvents<K, V> {
  /** Event to be triggered when the collection is cleared. */
  readonly onClear: Event<[thisArg: this]>;

  /** Event to be triggered when collection entry is added. */
  readonly onAdd: Event<[thisArg: this, key: K, value: V]>;

  /** Event to be triggered when collection entry is deleted. */
  readonly onDelete: Event<[thisArg: this, key: K, value: V]>;

  /** Optional event to be triggered when collection entry is updated. */
  readonly onUpdate?: Event<[thisArg: this, key: K, value: V, prevValue: V]>;
}

export type CollectionChangeEvent<K, V, T extends CollectionEvents<K, V>> =
  | [eventType: 'add', thisArg: T, key: K, value: V]
  | [eventType: 'clear', thisArg: T]
  | [eventType: 'delete', thisArg: T, key: K, value: V]
  | [eventType: 'update', thisArg: T, key: K, value: V, prevValue: V];

/** Creates a multiplexed event that listens to all changes on a collection. */
export function onCollectionChange<K, V, T extends CollectionEvents<K, V>>(collection: T): Event<CollectionChangeEvent<K, V, T>> {
  const event = new EventMultiplexer<CollectionChangeEvent<K, V, T>>();
  event.add(collection.onAdd, (e) => ['add', ...e]);
  event.add(collection.onClear, (e) => ['clear', ...e]);
  event.add(collection.onDelete, (e) => ['delete', ...e]);
  collection.onUpdate && event.add(collection.onUpdate, (e) => ['update', ...e]);
  return event;
}
