import { ModelEffect, ViewableSystem } from './types';

/**
 * Executes a system and returns the execution state.
 */
export function execute<Model, Action, View, Props>(
  system: ViewableSystem<Model, Action, View, Props>,
  props: Props
): Runtime<Model, Action, View> {
  const runtime = {
    state: RuntimeState.Running,
    model: null as Model | null,
    view: null as View | null,

    dispatch(action: Action) {
      if (this.state === RuntimeState.Running) {
        applyUpdate(system.update(this.model!, action));
      }
    },

    destroy() {
      if (this.state === RuntimeState.Running) {
        this.state = RuntimeState.Destroyed;
        system.destroy?.(this.model!);
        this.model = this.view = null;
      }
    }
  };
  const dispatch = runtime.dispatch.bind(runtime);

  function applyUpdate([newModel, effect]: ModelEffect<Model, Action>) {
    runtime.model = newModel;
    effect?.(dispatch);
    if (system.view) {
      runtime.view = system.view(runtime.model, dispatch);
    }
  }

  // Applies initial model and effect
  applyUpdate(system.create(props));

  return runtime;
}

/**
 * The runtime context of a system.
 */
export interface Runtime<Model, Action, View> {
  /**
   * Returns the state of the system.
   */
  get state(): RuntimeState;

  /**
   * Returns the current model.
   */
  get model(): Model | null;

  /**
   * Returns the current view.
   */
  get view(): View | null;

  /**
   * Dispatches an action to the system.
   */
  dispatch(action: Action): void;

  /**
   * Stops and destroys an active system. Does nothing if system is not active.
   */
  destroy(): void;
}

/**
 * The runtime state of a system.
 */
export enum RuntimeState {
  New = 0,
  Running = 1,
  Destroyed = 2,
}
