import { MODULE, TAG_ID, TAG_MULTI, TAG_SINGLETON } from './symbols';
import type { BindingMetadata, CustomTags, Identifier, ProviderMetadata, Tags } from './types';

/** Inversion of control container. */
export interface Container {
  /** Adds modules to the container. */
  add(...modules: unknown[]): void;

  /** Gets one instance from container. */
  get<T>(id: Identifier, tags?: CustomTags): T | undefined;

  /** Gets all matching instances from container. */
  getAll<T>(id: Identifier, tags?: CustomTags): T[];
}

/** Inversion of control container. */
export class Container implements Container {
  /** Creates a default container. */
  public static create(): Container {
    return new SimpleContainer();
  }
}

/** Simple map-based IoC container implementation. */
export class SimpleContainer implements Container {
  private bindings: Map<Identifier, BindingMetadata[]> = new Map();
  private cache: Map<Identifier, unknown[]> = new Map();

  public add(...modules: unknown[]): void {
    for (const module of modules) {
      const providers: ProviderMetadata[] | undefined = Reflect.getMetadata(MODULE, Object.getPrototypeOf(module));
      if (!providers) {
        continue; // Not a module
      }

      for (const provider of providers) {
        const id = provider.tags[TAG_ID] as Identifier;
        const bindings = this.bindings.get(id) || [];
        insertInOrder(bindings, {
          module: module as Record<string | symbol, (...args: unknown[]) => unknown>,
          ...provider,
        });
        this.bindings.set(id, bindings);
      }
    }
  }

  public get<T>(id: Identifier, tags?: CustomTags): T | undefined {
    const bindings = this.bindings.get(id);
    if (bindings) {
      for (let index = 0; index < bindings.length; ++index) {
        // TODO: log warning if there are multiple matches
        if (match(bindings[index], tags)) {
          return this.getBinding(id, index);
        }
      }
    }
    return undefined;
  }

  public getAll<T>(id: Identifier, tags?: CustomTags): T[] {
    const results: T[] = [];
    const bindings = this.bindings.get(id);
    if (bindings) {
      for (let index = 0; index < bindings.length; ++index) {
        if (match(bindings[index], tags)) {
          results.push(this.getBinding(id, index));
        }
      }
    }
    return results;
  }

  /** Gets instance of specific binding from container. */
  private getBinding<T>(id: Identifier, index: number): T {
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    const binding = this.bindings.get(id)![index]!;

    if (binding.tags[TAG_SINGLETON]) {
      const cached = this.cache.get(id)?.[index];
      if (cached !== undefined) {
        return cached as T;
      }
    }

    const args = binding.parameters.map((parameterTags) => {
      if (parameterTags[TAG_MULTI]) {
        return this.getAll(parameterTags[TAG_ID] as Identifier, parameterTags);
      }
      return this.get(parameterTags[TAG_ID] as Identifier, parameterTags);
    });
    const result = binding.module[binding.name](...args) as T;

    if (binding.tags[TAG_SINGLETON]) {
      const caches = this.cache.get(id) || [];
      caches[index] = result;
      this.cache.set(id, caches);
    }

    return result;
  }
}

function match(binding: BindingMetadata, tags?: Tags): boolean {
  if (!tags) {
    return true;
  }
  for (const tag in tags) {
    if (tags[tag] !== binding.tags[tag]) {
      return false;
    }
  }
  return true;
}

function insertInOrder(bindings: BindingMetadata[], binding: BindingMetadata) {
  bindings.push(binding);
  let i: number;
  for (i = bindings.length - 2; i >= 0 && (bindings[i].order || 0) > (binding.order || 0); --i) {
    bindings[i + 1] = bindings[i];
  }
  bindings[i + 1] = binding;
}
