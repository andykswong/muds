import { Dispatch, Effect } from './types';

/**
 * Map a parent system's dispatch function into one that is usable by child system.
 */
export function mapDispatch<ParentAction, ChildAction>(
  dispatch: Dispatch<ParentAction>,
  mapAction: (action: ChildAction) => ParentAction
): Dispatch<ChildAction> {
  return (action) => dispatch(mapAction(action));
}

/**
 * Map a child system's effect into parent system's effect.
 */
 export function mapEffect<ParentAction, ChildAction>(
  effect: Effect<ChildAction> | undefined,
  mapAction: (action: ChildAction) => ParentAction
): Effect<ParentAction> | undefined {
  return effect &&
    ((dispatch) => effect(mapDispatch(dispatch, mapAction)));
}
