import { GenIndex } from '../genindex.ts';
import * as IndexU64 from '../bigint.ts';

type BrandId = bigint & { __brand: 'brand' };

describe('IndexU64', () => {
  it('should fit the GenIndex interface', () => {
    const _: GenIndex<bigint> = IndexU64;
    const _branded: GenIndex<BrandId> = IndexU64;
  });

  describe('create', () => {
    it('should create correct id from index and generation values', () => {
      expect(IndexU64.create(0, 0)).toBe(0n);
      expect(IndexU64.create(7, 3)).toBe(12884901895n);
    });
  });

  describe('indexOf', () => {
    it('should get the correct index from valid generational id', () => {
      expect(IndexU64.indexOf(12884901899n)).toBe(11);
    });
  });

  describe('generationOf', () => {
    it('should get the correct generation from valid generational id', () => {
      expect(IndexU64.generationOf(21474836487n)).toBe(5);
    });
  });

  describe('next', () => {
    it('should return the index ID at next generation', () => {
      expect(IndexU64.next(12884901895n)).toBe(17179869191n);
    });
  });

  describe('nextGeneration', () => {
    it('should return the next generation value', () => {
      expect(IndexU64.nextGeneration(5)).toBe(6);
    });
  });
});
