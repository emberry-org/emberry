export default interface Cmd {
  title: string;
  type: CmdType;
  /** Also called a shortcut */
  accelerator: string[];
  /** The action to be executed */
  action: String | VoidAction;
}

type VoidAction = () => void;

// type StringAction = () => string;

export enum CmdType {
  Debug = 'debug',
  Control = 'control',
}