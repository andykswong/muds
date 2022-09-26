/**
 * Function to dispatch an action.
 */
export type Dispatch<Action> = (action: Action) => void;

/**
 * A side-effect function that can possibly dispatch actions.
 */
export type Effect<Action> = (Dispatch: Dispatch<Action>) => void;

/**
 * Pair of model and effect.
 */
export type ModelEffect<Model, Action> = readonly [Model, Effect<Action>?];

/**
 * A process which acts on a model from given actions.
 */
export interface System<Model, Action, Props> {
  /**
   * Creates a new model instance, possibly producing a side effect.
   */
  create(props: Props): ModelEffect<Model, Action>;

  /**
   * Optional deconstructor to clean up a model instance.
   */
  destroy?(model: Model): void;

  /**
   * Updates a model from given action, possibly producing a side effect.
   */
  update(model: Model, action: Action): ModelEffect<Model, Action>;
}

/**
 * A system that provides an interactive view.
 */
export interface ViewableSystem<Model, Action, View, Props> extends System<Model, Action, Props> {
  /**
   * Presents an interactive view, either by returning a view, or by side effects.
   */
  view?(model: Model, dispatch: Dispatch<Action>): View;
}
