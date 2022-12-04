import { createContext, useContext } from 'react';
import { Container, CustomTags, Identifier } from '@muds/ioc';

const ContainerContext = createContext(Container.create());

/** Provider of IoC container. */
export const ContainerProvider = ContainerContext.Provider;

/** Returns the current IoC container. */
export function useContainer(): Container {
  return useContext(ContainerContext);
}

/** Injects a value from IoC container. */
export function useInject<T>(id: Identifier, tags?: CustomTags): T | undefined {
  return useContainer().get(id, tags);
}

/** Injects values from IoC container. */
export function useMultiInject<T>(id: Identifier, tags?: CustomTags): T[] {
  return useContainer().getAll(id, tags);
}
