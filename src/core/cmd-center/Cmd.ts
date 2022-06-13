export default interface Cmd {
  title: string;
  type: CmdType;
  /** Also called a shortcut */
  accelerator: string[];
  /** The action to be executed */
  action: String | VoidAction | StringAction;
  /** Whether this actions takes input */
  input: boolean;
  /** Description of what this input is for */
  input_desc?: string;
}

export type VoidAction = () => void;

export type StringAction = (s: string) => void;

export enum CmdType {
  Debug = 'debug',
  Control = 'control',
}