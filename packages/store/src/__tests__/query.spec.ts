import { uniqueJoin } from '../query';

describe('uniqueJoin', () => {
  it('should perform left join by key', () => {
    const lhs = new Map<number, string>();
    const rhs1 = new Map<string, number>();
    const rhs2 = new Map<string, boolean>();
    const key = ([k, v]: [number, string]) => k === 3 ? undefined : v;

    lhs.set(1, 'a');
    lhs.set(2, 'b');
    lhs.set(3, 'c');
    rhs1.set('a', 11);
    rhs2.set('a', true);
    rhs2.set('b', false);

    const results = [];
    for (const result of uniqueJoin(lhs.entries(), key, rhs1, rhs2)) {
      results.push(result);
    }

    expect(results).toStrictEqual([
      [1, 'a', 11, true],
      [2, 'b', undefined, false],
      [3, 'c', undefined, undefined],
    ]);
  })
});
