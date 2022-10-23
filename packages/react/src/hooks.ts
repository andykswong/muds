import { useState, useEffect } from 'react';
import { Event } from '@muds/event';

/** Hook to derive state from event. */
export function useEventReducer<S, E extends unknown[]>(
  event: Event<E>,
  reducer: (state: S, ...event: E) => S,
  initialState: S | (() => S),
): S {
  const [state, setState] = useState(initialState);

  useEffect(() => {
    event.addListener(onChange);
    return () => event.removeListener(onChange);
  }, []);

  function onChange(...event: E) {
    setState(reducer(state, ...event));
  }

  return state;
}
