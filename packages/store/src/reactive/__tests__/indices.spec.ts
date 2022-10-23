import { UniqueIndex } from '../indices';
import { mockCollectionEvents } from './mocks';

describe('UniqueIndex', () => {

  describe('fromCollection', () => {
    it('should index items on collection add event', () => {
      const collection = mockCollectionEvents<number, [number, string]>(false);
      const index = UniqueIndex.fromCollection(collection, (_, [k]) => k);

      expect(index.size).toBe(0);

      const K1 = 1, UK1 = 21, K2 = 2, UK2 = 22;
      collection.onAdd.emit(collection, K1, [UK1, 'hello']);
      expect(index.size).toBe(1);
      expect(index.get(UK1)).toBe(K1);

      collection.onAdd.emit(collection, K2, [UK2, 'world']);
      expect(index.size).toBe(2);
      expect(index.get(UK2)).toBe(K2);
    });

    it('should remove items on collection delete event', () => {
      const collection = mockCollectionEvents<number, [number, string]>(false);
      const index = UniqueIndex.fromCollection(collection, (_, [k]) => k);

      const K1 = 1, UK1 = 21, K2 = 2, UK2 = 22;
      collection.onAdd.emit(collection, K1, [UK1, 'hello']);
      collection.onAdd.emit(collection, K2, [UK2, 'world']);
      expect(index.size).toBe(2);

      collection.onDelete.emit(collection, K1, [UK1, 'hello']);
      expect(index.size).toBe(1);
      expect(index.has(UK1)).toBeFalsy();
      expect(index.get(UK2)).toBe(K2);
    });

    it('should empty the index on collection clear', () => {
      const collection = mockCollectionEvents<number, [number, string]>(false);
      const index = UniqueIndex.fromCollection(collection, (_, [k]) => k);

      const K1 = 1, UK1 = 21, K2 = 2, UK2 = 22;
      collection.onAdd.emit(collection, K1, [UK1, 'hello']);
      collection.onAdd.emit(collection, K2, [UK2, 'world']);
      expect(index.size).toBe(2);

      collection.onClear.emit(collection);
      expect(index.size).toBe(0);
    });

    it('should reindex item on collection update event', () => {
      const collection = mockCollectionEvents<number, [number, string]>(true);
      const index = UniqueIndex.fromCollection(collection, (_, [k]) => k);

      const K1 = 1, UK1 = 21, K2 = 2, UK2 = 22, UK3 = 23;
      collection.onAdd.emit(collection, K1, [UK1, 'hello']);
      collection.onAdd.emit(collection, K2, [UK2, 'world']);
      expect(index.size).toBe(2);

      collection.onUpdate?.emit(collection, K2, [UK3, 'new'], [UK2, 'world']);
      expect(index.size).toBe(2);
      expect(index.has(UK1)).toBeTruthy();
      expect(index.has(UK2)).toBeFalsy();
      expect(index.get(UK3)).toBe(K2);
    });
  });

});
