import { jest } from '@jest/globals';

import { Event } from '../event';
import { EventMultiplexer } from '../multiplexer';

const SOURCE_DATA = 12;
const SOURCE_DATA2 = 'hello';
const MAPPER1 = ([i]: [number]) => [i * 13] as [number];
const MAPPER2 = ([s]: [string]) => [s.length] as [number];
const [DATA1] = MAPPER1([SOURCE_DATA]);
const [DATA2] = MAPPER2([SOURCE_DATA2]);

describe('EventMultiplexer', () => {
  it('should trigger listeners on emit', () => {
    const listener1 = jest.fn();

    const event = new EventMultiplexer<[number]>();
    event.addListener(listener1);
    event.emit(DATA1);

    expect(listener1).toBeCalledTimes(1);
    expect(listener1).toHaveBeenCalledWith(DATA1);

    event.removeListener(listener1);
    event.emit(DATA1);
    expect(listener1).toBeCalledTimes(1);
  });

  it('should trigger listeners when added event emits', () => {
    const listener1 = jest.fn();
    const sourceEvent1 = Event.create<[number]>();
    const sourceEvent2 = Event.create<[string]>();
    const event = new EventMultiplexer<[number]>();

    event.add(sourceEvent1, MAPPER1);
    event.addListener(listener1);
  
    sourceEvent1.emit(SOURCE_DATA);

    expect(listener1).toBeCalledTimes(1);
    expect(listener1).toHaveBeenCalledWith(DATA1);

    event.add(sourceEvent2, MAPPER2);
    sourceEvent2.emit(SOURCE_DATA2);
  
    expect(listener1).toBeCalledTimes(2);
    expect(listener1).toHaveBeenLastCalledWith(DATA2);
  });

  it('should not trigger listeners for removed event', () => {
    const listener1 = jest.fn();
    const sourceEvent = Event.create<[number]>();
    const event = new EventMultiplexer<[number]>();

    event.add(sourceEvent, MAPPER1);
    event.addListener(listener1);
    event.remove(sourceEvent);
  
    sourceEvent.emit(SOURCE_DATA);

    expect(listener1).not.toHaveBeenCalled();
  });
});
