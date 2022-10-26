import { inject, module, multi, provide, singleton, tagged } from '../decorators';
import { MODULE, PROVIDER, TAG_ID, TAG_MULTI, TAG_SINGLETON } from '../symbols';

describe('module', () => {
  it('should set metadata', () => {
    @module() class Module {}
    expect(Reflect.getOwnMetadata(MODULE, Module.prototype)).toEqual([]);
  });
});

describe('provide', () => {
  it('should set id tag metadata', () => {
    const id = Symbol('test');
    const providerName = 'getTest';

    @module() class Module {
      @provide(id) getTest() { return 'hello'; }
    }

    const expectedMetadata = { name: providerName, tags: { [TAG_ID]: id }, parameters: [] };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
    expect(Reflect.getOwnMetadata(MODULE, Module.prototype)).toEqual([expectedMetadata]);
  });
});

describe('singleton', () => {
  it('should set singleton tag metadata', () => {
    const providerName = 'getTest';

    @module() class Module {
      @singleton getTest() { return 'hello'; }
    }

    const expectedMetadata = { name: providerName, tags: { [TAG_SINGLETON]: true }, parameters: [] };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
  });

  it('should work with @provide', () => {
    const id = Symbol('test');
    const providerName = 'getTest';

    @module() class Module {
      @provide(id) @singleton getTest() { return 'hello'; }
    }

    const expectedMetadata = { name: providerName, tags: { [TAG_ID]: id, [TAG_SINGLETON]: true }, parameters: [] };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
    expect(Reflect.getOwnMetadata(MODULE, Module.prototype)).toEqual([expectedMetadata]);
  });
});

describe('tagged', () => {
  it('should set method tag metadata', () => {
    const tags = { tag: 1 };
    const providerName = 'getTest';

    @module() class Module {
      @tagged(tags) getTest() { return 1; }
    }

    const expectedMetadata = { name: providerName, tags, parameters: [] };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
  });

  it('should set parameter id tag metadata for @inject', () => {
    const id0 = Symbol('arg0');
    const id1 = Symbol('arg1');
    const providerName = 'getTest';

    @module() class Module {
      getTest(@inject(id0) arg0: string, @inject(id1) arg1: string) { return arg0 + arg1; }
    }

    const expectedMetadata = {
      name: providerName, tags: {},
      parameters: [{ [TAG_ID]: id0 }, { [TAG_ID]: id1 }],
    };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
  });

  it('should set parameter multi tag metadata for @multi', () => {
    const providerName = 'getTest';

    @module() class Module {
      getTest(@multi arg0: string) { return arg0; }
    }

    const expectedMetadata = {
      name: providerName, tags: {},
      parameters: [{ [TAG_MULTI]: true }],
    };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
  });

  it('should work with @provide and multiple tags', () => {
    const id = Symbol('test');
    const argId = Symbol('arg0');
    const providerName = 'getTest';
    const tags = { tag: 1 };

    @module() class Module {
      @provide(id) @tagged(tags) getTest(@multi @inject(argId) arg0: string) { return arg0; }
    }

    const expectedMetadata = {
      name: providerName, tags: { [TAG_ID]: id, ...tags },
      parameters: [{ [TAG_ID]: argId, [TAG_MULTI]: true }],
    };
    expect(Reflect.getOwnMetadata(PROVIDER, Module.prototype, providerName)).toEqual(expectedMetadata);
  });

});
