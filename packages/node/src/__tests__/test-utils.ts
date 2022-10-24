import { mat3, mat4, ReadonlyMat3, ReadonlyMat4, vec2, vec3 } from 'munum';
import { Node, setParent } from '../node';
import { Transform } from '../transform';
import { Transform2D } from '../transform2d';

export const I3: ReadonlyMat3 = mat3.create();
export const I4: ReadonlyMat4 = mat4.create();
const PI_OVER_6 = Math.PI / 6;
const COS_PI_OVER_6 = Math.sqrt(3) / 2;
const SIN_PI_OVER_6 = 0.5;
const ONE_OVER_SQRT3 = 1 / Math.sqrt(3);

export function expectNode(
  nodes: Map<number, Node>, nodeId: number,
  parent: number, lastChild: number, prevSibling: number, nextSibling: number,
) {
  expect(nodes.get(nodeId)).toEqual({ parent, lastChild, prevSibling, nextSibling });
}

export function getNodeTree() {
  const nodes = new Map<number, Node>();
  nodes.set(1, new Node());
  nodes.set(2, new Node());
  nodes.set(3, new Node());
  nodes.set(4, new Node());
  nodes.set(5, new Node());
  nodes.set(6, new Node());
  nodes.set(7, new Node());

  setParent(nodes, 7, 6);
  setParent(nodes, 5, 2);
  setParent(nodes, 4, 2);
  setParent(nodes, 3, 2);
  setParent(nodes, 6, 1);
  setParent(nodes, 2, 1);

  return nodes;
}

export function getTransform() {
  const transform = new Transform();
  transform.translation = vec3.create(1, 2, 3);
  transform.rotation = [ONE_OVER_SQRT3 * SIN_PI_OVER_6, ONE_OVER_SQRT3 * SIN_PI_OVER_6, ONE_OVER_SQRT3 * SIN_PI_OVER_6, COS_PI_OVER_6];
  transform.scale = vec3.create(3, 6, 9);
  transform.matrix = [2, 2, -1, 0, -2, 4, 4, 0, 6, -3, 6, 0, 1, 2, 3, 1];

  return transform;
}

export function getTransform2D() {
  const transform = new Transform2D();
  transform.translation = vec2.create(11, 13);
  transform.rotation = PI_OVER_6;
  transform.scale = vec2.create(5, 7);
  transform.matrix = [5 * COS_PI_OVER_6, 5 * SIN_PI_OVER_6, 0, -7 * SIN_PI_OVER_6, 7 * COS_PI_OVER_6, 0, 11, 13, 1];

  return transform;
}
