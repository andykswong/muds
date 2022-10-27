/// <reference types="reflect-metadata" />

/** Identifier type for objects. */
export type Identifier = string | symbol;

/** Tags to identify a target type. */
export type Tags = Record<Identifier, unknown>;

/** Custom tags to identify a target type. */
export type CustomTags = Record<string, unknown>;

/** Metadata for a provider. */
export interface ProviderMetadata {
  /** Provider method name. */
  name: string | symbol,

  /** Provider tags. */
  tags: Tags,

  /** Provider dependency parameter tags. */
  parameters: Tags[],

  /** Provider priority order. */
  order?: number,
}

/** Metadatam of an identifier binding. */
export interface BindingMetadata extends ProviderMetadata {
  /** Module to bind to. */
  module: Record<string | symbol, (...args: unknown[]) => unknown>;
}
