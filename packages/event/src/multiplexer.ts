import { Event, EventListener, SimpleEvent } from './event';

/** Function to map a source event to output event. */
type EventMapper<E extends unknown[], T> = (eventData: E) => T;

/** Multiplexer of events that allows listening on multiple events at once. */
export class EventMultiplexer<T extends unknown[]> extends SimpleEvent<T> implements Event<T> {
  private eventListeners: Map<Event<unknown[]>, EventListener<unknown[]>> = new Map();

  /** Adds an event source to this multiplexer. */
  public add<E extends unknown[]>(event: Event<E>, mapEvent: EventMapper<E, T>): void {
    const listener = (...eventData: E) => this.emit(...mapEvent(eventData));
    event.addListener(listener);
    this.eventListeners.set(event, listener as EventListener<unknown[]>);
  }

  /** Removes an event source from this multiplexer. */
  public remove<E extends unknown[]>(event: Event<E>): void {
    const listener = this.eventListeners.get(event);
    listener && event.removeListener(listener);
  }
}
