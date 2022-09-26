import { jest } from '@jest/globals';

import { mapDispatch, mapEffect } from '../map';
import { Dispatch, Effect } from '../types';

enum Action {
  First = 1,
  Second = 2,
}

enum MappedAction {
  Third = 3,
  Fourth = 4,
}

const mapAction = (action: Action) => {
  switch (action) {
    case Action.First: return MappedAction.Third;
    default: return MappedAction.Fourth;
  }
}

describe('mapDispatch', () => {
  it('should return a function that takes an action, map it, and forward it to original dispatch function', () => {
    const mockDispatch = jest.fn() as Dispatch<MappedAction>;

    const mappedDispatch = mapDispatch(mockDispatch, mapAction);

    mappedDispatch(Action.First);

    expect(mockDispatch).toBeCalledTimes(1);
    expect(mockDispatch).toHaveBeenCalledWith(MappedAction.Third);
  });
});

describe('mapEffect', () => {
  it('should return an effect that takes dispatch function for mapped action', () => {
    const mockDispatch = jest.fn() as Dispatch<MappedAction>;

    const effect: Effect<Action> = (dispatch) => dispatch(Action.First);
    const mappedEffect = mapEffect(effect, mapAction)!;

    mappedEffect(mockDispatch);

    expect(mockDispatch).toBeCalledTimes(1);
    expect(mockDispatch).toHaveBeenCalledWith(MappedAction.Third);
  });
});
