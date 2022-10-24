import { jest } from '@jest/globals';
import { mat, mat4, Mat4, ReadonlyMat4 } from 'munum';
import { Node } from '../node';
import { Transform, updateWorldTransforms } from '../transform';
import { getNodeTree, getTransform, I4 } from './test-utils';

describe('Transform', () => {
  describe('update', () => {
    it('should update matrix from TRS', () => {
      const transform = getTransform();
      const expected = transform.matrix as ReadonlyMat4;
      transform.matrix = null;

      transform.update(true);
      expect(transform.matrix && mat.fequal(transform.matrix, expected)).toBeTruthy();
    });

    it('should set needUpdate only for non-immediate update', () => {
      const transform = new Transform();
      transform.update();
      expect(transform.needUpdate).toBeTruthy();
      expect(transform.matrix).toBeNull();
    });

    it('should defaults to identity matrix', () => {
      const transform = new Transform();
      transform.update(true);
      expect(transform.matrix && mat.fequal(transform.matrix, I4)).toBeTruthy();
    });

    it('should reset needUpdate to false', () => {
      const transform = new Transform();
      transform.needUpdate = true;
      transform.update(true);
      expect(transform.needUpdate).toBeFalsy();
    });
  });
});

describe('updateWorldTransforms', () => {
  let nodes: Map<number, Node>;
  let transforms: Map<number, Transform>;
  let worldTransforms: Map<number, Mat4>;

  beforeEach(() => {
    nodes = getNodeTree();
    transforms = new Map();
    worldTransforms = new Map();
  });

  it('should traverse all nodes', () => {
    const getNode = jest.spyOn(nodes, 'get');

    updateWorldTransforms(nodes, transforms, worldTransforms, 1);
    for (const i of nodes.keys()) {
      expect(getNode).toHaveBeenCalledWith(i);
      expect(worldTransforms.get(i)).toStrictEqual(I4);
    }
  });

  it('should update world transforms from parent world transform', () => {
    const transform = new Transform();
    transform.matrix = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    transforms.set(2, transform);

    const nodeId = 4;
    const childTransform = new Transform();
    childTransform.matrix = [0, 3, 2, 1, 7, 6, 5, 4, 9, 3, 2, 2, 0, 3, 3, 1];
    transforms.set(nodeId, childTransform);

    const expectedWorld: ReadonlyMat4 = [46, 52, 58, 64, 134, 156, 178, 200, 68, 84, 100, 116, 55, 62, 69, 76];

    updateWorldTransforms(nodes, transforms, worldTransforms, 1);
    expect(worldTransforms.get(nodeId)).toStrictEqual(expectedWorld);
  
  });

  it('should update Transform with needUpdate = true', () => {
    const nodeId = 2;
    const transform = getTransform();
    const expectedMatrix = transform.matrix as ReadonlyMat4;

    transform.matrix = null;
    transform.needUpdate = true;
    transforms.set(nodeId, transform);

    updateWorldTransforms(nodes, transforms, worldTransforms, 1);

    expect(transform.needUpdate).toBeFalsy();
    expect(transform.matrix && mat.fequal(transform.matrix, expectedMatrix)).toBeTruthy();
  });

  it('should skip nodes that do not require an update', () => {
    const lastNodeWithWorld = 4;
    for (let i = 1; i <= lastNodeWithWorld; ++i) {
      worldTransforms.set(i, mat4.create());
    }

    const setWorld = jest.spyOn(worldTransforms, 'set');

    updateWorldTransforms(nodes, transforms, worldTransforms, 1);
    for (const i of nodes.keys()) {
      if (i <= lastNodeWithWorld) {
        expect(setWorld).not.toHaveBeenCalledWith(i, expect.anything());
      } else {
        expect(setWorld).toHaveBeenCalledWith(i, expect.anything());
      }
    }
  });
});
