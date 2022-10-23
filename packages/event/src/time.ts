import { Event, SimpleEvent } from './event';

/** A timer event that is triggered every set period of time, passing the delta time in milliseconds to listeners. */
export interface TimerEvent extends Event<[number]> {
  /** Starts the timer. */
  start(): void;

  /** Pauses the timer. */
  pause(): void;
}

/** Event that triggers on each animation frame. */
export class OnAnimationFrame extends SimpleEvent<[number]> implements TimerEvent {
  private lastTime = 0;
  private raf = 0;

  private onFrame = (time: number) => {
    this.raf = requestAnimationFrame(this.onFrame);
    const delta = this.lastTime ? time - this.lastTime : 0;
    this.lastTime = time;
    this.emit(delta);
  };

  public start(): void {
    if (!this.raf) {
      this.lastTime = 0;
      this.raf = requestAnimationFrame(this.onFrame);
    }
  }

  public pause(): void {
    cancelAnimationFrame(this.raf);
    this.raf = 0;
  }
}
