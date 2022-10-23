import { create, indexOf, generationOf } from '../id';

describe('GenId', () => {
  it('should create correct id from index and generation values', () => {
    expect(create(0, 0)).toBe(0);
    expect(create(7, 3)).toBe(12884901895);
  });

  it('should get the correct index from valid generational id', () => {
    expect(indexOf(12884901899)).toBe(11);
  });

  it('should get the correct generation from valid generational id', () => {
    expect(generationOf(21474836493)).toBe(5);
  });
});
