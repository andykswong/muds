# @muds/react
<p align="center">
  <a href="https://www.npmjs.com/package/@muds/react"><img src="https://img.shields.io/npm/v/@muds/react?label=@muds/react" alt="NPM @muds/react" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`@muds/react` is the [React](https://reactjs.org/) integrataion utils for [muds](https://github.com/andykswong/muds), the modular microframework for interactive data-oriented systems. It provides useful hooks for integrating your React components with `muds` libraries.

## API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save @muds/react @muds/event @muds/ioc
```

Use `useEventReducer(event, reducer, initialState)` hook to act on events within a React component. It is comparable to `useReducer` hook, but for muds events.
```javascript
import React from 'react';
import { Event } from '@muds/event';
import { useEventReducer } from '@muds/react';

function Counter({ event, initialCount = 0 }) {
  const count = useEventReducer(event, reducer, initialCount);
  return (<span>{count}<button onClick={() => event.emit('inc')}>+1</button></span>);
}

function reducer(count: number, action: Action): number {
  switch (action) {
    case 'inc': return count + 1;
    default: return count;
  }
}

const event = Event.create();
const vdom = <Counter event={event} initialCount={100} />;
event.emit('inc'); // increments the count outside of the component
```

Use `useInject(id, tags?)` to inject dependencies from IoC container, and pair with events to trigger updates and rerendering:
```javascript
import React from 'react';
import { Event } from '@muds/event';
import { ContainerProvider, useInject, useEventReducer } from '@muds/react';
import { Container, module, provide, singleton } from '@muds/ioc';

const COUNTS = Symbol('counts'), INC_COUNT = Symbol('incCount');

@module() class Module {
  @provide(COUNTS) @singleton counts() {
    return [10, 20];
  }

  @provide(INC_COUNT) @singleton onIncCount() {
    return Event.create();
  }
}

function reducer(counts, index) {
  counts[index]++;
  return [...counts]; // returns a copy to trigger rerender
}

function Counters() {
  const event = useInject(INC_COUNT);
  const counts = useEventReducer(event, reducer, useInject(COUNTS));
  return <span>{counts.map((count, i) => <button key={i} onClick={() => event.emit(i)}>{count}</button>)}</span>;
}

const component = (<ContainerProvider value={Container.create()}><Counters /></ContainerProvider>);
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
