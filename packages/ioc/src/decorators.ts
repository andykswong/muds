import { MODULE, PROVIDER, TAG_ID, TAG_MULTI, TAG_SINGLETON } from './symbols';
import type { Identifier, ProviderMetadata, Tags } from './types';

type Target = Record<string, unknown>;

/** Decorates a class as a container module. */
export function module(): ClassDecorator {
  // eslint-disable-next-line @typescript-eslint/ban-types
  return <T extends Function>(ctor: T) => {
    Reflect.defineMetadata(MODULE, Reflect.getOwnMetadata(MODULE, ctor.prototype) || [], ctor.prototype);
  };
}

/** Decorates a class method as a provider binding. */
export function provide(id: Identifier): MethodDecorator {
  return (target: unknown, propertyKey: string | symbol) => {
    const providers: ProviderMetadata[] = Reflect.getOwnMetadata(MODULE, target as Target) || [];
    const metadata: ProviderMetadata = {
      name: propertyKey,
      tags: {},
      parameters: [],
      ...Reflect.getOwnMetadata(PROVIDER, target as Target, propertyKey)
    };
    metadata.tags[TAG_ID] = id;
    providers.push(metadata);

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
export function inject(id: Identifier): ParameterDecorator {
  return tagged({ [TAG_ID]: id });
}

/** Decorates a method parameter to multi-inject a binding. */
export const multi: ParameterDecorator = tagged({ [TAG_MULTI]: true });
