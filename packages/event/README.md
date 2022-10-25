# @muds/event
<p align="center">
  <a href="https://www.npmjs.com/package/@muds/event"><img src="https://img.shields.io/npm/v/@muds/event?label=@muds/event" alt="NPM @muds/event" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT" /></a>
</p>

## Overview
`@muds/event` is the core event signal library for [muds](https://github.com/andykswong/muds), the modular microframework for interactive data-oriented systems. It provides the event emitter and listener interface, as well as several useful event implementations.

## API Documentation
http://andykswong.github.io/muds

## Usage
```shell
npm install --save @muds/event
```

```javascript
import { Event, EventMultiplexer } from '@muds/event';

const listener = (...msg) => console.log(...msg);
const event1 = Event.create(); // simple event
const event2 = Event.create();

event1.addListener(listener);
event1.emit('hello', 'world'); // log "hello world"
event1.removeListener(listener);

const multiplexer = new EventMultiplexer(); // multiplex events into a single event
multiplexer.addListener(listener);
multiplexer.add(event1, (event) => [1, ...event]); // add an event with event mapping
multiplexer.add(event2, (event) => [2, ...event]);
event1.emit('hello', 'again'); // log "1 hello again"
event2.emit('bye'); // log "2 bye"
```

```javascript
import { OnAnimationFrame } from '@muds/event';

const listener = (delta) => console.log(`delta: ${delta} ms`);
const event = new OnAnimationFrame(); // emit event with delta time on each requestAnimationFrame callback
event.addListener(listener);
event.start(); // log "delta: 16ms" periodically
// event.pause(); to stop
```

## License
This repository and the code inside it is licensed under the MIT License. Read [LICENSE](./LICENSE) for more information.
