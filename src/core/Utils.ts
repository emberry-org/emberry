export function insertString(base: string, val: string, i: number): string {
  return base.slice(0, i) + val + base.slice(i);
}