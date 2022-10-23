import { Event } from '@muds/event';
import { CollectionEvents } from '../collection';

export function mockCollectionEvents<K, V>(hasOnUpdate = true): CollectionEvents<K, V> {
  return {
    onClear: Event.create(),
    onAdd: Event.create(),
    onDelete: Event.create(),
    onUpdate: hasOnUpdate ? Event.create() : undefined,
  };
}
