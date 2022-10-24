import { UniqueIndexGet } from './types';
import { Node } from './node';

/** Traverse a the node hierarchy in right-to-left preorder. */
export function* preorderTraverse(
  nodes: UniqueIndexGet<number, Node>, rootId: number
): IterableIterator<[number, Node]> {
  let currentId = rootId;
  while (currentId) {
    const current = nodes.get(currentId);
    if (!current) {
      break;
    }

    yield [currentId, current];
    currentId = nextNode(nodes, rootId, currentId, current);
  }
}

/** find next node; try to traverse children first, else backtrack and find a sibling */
function nextNode(nodes: UniqueIndexGet<number, Node>, rootId: number, currentId: number, current: Node): number {
  if (current.lastChild) {
    return current.lastChild;
  }

  while (currentId && currentId !== rootId) {
    if (current.prevSibling) {
      return current.prevSibling;
    }

    let parent: Node | undefined;
    currentId = current.parent;
    if (currentId && (parent = nodes.get(currentId))) {
      current = parent;
    } else {
      currentId = 0; // invalid node parent (should never happen)
    }
  }

  return 0;
}
