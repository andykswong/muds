import { UniqueIndexGet } from './types';

/** A node in the node hierarchy. */
export class Node {
  /** Parent node ID. */
  public parent = 0;

  /** Last child node ID. */
  public lastChild = 0;

  /** Previous sibling node ID. */
  public prevSibling = 0;

  /** Next sibling node ID. */
  public nextSibling = 0;
}

/** Sets the parent of a node, returns if node is updated. */
export function setParent(nodes: UniqueIndexGet<number, Node>, nodeId: number, parentId = 0): boolean {
  const node = nodes.get(nodeId);
  const parent = nodes.get(parentId);

  // Skip invalid node or parent
  if (!node || (parentId && !parent)) {
    return false;
  }

  // Remove from original parent
  if (node.parent) {
    const origParent = nodes.get(node.parent);
    if (origParent?.lastChild == nodeId) {
      origParent.lastChild = node.prevSibling;
    }
    if (node.prevSibling) {
      const prevSibling = nodes.get(node.prevSibling);
      prevSibling && (prevSibling.nextSibling = node.nextSibling);
    }
    if (node.nextSibling) {
      const nextSibling = nodes.get(node.nextSibling);
      nextSibling && (nextSibling.prevSibling = node.prevSibling);
    }
    node.parent = node.prevSibling = node.nextSibling = 0;
  }

  // Add to last child of new parent
  if (parent) {
    if ((node.prevSibling = parent.lastChild)) {
      const prevSibling = nodes.get(node.prevSibling);
      prevSibling && (prevSibling.nextSibling = nodeId);
    }
    parent.lastChild = nodeId;
    node.parent = parentId;
  }

  return true;
}
