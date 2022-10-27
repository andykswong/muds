import React from 'react';
import { create, act, ReactTestRenderer } from 'react-test-renderer';
import { Event } from '@muds/event';
import { useEventReducer } from '../event';
import { ContainerProvider, useInject } from '../ioc';
import { Container, module, provide, singleton } from '@muds/ioc';


describe('integ', () => {
  const COUNTS = Symbol('counts'), INC_COUNT = Symbol('incCount');

  @module() class Module {
    @provide(COUNTS) @singleton counts() {
      return [10, 20];
    }

    @provide(INC_COUNT) @singleton onIncCount() {
      return Event.create<[number]>();
    }
  }

  function reducer(counts: number[], index: number) {
    counts[index]++;
    return [...counts];
  }

  function Counters() {
    const event = useInject(INC_COUNT) as Event<[number]>;
    const counts = useEventReducer(event, reducer, useInject(COUNTS) as number[]);
    return <span>{counts.map((count, i) => <button key={i} onClick={() => event.emit(i)}>{count}</button>)}</span>;
  }

  let container: Container;

  beforeEach(() => {
    container = Container.create();
    container.add(new Module());
  });

  test('integrate IoC container and events with React component', () => {
    let component!: ReactTestRenderer;

    act(() => {
      component = create(
        <ContainerProvider value={container}>
          <Counters />
        </ContainerProvider>
      );
    });

    expect(component.toJSON()).toMatchSnapshot();

    for (let i = 0; i < 2; ++i) {
      act(() => {
        const buttons = component.root.findAllByType('button');
        buttons[i].props.onClick();
      });
      expect(component.toJSON()).toMatchSnapshot();
    }

    act(() => component.unmount());
  });
});
