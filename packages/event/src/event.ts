/** Listener of events. */
export interface EventListener<T extends unknown[]> {
  (...data: T): void;
}

/** A typesafe event signal. */
export interface Event<T extends unknown[]> {
  /** Adds an event listener. */
  addListener(listener: EventListener<T>): void;

  /** Removes an event listener. */
  removeListener(listener: EventListener<T>): void;

  /** Emits an event. */
  emit(...data: T): void;
}

/** A typesafe event signal. */
export class Event<T> implements Event<T> {
  /** Creates a simple event instance. */
  public static create<T extends unknown[]>(): Event<T> {
    return new SimpleEvent();
  }
}

/** A simple Event implementation that triggers listeners immediately upon emitting. */
export class SimpleEvent<T extends unknown[]> implements Event<T> {
  private readonly listeners: EventListener<T>[] = [];

  public addListener(listener: EventListener<T>) {
    this.listeners.push(listener);
  }

  public removeListener(listener: EventListener<T>) {
    const index = this.listeners.indexOf(listener);
    if (index > -1) {
      this.listeners[index] = this.listeners[this.listeners.length - 1];
      this.listeners.pop();
    }
  }

  public emit(...data: T) {
    for (const listener of this.listeners) {
      listener(...data);
    }
  }
}
