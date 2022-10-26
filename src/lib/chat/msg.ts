export interface Message {
  /** Type of the message */
  type: string;

  sender: string;
  content: string;
  time: string;

  /** Is the message before this one from the same user? */
  chain: boolean;
}