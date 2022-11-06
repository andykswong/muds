import { MODULE, PROVIDER, TAG_ID, TAG_MULTI, TAG_SINGLETON } from './symbols';
import type { Identifier, ProviderMetadata, Tags } from './types';

type Target = Record<string, unknown>;

/** Decorates a class as a container module. */
export function module(): ClassDecorator {
  // eslint-disable-next-line @typescript-eslint/ban-types
  return <T extends Function>(ctor: T) => {
    Reflect.defineMetadata(MODULE, Reflect.getMetadata(MODULE, ctor.prototype) || [], ctor.prototype);
  };
}

/** Decorates a class method as a provider binding. */
export function provide(id?: Identifier): MethodDecorator {
  return (target: unknown, propertyKey: string | symbol) => {
    const providers: ProviderMetadata[] = Reflect.getMetadata(MODULE, target as Target) || [];
    const metadata: ProviderMetadata = {
      name: propertyKey,
      tags: {},
      parameters: [],
      ...Reflect.getOwnMetadata(PROVIDER, target as Target, propertyKey)
    };
    // Defaults to use return type as ID tag
    // TODO: log warning if the tag is undefined
    metadata.tags[TAG_ID] = id ?? Reflect.getOwnMetadata('design:returntype', target as Target, propertyKey);

    // Overrides provider of same name from base class
    const overrideIndex = providers.findIndex(provider => provider.name === metadata.name);
    if (overrideIndex < 0) {
      providers.push(metadata);
    } else {
      providers[overrideIndex] = {
        ...providers[overrideIndex],
        ...metadata,
        tags: {
          ...providers[overrideIndex].tags,
          ...metadata.tags,
        },
      };
    }

    Reflect.defineMetadata(MODULE, providers, target as Target);
    Reflect.defineMetadata(PROVIDER, metadata, target as Target, propertyKey);
  };
}

/** Decorates a class method as a singleton provider. */
export function singleton(target: unknown, propertyKey: string | symbol) {
  const metadata: ProviderMetadata = {
    name: propertyKey,
    tags: {},
    parameters: [],
    ...Reflect.getOwnMetadata(PROVIDER, target as Target, propertyKey),
  };
  metadata.tags[TAG_SINGLETON] = true;

  Reflect.defineMetadata(PROVIDER, metadata, target as Target, propertyKey);
}

/** Decorates a class method with binding order. A smaller value has higher priority. */
export function order(order: number): MethodDecorator {
  return (target: unknown, propertyKey: string | symbol) => {
    const metadata: ProviderMetadata = {
      name: propertyKey,
      tags: {},
      parameters: [],
      ...Reflect.getOwnMetadata(PROVIDER, target as Target, propertyKey)
    };
    metadata.order = order;

    Reflect.defineMetadata(PROVIDER, metadata, target as Target, propertyKey);
  };
}

/** Decorates a method or parameter with tags. */
export function tagged(tags: Tags): MethodDecorator & ParameterDecorator {
  return (target: unknown, propertyKey: string | symbol, indexOrDescriptor: number | PropertyDescriptor) => {
    const metadata: ProviderMetadata = {
      name: propertyKey,
      tags: {},
      parameters: [],
      ...Reflect.getOwnMetadata(PROVIDER, target as Target, propertyKey),
    };
    if (isNaN(indexOrDescriptor as number)) { // MethodDecorator
      metadata.tags = { ...tags, ...metadata.tags };
    } else { // ParameterDecorator
      metadata.parameters[indexOrDescriptor as number] = {
        ...metadata.parameters[indexOrDescriptor as number],
        ...tags
      };
    }

    Reflect.defineMetadata(PROVIDER, metadata, target as Target, propertyKey);
  };
}

/** Decorates a method parameter to inject a binding. */
export function inject(id?: Identifier): ParameterDecorator {
  return (target: unknown, propertyKey: string | symbol, parameterIndex: number) => {
    // Defaults to use parameter type as ID tag
    const tag = id ?? Reflect.getOwnMetadata('design:paramtypes', target as Target, propertyKey)?.[parameterIndex];
    if (tag !== undefined) {
      tagged({ [TAG_ID]: tag })(target as Target, propertyKey, parameterIndex);
    }
    // TODO: log warning if the tag is undefined
  };
}

/** Decorates a method parameter to multi-inject a binding. */
export const multi: ParameterDecorator = tagged({ [TAG_MULTI]: true });
