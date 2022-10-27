import React from 'react';
import { create, act } from 'react-test-renderer';
import { ContainerProvider, useContainer, useInject, useMultiInject } from '../ioc';
import { Container, module, provide, tagged } from '@muds/ioc';

const ID = Symbol('id');
const TAG = { tag: 't' };

describe('useContainer', () => {
  it('should return provided container', () => {
    const container = Container.create();
    let actualContainer: Container | null = null;

    function Component() {
      actualContainer = useContainer();
      return null;
    }

    act(() => {
      create(
        <ContainerProvider value={container}>
          <Component />
        </ContainerProvider>
      );
    });

    expect(actualContainer).toBe(container);
  });
});

describe('useInject', () => {
  it('should return injectable from container', () => {
    const expected = 'INJECTABLE';
    const container = Container.create();

    @module() class Module {
      @provide(ID) @tagged(TAG) injectable() { return expected; }
    }
    container.add(new Module());

    let result: string | undefined = undefined;

    function Component() {
      result = useInject(ID, TAG);
      return null;
    }

    act(() => {
      create(
        <ContainerProvider value={container}>
          <Component />
        </ContainerProvider>
      );
    });

    expect(result).toBe(expected);
  });
});

describe('useMultiInject', () => {
  it('should return injectables from container', () => {
    const expected = 'INJECTABLE';
    const expected2 = 'INJECTABLE2';
    const container = Container.create();

    @module() class Module {
      @provide(ID) @tagged(TAG) injectable() { return expected; }
      @provide(ID) @tagged(TAG) injectable2() { return expected2; }
    }
    container.add(new Module());

    let result: string[] = [];

    function Component() {
      result = useMultiInject(ID, TAG);
      return null;
    }

    act(() => {
      create(
        <ContainerProvider value={container}>
          <Component />
        </ContainerProvider>
      );
    });

    expect(result).toEqual([expected, expected2]);
  });
});
