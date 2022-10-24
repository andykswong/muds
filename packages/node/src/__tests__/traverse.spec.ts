import { preorderTraverse } from '../traverse';
import { getNodeTree } from './test-utils';

describe('preorderTraverse', () => {
  it('should iterate through all nodes in preorder', () => {
    const nodes = getNodeTree();

    let i = 1;
    for (const [nodeId, node] of preorderTraverse(nodes, 1)) {
      expect(nodeId).toBe(i);
      expect(node).toBe(nodes.get(nodeId));
      ++i;
    }
  });
});
