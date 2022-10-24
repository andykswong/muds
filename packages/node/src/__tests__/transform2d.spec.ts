import { jest } from '@jest/globals';
import { mat, mat3, Mat3, ReadonlyMat3 } from 'munum';
import { Node } from '../node';
import { Transform2D, updateWorldTransforms2D } from '../transform2d';
import { getNodeTree, getTransform2D, I3 } from './test-utils';

describe('Transform2D', () => {
  describe('update', () => {
    it('should update matrix from TRS', () => {
      const transform = getTransform2D();
      const expected = transform.matrix as ReadonlyMat3;
      transform.matrix = null;

      transform.update(true);
      expect(transform.matrix && mat.fequal(transform.matrix, expected)).toBeTruthy();
    });

    it('should set needUpdate only for non-immediate update', () => {
      const transform = new Transform2D();
      transform.update();
      expect(transform.needUpdate).toBeTruthy();
      expect(transform.matrix).toBeNull();
    });

    it('should defaults to identity matrix', () => {
      const transform = new Transform2D();
      transform.update(true);
      expect(transform.matrix && mat.fequal(transform.matrix, I3)).toBeTruthy();
    });

    it('should reset needUpdate to false', () => {
      const transform = new Transform2D();
      transform.needUpdate = true;
      transform.update(true);
      expect(transform.needUpdate).toBeFalsy();
    });
  });
});

describe('updateWorldTransforms2D', () => {
  let nodes: Map<number, Node>;
  let transforms: Map<number, Transform2D>;
  let worldTransforms: Map<number, Mat3>;

  beforeEach(() => {
    nodes = getNodeTree();
    transforms = new Map();
    worldTransforms = new Map();
  });

  it('should traverse all nodes', () => {
    const getNode = jest.spyOn(nodes, 'get');

    updateWorldTransforms2D(nodes, transforms, worldTransforms, 1);
    for (const i of nodes.keys()) {
      expect(getNode).toHaveBeenCalledWith(i);
      expect(worldTransforms.get(i)).toStrictEqual(I3);
    }
  });

  it('should update world transforms from parent world transform', () => {
    const transform = new Transform2D();
    transform.matrix = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    transforms.set(2, transform);

    const nodeId = 4;
    const childTransform = new Transform2D();
    childTransform.matrix = [0, 9, 7, 2, 1, 6, 3, 1, 8];
    transforms.set(nodeId, childTransform);

    const expectedWorld: ReadonlyMat3 = [85, 101, 117, 48, 57, 66, 63, 75, 87];

    updateWorldTransforms2D(nodes, transforms, worldTransforms, 1);
    expect(worldTransforms.get(nodeId)).toStrictEqual(expectedWorld);
  
  });

  it('should update Transform with needUpdate = true', () => {
    const nodeId = 2;
    const transform = getTransform2D();
    const expectedMatrix = transform.matrix as ReadonlyMat3;

    transform.matrix = null;
    transform.needUpdate = true;
    transforms.set(nodeId, transform);

    updateWorldTransforms2D(nodes, transforms, worldTransforms, 1);

    expect(transform.needUpdate).toBeFalsy();
    expect(transform.matrix && mat.fequal(transform.matrix, expectedMatrix)).toBeTruthy();
  });

  it('should skip nodes that do not require an update', () => {
    const lastNodeWithWorld = 4;
    for (let i = 1; i <= lastNodeWithWorld; ++i) {
      worldTransforms.set(i, mat3.create());
    }

    const setWorld = jest.spyOn(worldTransforms, 'set');

    updateWorldTransforms2D(nodes, transforms, worldTransforms, 1);
    for (const i of nodes.keys()) {
      if (i <= lastNodeWithWorld) {
        expect(setWorld).not.toHaveBeenCalledWith(i, expect.anything());
      } else {
        expect(setWorld).toHaveBeenCalledWith(i, expect.anything());
      }
    }
  });
});
