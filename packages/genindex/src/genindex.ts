/** Generational index factory. */
export interface GenIndex<T = number, I = number, G = number> {
    /** The maximum generation value. */
    readonly MAX_GENERATION: G;

    /** Creates a generational index ID from index and generation parts. */
    create(index?: I, generation?: G): T;

    /** Returns the same index ID at next generation. */
    next(id: T): T;

    /** Returns the index part of a generational index ID. */
    indexOf(id: T): I;

    /** Returns the generation part of a generational index ID. */
    generationOf(id: T): G;

    /** Returns the next value for the given generation. */
    nextGeneration(generation: G): G;
}
