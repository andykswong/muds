/**
 * @jest-environment jsdom
 */

import { jest } from '@jest/globals';

import { OnAnimationFrame } from '../time';

const FRAME_TIME = 16; // fake timer uses 16ms per frame

describe('OnAnimationFrame', () => {
  let timer: OnAnimationFrame;

  beforeAll(() => {
    jest.useFakeTimers();
  });

  beforeEach(() => {
    timer = new OnAnimationFrame();
  });

  afterEach(() => {
    timer.pause();
  });

  it('should trigger listener on each frame', () => {
    const listener = jest.fn();

    timer.addListener(listener);
    timer.start();

    jest.advanceTimersToNextTimer();

    expect(listener).toBeCalledTimes(1);
    expect(listener).toHaveBeenCalledWith(0);

    jest.advanceTimersToNextTimer();

    expect(listener).toBeCalledTimes(2);
    expect(listener).toHaveBeenLastCalledWith(FRAME_TIME);
  });


  it('should not trigger listener duplicately when start is called twice', () => {
    const listener = jest.fn();

    timer.addListener(listener);
    timer.start();
    timer.start();

    jest.advanceTimersToNextTimer();

    expect(listener).toBeCalledTimes(1);
  });

  it('should not trigger listener on pause', () => {
    const listener = jest.fn();
    timer.addListener(listener);
    timer.start();

    jest.advanceTimersToNextTimer();

    expect(listener).toBeCalledTimes(1);

    timer.pause();
    jest.advanceTimersToNextTimer();
    jest.advanceTimersToNextTimer();

    expect(listener).toBeCalledTimes(1);
  });
});
