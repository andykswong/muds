import React from 'react';
import { Dispatch, ModelEffect } from '@muds/runtime';

export type CounterModel = number;

export enum CounterAction {
  Increment = 1,
  Decrement = 2,
}

export const Counter = {
  create: (count: CounterModel = 0): ModelEffect<CounterModel, CounterAction> => [count],

  update: (count: CounterModel, action: CounterAction): ModelEffect<CounterModel, CounterAction> => {
    switch (action) {
      case CounterAction.Increment: return [count + 1];
      case CounterAction.Decrement: return [count - 1];
      default: return [count];
    }
  },

  view: (count: CounterModel, dispatch: Dispatch<CounterAction>): React.ReactElement => {
    return (
      <span>
        <button onClick={() => dispatch(CounterAction.Decrement)}>-</button>
        {` ${count} `}
        <button onClick={() => dispatch(CounterAction.Increment)}>+</button>
      </span>
    );
  },
};
