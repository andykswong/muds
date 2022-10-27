import { jest } from '@jest/globals';
import React, { useCallback, useState } from 'react';
import { create, act, ReactTestRenderer } from 'react-test-renderer';
import { Event } from '@muds/event';
import { useEventListener, useEventReducer } from '../event';

enum Action {
  Increment = 'inc',
  Decrement = 'dec',
}

function reducer(count: number, action: Action): number {
  switch (action) {
    case Action.Increment: return count + 1;
    case Action.Decrement: return count - 1;
    default: return count;
  }
}

describe('useEventListener', () => {
  function Counter({ event, initialCount = 0 }: { event: Event<[Action]>, initialCount?: number }) {
    const [count, setCount] = useState(initialCount);
    useEventListener(event, useCallback((action) => setCount(reducer(count, action)), [count, setCount]));
    return (<span>{count}</span>);
  }

  it('should add/remove event listener on mount/unmount', () => {
    const event = Event.create<[Action]>();

    const addListener = jest.spyOn(event, 'addListener');
    const removeListener = jest.spyOn(event, 'removeListener');

    let component!: ReactTestRenderer;

    act(() => {
      component = create(<Counter event={event} />);
    });

    expect(addListener).toHaveBeenCalled();
    const listener = addListener.mock.calls[0][0];

    act(() => component.unmount());
    expect(removeListener).toHaveBeenCalledWith(listener);
  });

  it('should add listener to event', () => {
    const event = Event.create<[Action]>();

    let component!: ReactTestRenderer;

    act(() => {
      component = create(<Counter event={event} />);
    });
    expect(component.toJSON()).toMatchSnapshot();

    for (const action of [Action.Increment, Action.Increment, Action.Decrement]) {
      act(() => event.emit(action));
      expect(component.toJSON()).toMatchSnapshot();
    }
  });
});

describe('useEventReducer', () => {
  function Counter({ event, initialCount = 0 }: { event: Event<[Action]>, initialCount?: number }) {
    const count = useEventReducer(event, reducer, initialCount);
    return (
      <span>
        <button onClick={() => event.emit(Action.Decrement)}>-</button>
        {` ${count} `}
        <button onClick={() => event.emit(Action.Increment)}>+</button>
      </span>
    );
  }

  it('should add/remove event listener on mount/unmount', () => {
    const event = Event.create<[Action]>();

    const addListener = jest.spyOn(event, 'addListener');
    const removeListener = jest.spyOn(event, 'removeListener');

    let component!: ReactTestRenderer;

    act(() => {
      component = create(<Counter event={event} />);
    });

    expect(addListener).toHaveBeenCalled();
    const listener = addListener.mock.calls[0][0];

    act(() => component.unmount());
    expect(removeListener).toHaveBeenCalledWith(listener);
  });

  it('should reduce events to state', () => {
    const event = Event.create<[Action]>();

    let component!: ReactTestRenderer;

    act(() => {
      component = create(<Counter event={event} />);
    });
    expect(component.toJSON()).toMatchSnapshot();

    for (let i = 0; i < 2; ++i) {
      act(() => {
        const [, inc] = component.root.findAllByType('button');
        inc.props.onClick();
      });
      expect(component.toJSON()).toMatchSnapshot();
    }
  });
});
