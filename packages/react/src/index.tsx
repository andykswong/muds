import React, { ReactElement } from 'react';
import { Root } from 'react-dom/client';

import { Dispatch, ViewableSystem } from '@muds/runtime';

/**
 * High-order system, which takes a system that returns React component view, and wraps it with React rendering logic.
 */
export function ReactSystem<Model, Action, Props>(
  system: ViewableSystem<Model, Action, ReactElement, Props>
): ViewableSystem<ReactSystemModel<Model>, Action, void, ReactSystemProps<Props>> {
  return {
    create: (props) => {
      const [model, effect] = system.create(props.props);
      return [
        { model, root: props.root }, effect];
    },

    destroy: (model) => {
      model.root.unmount();
      system.destroy?.(model.model);
    },

    update: (model, action) => {
      const [newModel, effect] = system.update(model.model, action);
      return [{ ...model, model: newModel }, effect];
    },

    view: (model, dispatch) => {
      model.root.render(
        <Container system={system} model={model.model} dispatch={dispatch} />
      );
    }
  };
}

export interface ReactSystemProps<Props> {
  root: Root,
  props: Props,
}

export interface ReactSystemModel<Model> {
  root: Root,
  model: Model,
}

function Container<Model, Action, Props>(
  props: {
    system: ViewableSystem<Model, Action, ReactElement, Props>,
    model: Model,
    dispatch: Dispatch<Action>
  }
) {
  return props.system.view?.(props.model, props.dispatch) || null;
}
