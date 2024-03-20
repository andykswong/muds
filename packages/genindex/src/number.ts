/**
 * Generational index number which consists of 32bit index and 21bit generation values.
 * @packageDocumentation
 */

const UNIT_GENERATION = 2 ** 32;

/** The maximum generation value. */
export const MAX_GENERATION = (1 << 21) - 1;

/** Creates a generational index ID from index and generation parts. */
export function create<T extends number>(index = 0, generation = 0): T {
  return ((generation & MAX_GENERATION) * UNIT_GENERATION + (index >>> 0)) as T;
}

/** Returns the index part (lower 32bit) of a generational index ID. */
export function indexOf(id: number): number {
  return id >>> 0;
}

/** Returns the generation part (upper 21bit) of a generational index ID. */
export function generationOf(id: number): number {
  return (id / UNIT_GENERATION) & MAX_GENERATION;
}

/** Returns the same index ID at next generation. */
export function next<T extends number>(id: T): T {
  return (((generationOf(id) + 1) & MAX_GENERATION) * UNIT_GENERATION + (id >>> 0)) as T;
}

/** Returns the next value for the given generation. */
export function nextGeneration(generation: number): number {
  return (generation + 1) & MAX_GENERATION;
}
