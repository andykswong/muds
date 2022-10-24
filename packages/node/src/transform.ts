import { mat4, Mat4, quat, Quat, ReadonlyMat4, ReadonlyQuat, ReadonlyVec3, transform, vec3, Vec3 } from 'munum';
import { UniqueIndexGet, UniqueIndexGetSet } from './types';
import { Node } from './node';

const Z3: ReadonlyVec3 = vec3.create();
const S3: ReadonlyVec3 = vec3.create(1, 1, 1);
const Q: ReadonlyQuat = quat.create();
const I4: ReadonlyMat4 = mat4.create();

/**
 * Defines a local space transformation.
 * TRS properties are postmultiplied in T * R * S order to compose the transformation matrix.
 */
export class Transform {
  /** A floating-point 4x4 local transformation matrix stored in column-major order. */
  public matrix: Mat4 | null = null;

  /** The node's translation along the x, y, and z axes. */
  public translation: Vec3 | null = null;

  /** The node's unit quaternion rotation. */
  public rotation: Quat | null = null;

  /** The node's scale, given as the scaling factors along the x, y, and z axes. */
  public scale: Vec3 | null = null;

  /** Set to true to indicate that the local transform matrix is dirty and needs to be updated from TRS. */
  public needUpdate = false;

  /**
   * Updates the local transformation matrix from TRS properties.
   * @param immediate Defaults to false, which only sets the needUpdate flag and defers the matrix calculation
   */
  public update(immediate = false): void {
    if (!(this.needUpdate = !immediate)) {
      this.matrix = transform(
        this.translation || Z3,
        this.rotation || Q,
        this.scale || S3,
        this.matrix || mat4.create()
      );
    }
  }
}

/** Updates the world transformations of a node hierarchy. */
export function updateWorldTransforms(
  nodes: UniqueIndexGet<number, Node>,
  transforms: UniqueIndexGet<number, Transform>,
  worldTransforms: UniqueIndexGetSet<number, Mat4>,
  rootId: number,
  rootTransform: ReadonlyMat4 = I4,
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
    world = mat4.create();
    forceUpdate = true; // need to update world if not exist
  }

  if (forceUpdate) {
    if (transform?.matrix) {
      mat4.mul(rootTransform, transform.matrix, world);
    } else {
      world = mat4.copy(rootTransform, world);
    }
    worldTransforms.set(rootId, world);
  }

  for (
    let childId = node?.lastChild || 0, child = nodes.get(childId);
    childId && child;
    childId = child.prevSibling, child = nodes.get(childId)
  ) {
    updateWorldTransforms(nodes, transforms, worldTransforms, childId, world, forceUpdate);
  }
}
