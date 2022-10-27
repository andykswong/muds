import { useState, useEffect, useRef } from 'react';
import { Event, EventListener } from '@muds/event';

/** Hook to listen to an event. */
export function useEventListener<E extends unknown[]>(
  event: Event<E>,
  listener: EventListener<E>,
): void {
  const savedListener = useRef<EventListener<E>>();

  useEffect(() => {
    savedListener.current = listener;
  }, [listener]);

  useEffect(() => {
    function onEvent(...event: E) {
      savedListener.current?.(...event);
    }
    event.addListener(onEvent);
    return () => event.removeListener(onEvent);
  }, [event]);
}

/** Hook to derive state from event. */
export function useEventReducer<S, E extends unknown[]>(
  event: Event<E>,
  reducer: (state: S, ...event: E) => S,
  initialState: S | (() => S),
): S {
  const [state, setState] = useState(initialState);
  const onEvent = useRef<EventListener<E>>();

  useEffect(() => {
    onEvent.current = (...event: E) => {
      setState(reducer(state, ...event));
    };
  }, [state, setState, reducer]);

  useEffect(() => {
    const listener = (...event: E) => onEvent.current?.(...event);
    event.addListener(listener);
    return () => event.removeListener(listener);
  }, [event]);

  return state;
}
