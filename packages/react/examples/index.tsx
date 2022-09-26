import { createRoot } from 'react-dom/client';

import { execute } from '@muds/runtime';
import { ReactSystem } from '../';

import { Counters } from './counters';

const container = document.createElement('div');
document.body.appendChild(container);

const root = createRoot(container);
const system = ReactSystem(Counters);

execute(system, {
  root,
  props: [0, 0, 0]
});
