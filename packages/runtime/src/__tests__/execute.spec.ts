import { jest } from '@jest/globals';

import { execute, RuntimeState } from '../execute';
import { Dispatch, ModelEffect } from '../types';

enum Action {
  First,
  Second,
}

type Model = string;
type Props = number;
type ME = ModelEffect<Model, Action>;

const MODEL = 'MODEL';
const VIEW = 'VIEW';
const PROPS = 123;

describe('execute', () => {
  it('should initialize system model and run initial effect', () => {
    const mockInitialEffect = jest.fn();
    const mockCreate = jest.fn().mockReturnValueOnce([MODEL, mockInitialEffect]);

    const runtime = execute({
      create: mockCreate as ((props: Props) => ME),
      update: () => [MODEL],
    }, PROPS);

    expect(runtime.state).toBe(RuntimeState.Running);
    expect(runtime.model).toBe(MODEL);
    expect(mockCreate).toHaveBeenCalledWith(PROPS);
    expect(mockInitialEffect).toBeCalledTimes(1);
  });

  it('should dispatch action from initial effect', () => {
    const mockUpdate = jest.fn().mockReturnValue([MODEL]);

    execute({
      create: () => [MODEL, (dispatch) => dispatch(Action.First)],
      update: mockUpdate as ((model: Model, action: Action) => ME),
    }, PROPS);

    expect(mockUpdate).toBeCalledTimes(1);
    expect(mockUpdate).toHaveBeenLastCalledWith(MODEL, Action.First);
  });

  it('should render initial view', () => {
    const mockView = jest.fn().mockReturnValueOnce(VIEW);

    const runtime = execute({
      create: () => [MODEL],
      update: () => [MODEL],
      view: mockView,
    }, PROPS);

    expect(runtime.view).toBe(VIEW);
    expect(mockView).toBeCalledTimes(1);
    expect(mockView.mock.calls[0][0]).toBe(MODEL);
  });

  it('should supply valid dispatch function to view', () => {
    let dispatchFn: Dispatch<Action>;
    const newModel = 'NEW_MODEL';
    const mockView = jest.fn((_, dispatch) => {
      dispatchFn = dispatch;
      return VIEW;
    });
    const mockUpdate = jest.fn().mockReturnValueOnce([newModel]) as ((model: Model, action: Action) => ME);

    execute({
      create: () => [MODEL],
      update: mockUpdate,
      view: mockView,
    }, PROPS);

    expect(dispatchFn!).toBeDefined();
    expect(mockUpdate).not.toBeCalled();

    dispatchFn!(Action.First);

    expect(mockUpdate).toHaveBeenLastCalledWith(MODEL, Action.First);
    expect(mockView).toBeCalledTimes(2);
    expect(mockView.mock.calls[0][0]).toBe(MODEL);
    expect(mockView.mock.calls[1][0]).toBe(newModel);
  });

  describe('Runtime', () => {
    it('should dispatch action to system', () => {
      const newModel = 'NEW_MODEL';
      const mockUpdate = jest.fn().mockReturnValueOnce([newModel]) as ((model: Model, action: Action) => ME);

      const runtime = execute({
        create: () => [MODEL],
        update: mockUpdate,
        view: () => VIEW,
      }, PROPS);

      runtime.dispatch(Action.First);
      expect(runtime.model).toBe(newModel);
      expect(mockUpdate).toHaveBeenLastCalledWith(MODEL, Action.First);
    });

    it('should run effect from update', () => {
      const mockUpdate = jest.fn((_, action) => {
        if (action === Action.First) {
          return [MODEL, (dispatch: Dispatch<Action>) => dispatch(Action.Second)];
        }
        return [MODEL];
      }) as unknown as ((model: Model, action: Action) => ME);

      const runtime = execute({
        create: () => [MODEL],
        update: mockUpdate,
      }, PROPS);

      runtime.dispatch(Action.First);

      expect(mockUpdate).toBeCalledTimes(2);
      expect(mockUpdate).toHaveBeenCalledWith(MODEL, Action.First);
      expect(mockUpdate).toHaveBeenLastCalledWith(MODEL, Action.Second);
    });

    it('can be destroyed only once', () => {
      const mockDestroy = jest.fn();

      const runtime = execute({
        create: () => [MODEL],
        update: () => [MODEL],
        view: () => VIEW,
        destroy: mockDestroy,
      }, PROPS);

      runtime.destroy();

      expect(runtime.state).toBe(RuntimeState.Destroyed);
      expect(runtime.model).toBeNull();
      expect(runtime.view).toBeNull();
      expect(mockDestroy).toBeCalledTimes(1);

      runtime.destroy();
      expect(mockDestroy).toBeCalledTimes(1);
    });

    it('should not dispatch to destroyed runtime', () => {
      const mockUpdate = jest.fn().mockReturnValue([MODEL]) as ((model: Model, action: Action) => ME);

      const runtime = execute({
        create: () => [MODEL],
        update: mockUpdate,
        view: () => VIEW,
      }, PROPS);

      runtime.destroy();

      runtime.dispatch(Action.First);
      expect(mockUpdate).not.toBeCalled();
    });
  });
});
