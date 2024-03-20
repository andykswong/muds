/**
 * A 64bit bigint generation index with 32bit index and 32bit generation.
 * @packageDocumentation
 */

const COMPONENT_BITS = 32n;
const UNIT_GENERATION = 2n ** COMPONENT_BITS;

/** The maximum generation value. */
export const MAX_GENERATION = 1 << 32 - 1;

/** Creates a generational index ID from index and generation parts. */
export function create<T extends bigint>(index = 0, generation = 0): T {
  return ((BigInt(generation >>> 0) << COMPONENT_BITS) + BigInt(index >>> 0)) as T;
}

/** Returns the index part (lower 32bit) of a generational index ID. */
export function indexOf(id: bigint): number {
  return Number(id % UNIT_GENERATION);
}

/** Returns the generation part (upper 32bit) of a generational index ID. */
export function generationOf(id: bigint): number {
  return Number(id >> COMPONENT_BITS) >>> 0;
}

/** Returns the same index ID at next generation. */
export function next<T extends bigint>(id: T): T {
  return (((id >> COMPONENT_BITS) + 1n) << COMPONENT_BITS) + (id % UNIT_GENERATION) as T;
}

/** Returns the next value for the given generation. */
export function nextGeneration(generation: number): number {
  return (generation + 1) >>> 0;
}
