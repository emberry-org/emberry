/**
 * Merge two objects of the same type.
 * @param a Object that will be overwritten.
 * @param b Object that will overwrite.
 * @returns The merged object where `b` masks `a`.
 */
export function merge<T>(a: T, b: T): T {
  return {
    ...a,
    ...b,
  }
}
