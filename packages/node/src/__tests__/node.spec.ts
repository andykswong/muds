import { Node, setParent } from '../node';
import { expectNode } from './test-utils';

const ROOT1_ID = 1;
const ROOT2_ID = 11;
const CHILD1_ID = 2;
const CHILD2_ID = 3;

describe('Node', () => {
  describe('setParent', () => {
    it('should attach node to new parent', () => {
      const nodes = new Map<number, Node>();
      nodes.set(ROOT1_ID, new Node());
      nodes.set(CHILD1_ID, new Node());
      nodes.set(CHILD2_ID, new Node());

      expect(setParent(nodes, CHILD1_ID, ROOT1_ID)).toBeTruthy();
      expectNode(nodes, ROOT1_ID, 0, CHILD1_ID, 0, 0);
      expectNode(nodes, CHILD1_ID, ROOT1_ID, 0, 0, 0);

      expect(setParent(nodes, CHILD2_ID, ROOT1_ID)).toBeTruthy();
      expectNode(nodes, ROOT1_ID, 0, CHILD2_ID, 0, 0);
      expectNode(nodes, CHILD1_ID, ROOT1_ID, 0, 0, CHILD2_ID);
      expectNode(nodes, CHILD2_ID, ROOT1_ID, 0, CHILD1_ID, 0);
    });

    it('should detach existing node parent', () => {
      const nodes = new Map<number, Node>();
      nodes.set(ROOT1_ID, new Node());
      nodes.set(CHILD1_ID, new Node());
      nodes.set(CHILD2_ID, new Node());

      expect(setParent(nodes, CHILD2_ID, ROOT1_ID)).toBeTruthy();
      expect(setParent(nodes, CHILD1_ID, ROOT1_ID)).toBeTruthy();
 
      // Detach last child
      expect(setParent(nodes, CHILD1_ID)).toBeTruthy();
      expectNode(nodes, ROOT1_ID, 0, CHILD2_ID, 0, 0);
      expectNode(nodes, CHILD1_ID, 0, 0, 0, 0);
      expectNode(nodes, CHILD2_ID, ROOT1_ID, 0, 0, 0);

      // Detach non-last child
      expect(setParent(nodes, CHILD1_ID, ROOT1_ID)).toBeTruthy();
      expect(setParent(nodes, CHILD2_ID)).toBeTruthy();
      expectNode(nodes, ROOT1_ID, 0, CHILD1_ID, 0, 0);
      expectNode(nodes, CHILD1_ID, ROOT1_ID, 0, 0, 0);
      expectNode(nodes, CHILD2_ID, 0, 0, 0, 0);
    });

    it('should detach from existing node parent before attaching to new parent', () => {
      const nodes = new Map<number, Node>();
      nodes.set(ROOT1_ID, new Node());
      nodes.set(ROOT2_ID, new Node());
      nodes.set(CHILD1_ID, new Node());

      expect(setParent(nodes, CHILD1_ID, ROOT1_ID)).toBeTruthy();
      expect(setParent(nodes, CHILD1_ID, ROOT2_ID)).toBeTruthy();

      expectNode(nodes, ROOT1_ID, 0, 0, 0, 0);
      expectNode(nodes, ROOT2_ID, 0, CHILD1_ID, 0, 0);
      expectNode(nodes, CHILD1_ID, ROOT2_ID, 0, 0, 0);
    });

    it('should ignore invalid node updates', () => {
      const nodes = new Map<number, Node>();
      nodes.set(ROOT1_ID, new Node());
      nodes.set(CHILD1_ID, new Node());

      expect(setParent(nodes, -1, ROOT1_ID)).toBeFalsy();
      expectNode(nodes, ROOT1_ID, 0, 0, 0, 0);

      expect(setParent(nodes, CHILD1_ID, -1)).toBeFalsy();
      expectNode(nodes, CHILD1_ID, 0, 0, 0, 0);
    });
  });
});
