<h1 align="center">〰ᗰᑌᗪᔕ〰</h1>
<h2 align="center">muds - Micro-framework for interactive data-driven systems in JavaScript</h2>
<br />
<p align="center">
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a> 
  <a href="https://www.npmjs.com/package/@muds/runtime"><img src="https://img.shields.io/npm/v/@muds/runtime?label=@muds/runtime" alt="NPM @muds/runtime" /></a> 
  <a href="https://www.npmjs.com/package/@muds/react"><img src="https://img.shields.io/npm/v/@muds/runtime?label=@muds/react" alt="NPM @muds/react" /></a> 
</p>

## Overview
`muds` is a micro-framework for structuring interactive data-driven systems, like web apps and games, that is maintainable, reusable, scalable and testable. It is heavily inspired by the [Elm Architecture](https://guide.elm-lang.org/architecture/) and [Redux](https://redux.js.org/).

## Getting Started
Install via NPM for JS/TS projects: 

```shell
npm install --save @muds/runtime
```

### Simple System Example

Below is a simple system that gets user's name from a prompt:

```javascript
import { execute } from '@muds/runtime';

const system = {
  create: () => [{ name: '' }],

  update: (model, action) => {
    if (action?.type === 'SET_NAME') {
      return [{ name: action.name }];
    }
    return [model];
  },

  destroy: (model) => {},

  view: (model, dispatch) => {
    if (!model.name) {
      const name = prompt(`Hello, may I know your name?`);
      if (name) {
        dispatch({ type: 'SET_NAME', name });
      }
    } else {
      alert(`Welcome, ${model.name}!`);
    }
  },
};

const runtime = execute(system);
runtime.destroy();
```

### React Integration

`@muds/react` provides a high-order system for React view rendering. Below is a simple counter example:

```javascript
import { createRoot } from 'react-dom/client';
import { execute } from '@muds/runtime';
import { ReactSystem } from '@muds/react';

const container = document.createElement('div');
const root = createRoot(container);
document.body.appendChild(container);

const CounterSystem = {
  create: (count) => [count],

  update: (count, action) {
    switch (action) {
      case 'INC': return [count + 1];
      case 'DEC': return [count - 1];
      default: return [count];
    }
  },

  view: (count, dispatch) => (
    <span>
      <button onClick={() => dispatch('DEC')}>-</button>
      {` ${count} `}
      <button onClick={() => dispatch('INC')}>+</button>
    </span>
  ),
};

const system = ReactSystem(CounterSystem);

execute(system, { root, props: 0 });

```

### Redux DevTools Integration

`muds` can be easily integrated with [Redux DevTools](https://github.com/reduxjs/redux-devtools) for debugging state changes. You just need to wrap your system with `WithDevTools`:

```javascript
import { execute, WithDevTools } from '@muds/runtime';

const system = WithDevTools(YourSystem);
execute(system, YourSystemProps);
```


## API Usage
With `muds`, you write stateless system with 4 APIs that define its model and view:

Model APIs:
- ```create(Props): [Model, Effect?]```: a function that takes a props object (input parameters) and constructs a new model with optional side-effect
- ```update(Model, Action): [Model, Effect?]```: a function that applies a given action to current model, and returns a new model with optional side-effect
- ```destroy(Model): void```: a function that cleans up a model when destroyed

View API:
- ```view(Model, dispatch): View```: a function that takes a model and a dispatch function to render an interactive view. `muds` is frame-agnostic and does not know how to render a view. A top-level system must perform the rendering through side-effects

Where:
- `Props`, `Model` and `Action` can be any object defined by the system
- ```dispatch: (Action) => void``` is a function to dispatch actions to the system's `update` function, same as [Redux's dispatch](https://redux.js.org/api/store#dispatchaction)
- ```Effect: (dispatch) => void``` is a function that contains side effects and possibly dispatch actions.

At top level, you use the `muds` runtime to execute a system:
- ```execute(system, props)```

Systems are just stateless functions with clear model/view separation. They can be easily composed or nested by calling other systems to form a complex application. It is also easy to swap a view while reusing the model (e.g. running headless for debugging), or vice versa.

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
