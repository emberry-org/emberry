export function insertString(base: string, val: string, i: number): string {
  return base.slice(0, i) + val + base.slice(i);
}

export function removeRange(base: string, range: [number, number]): string {
  if (range[0] >= 0) {
    return base.slice(0, range[0]) + base.slice(range[1]);
  }
  return base;
}