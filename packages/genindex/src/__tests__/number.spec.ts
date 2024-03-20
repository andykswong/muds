import { GenIndex } from '../genindex.ts';
import * as IndexF64 from '../number.ts';

type BrandId = number & { __brand: 'brand' };

describe('IndexF64', () => {
  it('should fit the GenIndex interface', () => {
    const _: GenIndex = IndexF64;
    const _branded: GenIndex<BrandId> = IndexF64;
  });

  describe('create', () => {
    it('should create correct id from index and generation values', () => {
      expect(IndexF64.create(0, 0)).toBe(0);
      expect(IndexF64.create(7, 3)).toBe(12884901895);
    });
  });

  describe('indexOf', () => {
    it('should get the correct index from valid generational id', () => {
      expect(IndexF64.indexOf(12884901899)).toBe(11);
    });
  });

  describe('generationOf', () => {
    it('should get the correct generation from valid generational id', () => {
      expect(IndexF64.generationOf(21474836493)).toBe(5);
    });
  });

  describe('next', () => {
    it('should return the index ID at next generation', () => {
      expect(IndexF64.next(12884901899)).toBe(17179869195);
    });
  });

  describe('nextGeneration', () => {
    it('should return the next generation value', () => {
      expect(IndexF64.nextGeneration(5)).toBe(6);
    });
  });
});
