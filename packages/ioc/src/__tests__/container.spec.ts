import { jest } from '@jest/globals';
import { Container, SimpleContainer } from '../container';
import { inject, module, multi, order, provide, singleton, tagged } from '../decorators';
import { TAG_ID, TAG_SINGLETON } from '../symbols';

describe('Container', () => {
  const id0 = Symbol('id0'), id1 = Symbol('id1'), id2 = Symbol('id2'), id3 = Symbol('id3'), id4 = Symbol('id4');

  @module() class Module {
    @provide(id0) ten() { return 10; }
    @provide(id1) @singleton tens(@inject(id0) ten: number) { return [ten, ten]; }
  }

  @module() class MultiModule {
    @provide(id0) @tagged({ id: 2 }) @order(2) second() { return 2; }
    @provide(id0) @tagged({ id: 1 }) @singleton first() { return 1; }
    @provide(id2) @singleton third() { return 3; }
    @provide(id3) injectSecond(@inject(id0) @tagged({ id: 2 }) second: number) { return second; }
    @provide(id1) nums(@inject(id0) @multi num: number[], @inject(id2) third: number) {
      return [...num, third].join();
    }
    @provide(id4) unordered1() { return 1; }
    @provide(id4) unordered2() { return 2; }
  }

  describe('add', () => {
    it('should add module providers to container', () => {
      const container = Container.create() as SimpleContainer;
      const m = new Module();
      container.add(m);

      expect(container['bindings'].size).toBe(2);
      expect(container['bindings'].get(id0))
        .toEqual([{ module: m, name: 'ten', tags: { [TAG_ID]: id0 }, parameters: [] }]);
      expect(container['bindings'].get(id1))
        .toEqual([
          { module: m, name: 'tens', tags: { [TAG_ID]: id1, [TAG_SINGLETON]: true }, parameters: [{ [TAG_ID]: id0 }] }
        ]);
    });

    it('should do nothing when non-module is added', () => {
      class Module { }
      const container = Container.create();
      container.add(new Module());
    });
  });

  describe('get', () => {
    it('should return result from matching provider', () => {
      const container = Container.create();
      container.add(new Module());

      expect(container.get(id1)).toEqual([10, 10]);
    });

    it('should return result from provider with matching tags', () => {
      const container = Container.create();
      container.add(new MultiModule());

      expect(container.get(id0, { id: 1 })).toEqual(1);
    });

    it('should cache singleton values', () => {
      const container = Container.create();
      const m = new Module();
      const providerSpy = jest.spyOn(m, 'ten');
      container.add(m);

      const firstCall = container.get(id1);
      const secondCall = container.get(id1);

      expect(firstCall).toEqual([10, 10]);
      expect(secondCall).toBe(firstCall);
      expect(providerSpy).toHaveBeenCalledTimes(1);
    });

    it('should support multi-injecting parameter into provider', () => {
      const container = Container.create();
      container.add(new MultiModule());

      expect(container.get(id1)).toEqual('1,2,3');
    });

    it('should support injecting tagged parameter into provider', () => {
      const container = Container.create();
      container.add(new MultiModule());

      expect(container.get(id3)).toEqual(2);
    });

    it('should return undefined for unknown ids', () => {
      const id = Symbol('id');
      const container = Container.create();
      expect(container.get(id)).toBeUndefined();
    });
  });

  describe('multiGet', () => {
    it('should return ordered array of results from matching providers', () => {
      const container = Container.create();
      container.add(new MultiModule());

      expect(container.multiGet(id0)).toEqual([1, 2]);
    });

    it('should return unordered results from matching providers sorted by declaration order', () => {
      const container = Container.create();
      container.add(new MultiModule());

      expect(container.multiGet(id4)).toEqual([1, 2]);
    });

    it('should return result from provider with matching tags', () => {
      const container = Container.create();
      container.add(new MultiModule());

      expect(container.multiGet(id0, { id: 1 })).toEqual([1]);
    });

    it('should return empty array for unknown ids', () => {
      const id = Symbol('id');
      const container = Container.create();
      expect(container.multiGet(id)).toEqual([]);
    });
  });
});
