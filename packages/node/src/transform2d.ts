import { mat3, Mat3, ReadonlyMat3, ReadonlyVec2, transform2d, vec2, Vec2 } from 'munum';
import { UniqueIndexGet, UniqueIndexGetSet } from './types';
import { Node } from './node';

const Z2: ReadonlyVec2 = vec2.create();
const S2: ReadonlyVec2 = vec2.create(1, 1);
const I3: ReadonlyMat3 = mat3.create();

/**
 * Defines a local space 2D transformation.
 * TRS properties are postmultiplied in T * R * S order to compose the transformation matrix.
 */
export class Transform2D {
  /** A floating-point 3x3 local transformation matrix stored in column-major order. */
  public matrix: Mat3 | null = null;

  /** The node's translation along the x and y axes. */
  public translation: Vec2 | null = null;

  /** The node's rotation in radian. */
  public rotation = 0;

  /** The node's scale, given as the scaling factors along the x and y axes. */
  public scale: Vec2 | null = null;

  /** Set to true to indicate that the local transform matrix is dirty and needs to be updated from TRS. */
  public needUpdate = false;

  /**
   * Updates the local transformation matrix from TRS properties.
   * @param immediate Defaults to false, which only sets the needUpdate flag and defers the matrix calculation
   */
  public update(immediate = false): void {
    if (!(this.needUpdate = !immediate)) {
      this.matrix = transform2d(
        this.translation || Z2,
        this.rotation,
        this.scale || S2,
        this.matrix || mat3.create()
      );
    }
  }
}

/** Updates the 2D world transformations of a node hierarchy. */
export function updateWorldTransforms2D(
  nodes: UniqueIndexGet<number, Node>,
  transforms: UniqueIndexGet<number, Transform2D>,
  worldTransforms: UniqueIndexGetSet<number, Mat3>,
  rootId: number,
  rootTransform: ReadonlyMat3 = I3,
  forceUpdate = false,
) {
  const node = nodes.get(rootId);
  const transform = transforms.get(rootId);

  if (transform?.needUpdate) {
    transform.update(true);
    forceUpdate = true; // need to update world if local transform is updated
  }

  let world = worldTransforms.get(rootId);
  if (!world) {
    world = mat3.create();
    forceUpdate = true; // need to update world if not exist
  }

  if (forceUpdate) {
    if (transform?.matrix) {
      mat3.mul(rootTransform, transform.matrix, world);
    } else {
      world = mat3.copy(rootTransform, world);
    }
    worldTransforms.set(rootId, world);
  }

  for (
    let childId = node?.lastChild || 0, child = nodes.get(childId);
    childId && child;
    childId = child.prevSibling, child = nodes.get(childId)
  ) {
    updateWorldTransforms2D(nodes, transforms, worldTransforms, childId, world, forceUpdate);
  }
}
