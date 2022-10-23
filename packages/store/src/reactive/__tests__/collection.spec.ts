import { jest } from '@jest/globals';
import { Event } from '@muds/event';
import { CollectionEvents, onCollectionChange } from '../collection';
import { mockCollectionEvents } from './mocks';

type Collection = CollectionEvents<number, string>;
type EventData = [] | [number, string] | [number, string, string];

describe('onCollectionChange', () => {
  it.each([
    ['add', (collection: Collection) => collection.onAdd, [1, 'a'] as EventData],
    ['clear', (collection: Collection) => collection.onClear, [] as EventData],
    ['delete', (collection: Collection) => collection.onDelete, [1, 'a'] as EventData],
    ['update', (collection: Collection) => collection.onUpdate, [1, 'b', 'a'] as EventData],
  ])('should trigger listener on collection %s', (eventType, sourceEvent, eventData: EventData) => {
    const collection: CollectionEvents<number, string> = mockCollectionEvents(eventType === 'update');
    const listener = jest.fn();

    const event = onCollectionChange(collection);
    event.addListener(listener);

    (sourceEvent(collection) as Event<unknown[]> | undefined)?.emit(collection, ...eventData);

    expect(listener).toBeCalledTimes(1);
    expect(listener).toHaveBeenCalledWith(eventType, collection, ...eventData);
  });

  it('should not trigger unsubscribed listener', () => {
    const collection = mockCollectionEvents();
    const listener = jest.fn();

    const event = onCollectionChange(collection);
    event.addListener(listener);
    event.removeListener(listener);

    collection.onClear.emit(collection);

    expect(listener).not.toHaveBeenCalled();
  });
});
