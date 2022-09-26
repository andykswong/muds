import React from 'react';
import { Dispatch, mapDispatch, ModelEffect } from '@muds/runtime';

import { CounterAction, Counter } from './counter';

export type CountersModel = number[];

export interface CountersAction {
  type: CounterAction,
  counterId: number,
}

function countersAction(counterId: number, type: CounterAction): CountersAction {
  return { type, counterId };
}

export const Counters = {
  create: (counts: CountersModel = [0, 0]): ModelEffect<CountersModel, CountersAction> => [counts],

  update: (counts: CountersModel, action: CountersAction): ModelEffect<CountersModel, CountersAction> => {
    const [newCount] = Counter.update(counts[action.counterId], action.type);
    counts[action.counterId] = newCount;
    return [counts];
  },

  view: (counts: CountersModel, dispatch: Dispatch<CountersAction>): React.ReactElement => {
    return (
      <div>
        <p>List of counters:</p>
        {counts.map((count, i) => (
          <p key={i}>
            Counter {i}: {Counter.view(count, mapDispatch(dispatch, countersAction.bind(null, i)))}
          </p>
        ))}
      </div>
    );
  },
};
