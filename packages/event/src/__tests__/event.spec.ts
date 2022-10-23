import { jest } from '@jest/globals';

import { Event } from '../event';

const DATA = 123;

describe('Event', () => {
  it('should trigger listeners on emit', () => {
    const listener1 = jest.fn();
    const listener2 = jest.fn();

    const event = Event.create<[number]>();
    event.addListener(listener1);
    event.emit(DATA);

    expect(listener1).toBeCalledTimes(1);
    expect(listener1).toHaveBeenCalledWith(DATA);

    event.addListener(listener2);
    event.emit(DATA);

    expect(listener1).toBeCalledTimes(2);
    expect(listener2).toBeCalledTimes(1);
    expect(listener2).toHaveBeenCalledWith(DATA);
  });

  it('should not trigger removed listeners', () => {
    const listener1 = jest.fn();
    const listener2 = jest.fn();

    const event = Event.create<[number]>();
    event.addListener(listener1);
    event.addListener(listener2);
    event.emit(DATA);

    expect(listener1).toBeCalledTimes(1);
    expect(listener2).toBeCalledTimes(1);

    event.removeListener(listener1);
    event.emit(DATA);

    expect(listener1).toBeCalledTimes(1);
    expect(listener2).toBeCalledTimes(2);
  });

  it('is no-op when trying to remove listener that was not added', () => {
    const listener1 = jest.fn();
    const listener2 = jest.fn();

    const event = Event.create<[number]>();
    event.addListener(listener1);
    event.removeListener(listener2);
    event.emit(DATA);

    expect(listener1).toBeCalledTimes(1);
  });
});
