/* eslint-disable @typescript-eslint/no-explicit-any */

import { mapDispatch, mapEffect } from './map';
import { Dispatch, ViewableSystem } from './types';

export function WithDevTools<Model, Action, View, Props>(
  system: ViewableSystem<Model, Action, View, Props>,
  options?: Record<string, any>,
): ViewableSystem<WithDevToolsModel<Model, Action>, WithDevToolsAction<Action>, View, Props> {
  return {
    create: (props) => {
      const [model, effect] = system.create(props);
      const devTools = devToolsSubscription<Model, Action>(model, options);
      return [
        { model, devTools },
        (dispatch) => {
          devTools?.effect(dispatch);
          effect?.(mapDispatch(dispatch, wrapAction));
        }
      ];
    },

    update: (model, action) => {
      if (action.action) {
        const [newModel, effect] = system.update(model.model, action.action);
        model.devTools?.send(newModel, action.action);
        return [
          { ...model, model: newModel },
          mapEffect(effect, wrapAction)
        ];
      } else if (action.devSetModel) {
        return [{ ...model, model: action.devSetModel }];
      }
      return [model];
    },
  
    destroy: (model) => {
      model.devTools?.unsubscribe();
      system.destroy?.(model.model);
    },
  
    view: system.view && ((model, dispatch) => {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return system.view!(model.model, mapDispatch(dispatch, wrapAction));
    })
  };
}

function devToolsSubscription<Model, Action>(initValue: Model, options?: Record<string, any>): DevToolsSubscription<Model, Action> | null {
  const devToolsExt = (typeof window !== 'undefined' && (window as any).__REDUX_DEVTOOLS_EXTENSION__);

  if (!devToolsExt) {
    return null;
  }

  // Keep a copy of initial values
  initValue = JSON.parse(JSON.stringify(
    options?.stateSanitizer ? options.stateSanitizer(initValue) : initValue
  ));

  const devTools = devToolsExt.connect(options);
  let unsubscribeFn: (() => void) | null = null;
  let model: Model = initValue;

  devTools.init(initValue);

  return {
    effect: (dispatch: Dispatch<WithDevToolsAction<Action>>): void => {
      unsubscribeFn = devTools.subscribe((message: any) => {
        if (message.type === 'DISPATCH') {
          switch (message.payload.type) {
            case 'RESET':
              dispatch({ devSetModel: initValue });
              devTools.init(initValue);
              return;
            case 'COMMIT':
              devTools.init(model);
              return;
            case 'ROLLBACK': {
              const model = JSON.parse(message.state);
              dispatch({ devSetModel: model });
              devTools.init(model);
              return;
            }
            case 'JUMP_TO_STATE':
            case 'JUMP_TO_ACTION':
              dispatch({ devSetModel: JSON.parse(message.state) });
              return;
          }
          console.warn(`Unsupported dev tools action`, message);
        }
      });
    },

    send: (newModel: Model, action: Action) => {
      model = newModel;
      devTools.send(action, newModel);
    },

    unsubscribe: () => {
      model = initValue;
      unsubscribeFn?.();
    }
  }
}

function wrapAction<Action>(action: Action): WithDevToolsAction<Action> {
  return { action };
}

interface WithDevToolsModel<Model, Action> {
  devTools: DevToolsSubscription<Model, Action> | null,
  model: Model,
}

interface WithDevToolsAction<Action> {
  devSetModel?: any,
  action?: Action,
}

interface DevToolsSubscription<Model, Action> {
  effect(dispatch: Dispatch<WithDevToolsAction<Action>>): void;

  send(model: Model, action: Action): void;

  unsubscribe(): void;
}
