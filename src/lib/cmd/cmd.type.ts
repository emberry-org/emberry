/* Types of actions */
export type VoidAction = () => void;
export type StringAction = (s: string) => void;

/** Type of commands */
export enum CmdType {
  Debug = 'debug',
  Control = 'control',
}